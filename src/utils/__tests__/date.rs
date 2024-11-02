#[cfg(test)]
mod tests {
    use crate::utils::date::get_year_range;

    use chrono::{Datelike, NaiveDate, Weekday};

    #[test]
    fn test_get_year_range_normal_year() {
        let (start, end) = get_year_range(2024).unwrap();

        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2025, 1, 4).unwrap());

        assert_eq!(start.weekday(), Weekday::Sun);
        assert_eq!(end.weekday(), Weekday::Sat);
    }

    #[test]
    fn test_get_year_range_leap_year() {
        let (start, end) = get_year_range(2024).unwrap();

        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(2025, 1, 4).unwrap());

        assert_eq!(start.weekday(), Weekday::Sun);
        assert_eq!(end.weekday(), Weekday::Sat);
    }

    #[test]
    fn test_get_year_range_min_year() {
        assert!(get_year_range(i32::MIN).is_none());
    }

    #[test]
    fn test_get_year_range_max_year() {
        assert!(get_year_range(i32::MAX).is_none());
    }

    #[test]
    fn test_get_year_range_out_of_range_low() {
        assert!(get_year_range(-262144).is_none());
    }

    #[test]
    fn test_get_year_range_out_of_range_high() {
        assert!(get_year_range(262144).is_none());
    }

    #[test]
    fn test_get_year_range_year_0() {
        let (start, end) = get_year_range(0).unwrap();

        assert_eq!(start, NaiveDate::from_ymd_opt(-1, 12, 26).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(1, 1, 6).unwrap());

        assert_eq!(start.weekday(), Weekday::Sun);
        assert_eq!(end.weekday(), Weekday::Sat);
    }

    #[test]
    fn test_get_year_range_consistency() {
        for year in -1000..=1000 {
            let (start, end) = get_year_range(year).unwrap();

            assert_eq!(start.weekday(), Weekday::Sun);
            assert!(end.weekday() == Weekday::Sat || end.weekday() == Weekday::Sun);
            assert!(start <= NaiveDate::from_ymd_opt(year, 1, 1).unwrap());
            assert!(end >= NaiveDate::from_ymd_opt(year, 12, 31).unwrap());
        }
    }
}
