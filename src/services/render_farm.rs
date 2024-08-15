use std::collections::HashMap;

use crate::{
    api::{
        contributions::get_daily_commits,
        stats::{get_stats, ContributionsCollection},
    },
    env::state::AppState,
    render::objects::Objects,
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

fn register_objects() -> String {
    let mut objects = String::new();

    for object in Objects::iter() {
        let path = object.to_path();
        let encoded = encode_from_path(&path);

        objects.push_str(&format!(
            "<image id=\"{}\" width=\"{}\" height=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
            object.to_string(),
            object.to_size().0,
            object.to_size().1,
            encoded
        ));
    }

    objects
}

pub async fn render_farm_service(user_name: &str, year: i32, state: AppState) -> String {
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

            cells.push_str(&format!(
                "<use  x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
                x,
                y,
                Objects::Dirt.to_string()
            ));

            if *commit_level > 0 {
                let flower = match *commit_level {
                    1 => Objects::FlowerOne.to_string(),
                    2 => Objects::FlowerTwo.to_string(),
                    3 => Objects::FlowerThree.to_string(),
                    _ => Objects::FlowerFour.to_string(),
                };

                cells.push_str(&format!(
                    "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
                    x, y, flower
                ));
            }
        }
    }

    cells
}

fn generate_home(user_name: &str) -> String {
    let (x, y) = generate_coordinate(user_name, (80.0, 730.0), (25.0, 70.0));
    let home = encode_from_path("objects/home.png");
    let road = encode_from_path("objects/stone_road.png");

    format!(
        "<image width=\"151\" height=\"155\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" /><image width=\"31\" height=\"89\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
        x, y, home, x + 67.0, y + 152.0, road
    )
}

fn generate_grasses(
    user_name: &str,
    width: u32,
    contributions_collection: &ContributionsCollection,
) -> String {
    let x_max = width as f64 - 16.0;
    let y_max = 300.0;

    let grass_types = [
        (
            Objects::GrassOne,
            contributions_collection.total_issue_contributions,
        ),
        (
            Objects::GrassTwo,
            contributions_collection.total_pull_request_contributions,
        ),
        (
            Objects::GrassThree,
            contributions_collection.total_pull_request_review_contributions,
        ),
        (
            Objects::GrassFour,
            contributions_collection.total_repositories_with_contributed_issues,
        ),
        (
            Objects::GrassFive,
            contributions_collection.total_repositories_with_contributed_pull_requests,
        ),
        (
            Objects::GrassSix,
            contributions_collection.total_repositories_with_contributed_pull_request_reviews,
        ),
    ];

    grass_types
        .iter()
        .enumerate()
        .flat_map(|(index, (grass_type, count))| {
            (0..*count).map(move |i| {
                let (x, y) = generate_coordinate(
                    &format!("{}-grass-{}-{}", user_name, index + 1, i),
                    (0.0, x_max),
                    (0.0, y_max),
                );
                format!(
                    "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
                    x,
                    y,
                    grass_type.to_string()
                )
            })
        })
        .collect::<Vec<String>>()
        .join("")
}

fn generate_trees(user_name: &str, width: u32, repositories_contributed_to: i32) -> String {
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
        let (tree_kind, _) = generate_coordinate(
            &format!("{}-tree-kind-{}", user_name, x + y),
            (1.0, 2.0),
            (1.0, 2.0),
        );
        let tree = match tree_kind.round() {
            1.0 => Objects::TreeOne.to_string(),
            _ => Objects::TreeTwo.to_string(),
        };

        trees.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x, y, tree
        ));
    }

    trees
}

async fn generate_svg(
    user_name: &str,
    year: i32,
    state: AppState,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
) -> String {
    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    const HEIGHT: u32 = 465;

    let stats = get_stats(
        &user_name,
        format!("{}-01-01T00:00:00Z", year),
        format!("{}-12-31T23:59:59Z", year),
        state.token,
    )
    .await;

    if stats.is_err() {
        return String::new();
    }

    let stats = stats.unwrap();

    println!("{:?}", stats);

    let objects = register_objects();
    let cells = generate_contribution_cells(year, start_date, weeks, commits);
    let home = generate_home(&user_name);
    let grasses = generate_grasses(&user_name, width, &stats.contributions_collection);
    let trees = generate_trees(
        &user_name,
        width,
        stats
            .contributions_collection
            .total_repositories_with_contributed_commits,
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
            <defs>{}</defs>
            <g>{}</g>
            <g>{}</g>
            <g>{}</g>
            <g>{}</g>
        </svg>
        "##,
        width, HEIGHT, width, HEIGHT, objects, grasses, trees, home, cells
    )
}
