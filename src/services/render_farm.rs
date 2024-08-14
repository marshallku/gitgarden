use std::collections::HashMap;

use crate::{
    api::{contributions::get_daily_commits, stats::get_stats},
    env::state::AppState,
    utils::{
        coordinate::generate_coordinate,
        date::{calculate_weeks, get_year_range, WEEK_TO_DAY},
        encode::encode_from_path,
    },
};
use chrono::{Datelike, Duration, NaiveDate};

const CELL_SIZE: u32 = 16;
const CELL_SPACING: u32 = 4;
const GRID_LEFT_PADDING: u32 = 24;
const GRID_TOP_PADDING: u32 = 312;

pub async fn render_farm_service(user_name: String, year: i32, state: AppState) -> String {
    let commits = get_daily_commits(&user_name, year).await.unwrap();
    let (start_date, end_date) = get_year_range(year).unwrap();
    let weeks = calculate_weeks(start_date, end_date);

    generate_svg(user_name, year, state, start_date, weeks, commits).await
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

            let x = GRID_LEFT_PADDING + week as u32 * (CELL_SIZE + CELL_SPACING);
            let y = GRID_TOP_PADDING + day as u32 * (CELL_SIZE + CELL_SPACING);

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

fn generate_home(user_name: String) -> String {
    let (x, y) = generate_coordinate(user_name, (80.0, 730.0), (25.0, 70.0));
    let home = encode_from_path("objects/home.png");
    let road = encode_from_path("objects/stone_road.png");

    format!(
        "<image width=\"151\" height=\"155\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" /><image width=\"31\" height=\"89\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
        x, y, home, x + 67.0, y + 152.0, road
    )
}

fn generate_trees(user_name: String, width: u32, repositories_contributed_to: i32) -> String {
    let mut trees = String::new();
    let tree_count = repositories_contributed_to;
    let mut coords: Vec<(f64, f64)> = (0..tree_count)
        .map(|i| {
            generate_coordinate(
                &format!("{}-tree-{}", user_name, i),
                (5.0, width as f64 - 50.0),
                (5.0, 230.0),
            )
        })
        .collect();

    coords.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (x, y) in coords {
        let (tree, _) = generate_coordinate("{}-tree-kind-{}", (1.0, 2.0), (1.0, 2.0));
        let tree = encode_from_path(format!("objects/tree{}.png", tree.round() as i32).as_str());

        trees.push_str(&format!(
            "<image width=\"35\" height=\"60\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
            x, y, tree
        ));
    }

    trees
}

async fn generate_svg(
    user_name: String,
    year: i32,
    state: AppState,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
) -> String {
    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    const HEIGHT: u32 = 465;

    let stats = get_stats(user_name.clone(), state.token.clone()).await;

    if stats.is_err() {
        return String::new();
    }

    let stats = stats.unwrap();

    println!("{:?}", stats);

    let cells = generate_contribution_cells(year, start_date, weeks, commits);
    let home = generate_home(user_name.clone());
    let trees = generate_trees(
        user_name,
        width.clone(),
        stats.clone().repositories_contributed_to.total_count,
    );

    format!(
        r##"
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 {} {}"
            fill="none"
            style="width: {}px; height: {}px;"
        >
            <rect width="100%" height="100%" fill="#a5c543" />
            <g>{}</g>
            <g>{}</g>
            <g>{}</g>
        </svg>
        "##,
        width, HEIGHT, width, HEIGHT, trees, home, cells
    )
}
