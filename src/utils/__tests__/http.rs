#[cfg(test)]
mod tests {
    use crate::{
        constants::date::{
            DAY_TO_SECONDS, HOUR_TO_SECONDS, MINUTE_TO_SECONDS, MONTH_TO_SECONDS, WEEK_TO_SECONDS,
            YEAR_TO_SECONDS,
        },
        utils::http::parse_age,
    };

    #[test]
    fn test_valid_inputs() {
        assert_eq!(parse_age("30s"), Ok(30));
        assert_eq!(parse_age("5m"), Ok(5 * MINUTE_TO_SECONDS));
        assert_eq!(parse_age("2h"), Ok(2 * HOUR_TO_SECONDS));
        assert_eq!(parse_age("1d"), Ok(1 * DAY_TO_SECONDS));
        assert_eq!(parse_age("1w"), Ok(1 * WEEK_TO_SECONDS));
        assert_eq!(parse_age("1M"), Ok(1 * MONTH_TO_SECONDS));
        assert_eq!(parse_age("1y"), Ok(1 * YEAR_TO_SECONDS));

        assert_eq!(parse_age("0s"), Ok(0));
        assert_eq!(parse_age("1s"), Ok(1));
        // Max value for u64
        assert_eq!(parse_age("18446744073709551615s"), Ok(18446744073709551615));
    }

    #[test]
    fn text_empty_input() {
        assert_eq!(parse_age(""), Err("Empty input"));
    }

    #[test]
    fn test_invalid_unit() {
        assert_eq!(parse_age("30x"), Err("Invalid unit"));
        assert_eq!(parse_age("30"), Err("Invalid unit"));
    }

    #[test]
    fn test_invalid_number() {
        assert_eq!(parse_age("abc"), Err("Invalid number"));
        assert_eq!(parse_age("-1s"), Err("Invalid number"));
        assert_eq!(parse_age("18446744073709551616s"), Err("Invalid number"));
    }

    #[test]
    fn test_unwrapped_value() {
        assert_eq!(parse_age("30s").unwrap(), 30);
        assert_eq!(parse_age("30x").unwrap_or_default(), 0);
    }
}
