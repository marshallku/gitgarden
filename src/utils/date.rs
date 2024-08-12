use chrono::{Datelike, Duration, NaiveDate};

pub fn get_year_range(year: i32) -> (NaiveDate, NaiveDate) {
    let first_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    let start_date = first_day - Duration::days(first_day.weekday().num_days_from_sunday() as i64);
    let end_date =
        last_day + Duration::days((6 - last_day.weekday().num_days_from_sunday()) as i64);
    (start_date, end_date)
}
