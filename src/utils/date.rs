use chrono::{Datelike, Duration, NaiveDate, Utc};

use crate::constants::date::{WEEK_TO_DAY, YEAR_TO_DAYS};

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

pub fn calculate_day_passed_in_year(year: i32) -> f32 {
    let now = Utc::now().naive_utc().date();

    if year < now.year() {
        return YEAR_TO_DAYS;
    }

    let start_date = NaiveDate::from_ymd_opt(year, 1, 1);
    let end_date = NaiveDate::from_ymd_opt(year, now.month(), now.day());

    match (start_date, end_date) {
        (Some(start), Some(end)) => (end - start).num_days() as f32,
        _ => YEAR_TO_DAYS,
    }
}
