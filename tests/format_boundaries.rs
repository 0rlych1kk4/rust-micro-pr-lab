use rust_micro_pr_lab::format_cents;

#[test]
fn formatting_boundaries() {
    let cases = [
        (0, "0.00"),
        (1, "0.01"),
        (99, "0.99"),
        (100, "1.00"),
        (999, "9.99"),
        (100_000, "1,000.00"),
        (123_456_789, "1,234,567.89"),
    ];

    for (input, expected) in cases {
        assert_eq!(format_cents(input), expected);
    }
}
