use chrono::{Datelike, NaiveDate, Weekday};

use crate::api::contributions::get_daily_commits;

pub async fn index_service(user_name: String, year: i32) -> String {
    let response = get_daily_commits(&user_name, year).await.unwrap();
    let first_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

    // Find the first Sunday before or on January 1st
    let start_date =
        first_day - chrono::Duration::days(first_day.weekday().num_days_from_sunday() as i64);
    // Find the last Saturday after or on December 31st
    let end_date =
        last_day + chrono::Duration::days((6 - last_day.weekday().num_days_from_sunday()) as i64);

    let weeks = ((end_date - start_date).num_days() / 7 + 1) as usize;

    let mut current_date =
        first_day - chrono::Duration::days(first_day.weekday().num_days_from_sunday() as i64);
    let mut cells = String::new();

    for week in 0..weeks {
        for day in 0..7 {
            let x = week * 14 + 13;
            let y = day * 13 + 13;

            let color = if current_date.year() == year {
                match current_date.weekday() {
                    Weekday::Sat | Weekday::Sun => "#ebedf0",
                    _ => "#9be9a8",
                }
            } else {
                "#ffffff"
            };

            cells.push_str(&format!(
                "  <rect width=\"10\" height=\"10\" x=\"{}\" y=\"{}\" fill=\"{}\" title=\"{}\" />\n",
                x, y, color, current_date.format("%Y-%m-%d")
            ));

            if current_date <= last_day {
                current_date = current_date.succ_opt().unwrap();
            }
        }
    }

    let svg = format!(
        r##"
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 930 465"
            fill="none"
        >
        {}
        </svg>
        "##,
        cells
    );

    println!("{:?}", response);

    svg.to_string()
}
