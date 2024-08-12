use crate::api::contributions::get_daily_commits;
use chrono::{Datelike, Duration, NaiveDate, Weekday};

const CELL_SIZE: u32 = 10;
const CELL_SPACING: u32 = 4;
const GRID_LEFT_PADDING: u32 = 13;
const GRID_TOP_PADDING: u32 = 13;
const WEEK_TO_DAY: usize = 7;

pub async fn index_service(user_name: String, year: i32) -> String {
    let commits = get_daily_commits(&user_name, year).await.unwrap();
    let (start_date, end_date) = get_year_range(year);
    let weeks = calculate_weeks(start_date, end_date);

    let cells = generate_contribution_cells(start_date, weeks, year);

    generate_svg(cells)
}

fn get_year_range(year: i32) -> (NaiveDate, NaiveDate) {
    let first_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    let start_date = first_day - Duration::days(first_day.weekday().num_days_from_sunday() as i64);
    let end_date =
        last_day + Duration::days((6 - last_day.weekday().num_days_from_sunday()) as i64);
    (start_date, end_date)
}

fn calculate_weeks(start_date: NaiveDate, end_date: NaiveDate) -> usize {
    ((end_date - start_date).num_days() / WEEK_TO_DAY as i64 + 1) as usize
}

fn generate_contribution_cells(start_date: NaiveDate, weeks: usize, year: i32) -> String {
    let mut cells = String::new();

    for week in 0..weeks {
        for day in 0..WEEK_TO_DAY {
            let current_date = start_date + Duration::days((week * WEEK_TO_DAY + day) as i64);
            let x = week as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING;
            let y = day as u32 * (CELL_SIZE + CELL_SPACING) + GRID_TOP_PADDING;
            let color = get_cell_color(current_date, year);

            cells.push_str(&format!(
                "  <rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" fill=\"{}\" title=\"{}\" />\n",
                CELL_SIZE, CELL_SIZE, x, y, color, current_date.format("%Y-%m-%d")
            ));
        }
    }

    cells
}

fn get_cell_color(date: NaiveDate, year: i32) -> &'static str {
    if date.year() == year {
        match date.weekday() {
            Weekday::Sat | Weekday::Sun => "#ebedf0",
            _ => "#9be9a8",
        }
    } else {
        "#ffffff" // Color for days outside the year
    }
}

fn generate_svg(cells: String) -> String {
    format!(
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
    )
}
