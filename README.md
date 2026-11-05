# Rust Micro PR Lab

A **new, standalone** Rust repository combining an amount-formatting library, deterministic regression fixtures, and an opt-in PR automation CLI. It **does not modify your existing repositories**.


## Requirements

- macOS/Linux with Rust (`cargo`, `rustc`), `git`, GitHub CLI (`gh`)
- Logged in via `gh auth login`; `gh auth status` works
- A **new GitHub repository**, with `main` branch and `origin` remote
- Repository Settings → General → Pull Requests → **Allow auto-merge** enabled
- Repository ruleset/branch protection requiring the **validate** CI job before merging (otherwise auto-merge might merge immediately)
- Git identity matches your intended GitHub account; create PRs using your own `gh` authentication

## Initial setup on Mac

```bash
unzip rust-micro-pr-lab.zip
cd rust-micro-pr-lab
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release

gh auth status
# Create a NEW GitHub repo and push the starter project (do not use an existing repo):
git init -b main
git add .
git commit -m 'feat: initialize Rust micro PR lab'
gh repo create rust-micro-pr-lab --public --source=. --remote=origin --push
```

Configure ruleset requiring GitHub Actions status check `validate` after the CI workflow appears, and enable auto-merge. Verify protection before enabling unattended operation.

## Commands

```bash
cargo run -- plan
cargo run -- run --execute
cargo run -- run --execute --auto-merge
```

Each execution performs at most **one** real, tracked code/test fixture change and one PR. It refuses to continue if there is an open PR, if your local repo is dirty, if tests fail, or once all 905 fixtures exist. The PR is created via `gh pr create` under the identity authenticated locally.

After a successful merge, execute the same command again to create the next PR.

### Optional automatic macOS scheduling

The bundled `examples/com.example.micro-pr.plist` runs the Rust executable every 6 hours. Replace both `REPLACE_USERNAME` placeholders with your actual macOS username and adjust the repository path. Verify one manual run first, then:

```bash
cp examples/com.example.micro-pr.plist ~/Library/LaunchAgents/com.example.micro-pr.plist
launchctl bootstrap "gui/$(id -u)" ~/Library/LaunchAgents/com.example.micro-pr.plist
```

To stop scheduled operation:

```bash
launchctl bootout "gui/$(id -u)" ~/Library/LaunchAgents/com.example.micro-pr.plist
```
