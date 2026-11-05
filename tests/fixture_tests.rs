use rust_micro_pr_lab::format_cents;
use std::{fs, path::Path};

#[test]
fn regression_fixtures() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cases");
    for file in fs::read_dir(dir).expect("fixture directory") {
        let file = file.expect("fixture entry");
        if file.path().extension().and_then(|x| x.to_str()) != Some("txt") {
            continue;
        }
        let contents = fs::read_to_string(file.path()).expect("read fixture");
        let (input, expected) = contents
            .trim_end()
            .split_once('\n')
            .expect("two-line fixture");
        let input = input.parse::<u64>().expect("integer cents");
        assert_eq!(format_cents(input), expected, "fixture {:?}", file.path());
    }
}
