use std::{env, error::Error, fs, path::Path, process::Command};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const TARGET: usize = 905;

fn run(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        return Err(format!(
            "{command} {args:?} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn expected_amount(cents: u64) -> String {
    // Independent fixture oracle: build groups from the right, instead of from the left.
    let mut whole = cents / 100;
    let mut groups = vec![format!("{:03}", whole % 1000)];
    whole /= 1000;
    while whole != 0 {
        groups.push(format!("{:03}", whole % 1000));
        whole /= 1000;
    }
    if let Some(last) = groups.last_mut() {
        *last = last.trim_start_matches('0').to_owned();
        if last.is_empty() {
            *last = "0".to_string();
        }
    }
    groups.reverse();
    format!("{}.{:02}", groups.join(","), cents % 100)
}

fn fixture_content(n: usize) -> String {
    // Stable, varied values including comma-grouped, decimal, and boundary-like cases.
    let value = (n as u64).pow(3) * 7919 + (n as u64 * 17) % 100;
    format!("{value}\n{}\n", expected_amount(value))
}

fn next_id() -> Option<usize> {
    (1..=TARGET).find(|n| !Path::new(&format!("tests/cases/{n:04}.txt")).exists())
}

fn verify_clean() -> Result<()> {
    if !run("git", &["status", "--porcelain"])?.is_empty() {
        return Err("Working tree isn't clean; commit/stash changes before running".into());
    }
    Ok(())
}

fn plan() -> Result<()> {
    let count = (1..=TARGET)
        .filter(|n| Path::new(&format!("tests/cases/{n:04}.txt")).exists())
        .count();
    println!("Fixtures present: {count}/{TARGET}");
    match next_id() {
        Some(n) => {
            let data = fixture_content(n);
            println!("Next possible change: tests/cases/{n:04}.txt\n{data}");
            println!("One PR per invocation; automatic merge needs required CI checks and repository auto-merge enabled.");
        }
        None => println!("All target fixtures exist; no changes are required."),
    }
    Ok(())
}

fn execute(auto_merge: bool) -> Result<()> {
    if !Path::new("Cargo.toml").exists() || !Path::new("tests/cases").exists() {
        return Err("Run from the rust-micro-pr-lab repository root".into());
    }
    verify_clean()?;
    run("gh", &["auth", "status"])?;
    // The --json query checks the repo for ANY open PR to prevent piling up submissions.
    let open = run(
        "gh",
        &[
            "pr", "list", "--state", "open", "--json", "number", "--jq", "length",
        ],
    )?;
    if open != "0" {
        return Err(
            format!("{open} open PR(s) found. Resolve them before creating another.").into(),
        );
    }
    run("git", &["switch", "main"])?;
    run("git", &["pull", "--ff-only", "origin", "main"])?;
    verify_clean()?;
    let n = match next_id() {
        Some(n) => n,
        None => {
            println!("All {TARGET} fixtures present.");
            return Ok(());
        }
    };
    let branch = format!("test/fixture-{n:04}");
    run("git", &["switch", "-c", &branch])?;
    let path = format!("tests/cases/{n:04}.txt");
    fs::write(&path, fixture_content(n))?;
    // Run tests before committing, so a failed test cannot create a remote PR.
    run("cargo", &["fmt", "--all", "--", "--check"])?;
    run(
        "cargo",
        &["clippy", "--all-targets", "--", "-D", "warnings"],
    )?;
    run("cargo", &["test", "--all"])?;
    run("git", &["add", "--", &path])?;
    let message = format!("test: add amount regression fixture {n:04}");
    run("git", &["commit", "-m", &message])?;
    run("git", &["push", "-u", "origin", &branch])?;
    let body = format!("Adds deterministic amount-formatting regression fixture {n:04}.\n\nValidated locally with cargo fmt, clippy, and test.\n\nThis is one fixture change, not an empty PR.");
    let url = run(
        "gh",
        &[
            "pr", "create", "--base", "main", "--head", &branch, "--title", &message, "--body",
            &body,
        ],
    )?;
    println!("Created PR: {url}");
    if auto_merge {
        run(
            "gh",
            &["pr", "merge", &url, "--auto", "--squash", "--delete-branch"],
        )?;
        println!("Requested auto-merge for {url}; verify required CI checks are configured.");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.as_slice() {
        [cmd] if cmd == "plan" => plan(),
        [cmd, flag] if cmd == "run" && flag == "--execute" => execute(false),
        [cmd, flag, auto] if cmd == "run" && flag == "--execute" && auto == "--auto-merge" => {
            execute(true)
        }
        _ => {
            eprintln!("Usage:\n  micro-pr plan\n  micro-pr run --execute [--auto-merge]\n\nTargets up to {TARGET} distinct regression fixtures, one PR at a time.");
            std::process::exit(2);
        }
    };
    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
