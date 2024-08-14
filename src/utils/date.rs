use chrono::{Datelike, Duration, NaiveDate};

pub const WEEK_TO_DAY: usize = 7;

pub fn get_year_range(year: i32) -> Option<(NaiveDate, NaiveDate)> {
    let first_day = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31)?;

    let start_date = first_day.checked_sub_signed(Duration::days(
        first_day.weekday().num_days_from_sunday() as i64,
    ))?;
    let end_date = last_day.checked_add_signed(Duration::days(
        (6 - last_day.weekday().num_days_from_sunday()) as i64,
    ))?;

    Some((start_date, end_date))
}

pub fn calculate_weeks(start_date: NaiveDate, end_date: NaiveDate) -> usize {
    ((end_date - start_date).num_days() / WEEK_TO_DAY as i64 + 1) as usize
}
