use std::collections::HashMap;

use crate::{api::contributions::get_daily_commits, utils::date::get_year_range};
use chrono::{Datelike, Duration, NaiveDate};

const CELL_SIZE: u32 = 10;
const CELL_SPACING: u32 = 4;
const GRID_LEFT_PADDING: u32 = 13;
const GRID_TOP_PADDING: u32 = 13;
const WEEK_TO_DAY: usize = 7;

pub async fn index_service(user_name: String, year: i32) -> String {
    let commits = get_daily_commits(&user_name, year).await.unwrap();
    let (start_date, end_date) = get_year_range(year).unwrap();
    let weeks = calculate_weeks(start_date, end_date);

    let cells = generate_contribution_cells(year, start_date, weeks, commits);

    generate_svg(cells)
}

fn calculate_weeks(start_date: NaiveDate, end_date: NaiveDate) -> usize {
    ((end_date - start_date).num_days() / WEEK_TO_DAY as i64 + 1) as usize
}

fn generate_contribution_cells(
    year: i32,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
) -> String {
    let mut cells = String::new();

    for week in 0..weeks {
        for day in 0..WEEK_TO_DAY {
            let current_date = start_date + Duration::days((week * WEEK_TO_DAY + day) as i64);

            if current_date.year() != year {
                continue;
            }

            let formatted_date = current_date.format("%Y-%m-%d").to_string();

            let x = week as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING;
            let y = day as u32 * (CELL_SIZE + CELL_SPACING) + GRID_TOP_PADDING;

            let commit_level = commits.get(&formatted_date).unwrap_or(&0);
            let color = get_cell_color(commit_level);

            cells.push_str(&format!(
                "  <rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" fill=\"{}\" title=\"{}\" data-level=\"{}\" />\n",
                CELL_SIZE, CELL_SIZE, x, y, color, formatted_date, commit_level
            ));
        }
    }

    cells
}

fn get_cell_color(commit_level: &u32) -> &'static str {
    match commit_level {
        1 => "#c6e48b",
        2 => "#7bc96f",
        3 => "#239a3b",
        4 => "#196127",
        _ => "#ebedf0",
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
