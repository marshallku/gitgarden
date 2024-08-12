use std::collections::HashMap;

use crate::{
    api::contributions::get_daily_commits,
    utils::{date::get_year_range, encode::encode_from_path},
};
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

    generate_svg(year, start_date, weeks, commits)
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

            let field = encode_from_path("field/dirt2.png");

            cells.push_str(&format!(
                "<image width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
                CELL_SIZE, CELL_SIZE, x, y, field
            ));

            if *commit_level > 0 {
                let flower = encode_from_path(&format!("flowers/1-{}.png", commit_level));

                cells.push_str(&format!(
                    "<image width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
                    CELL_SIZE, CELL_SIZE, x, y, flower
                ));
            }
        }
    }

    cells
}

fn generate_svg(
    year: i32,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
) -> String {
    const WIDTH: u32 = 930;
    const HEIGHT: u32 = 465;

    let cells = generate_contribution_cells(year, start_date, weeks, commits);

    format!(
        r##"
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 {} {}"
            fill="none"
        >
        {}
        </svg>
        "##,
        WIDTH, HEIGHT, cells
    )
}
