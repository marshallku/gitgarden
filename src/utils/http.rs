use axum::http::HeaderMap;
use chrono::{Duration, Utc};

use crate::constants::date::{
    DAY_TO_SECONDS, HOUR_TO_SECONDS, MINUTE_TO_SECONDS, MONTH_TO_SECONDS, WEEK_TO_SECONDS,
    YEAR_TO_SECONDS,
};

pub fn get_cache_header(age: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let parsed_age = parse_age(&age).unwrap_or_default();

    let cache_age = if parsed_age == 0 {
        "no-cache".to_string()
    } else {
        format!("public, max-age={}", age)
    };

    let expires = if parsed_age <= 0 {
        "0".to_string()
    } else {
        let now = Utc::now();
        let expires = now + Duration::seconds(parsed_age as i64);
        expires.to_rfc2822()
    };

    headers.insert("Cache-Control", cache_age.parse().unwrap());
    headers.insert("Expires", expires.parse().unwrap());

    headers
}

pub fn parse_age(age: &str) -> Result<u64, &'static str> {
    if age.is_empty() {
        return Err("Empty input");
    }

    let (value, unit) = age.split_at(age.len() - 1);
    let value: u64 = value.parse().map_err(|_| "Invalid number")?;

    match unit {
        "s" => Ok(value),
        "m" => Ok(value * MINUTE_TO_SECONDS),
        "h" => Ok(value * HOUR_TO_SECONDS),
        "d" => Ok(value * DAY_TO_SECONDS),
        "w" => Ok(value * WEEK_TO_SECONDS),
        "M" => Ok(value * MONTH_TO_SECONDS),
        "y" => Ok(value * YEAR_TO_SECONDS),
        _ => Err("Invalid unit"),
    }
}
