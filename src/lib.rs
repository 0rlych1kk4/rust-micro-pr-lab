/// Render a nonnegative number of cents as an amount with grouping separators.
///
/// Example: 123456789 cents becomes "1,234,567.89".
pub fn format_cents(cents: u64) -> String {
    let dollars = (cents / 100).to_string();
    let mut grouped = String::with_capacity(dollars.len() + dollars.len() / 3);
    for (i, ch) in dollars.chars().enumerate() {
        if i != 0 && (dollars.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(ch);
    }
    format!("{}.{:02}", grouped, cents % 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundary_examples() {
        assert_eq!(format_cents(0), "0.00");
        assert_eq!(format_cents(1), "0.01");
        assert_eq!(format_cents(100), "1.00");
        assert_eq!(format_cents(123_456_789), "1,234,567.89");
        assert_eq!(format_cents(u64::MAX), "184,467,440,737,095,516.15");
    }
}
