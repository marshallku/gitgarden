use std::collections::HashMap;

use crate::{
    api::{
        contributions::get_daily_commits,
        stats::{get_stats, ContributionsCollection},
    },
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Objects {
    FlowerOne,
    FlowerTwo,
    FlowerThree,
    FlowerFour,
    TreeOne,
    TreeTwo,
    GrassOne,
    GrassTwo,
    GrassThree,
    GrassFour,
    GrassFive,
    GrassSix,
    Dirt,
}

impl Objects {
    pub fn to_string(&self) -> String {
        match self {
            Objects::FlowerOne => "flower-1".to_string(),
            Objects::FlowerTwo => "flower-2".to_string(),
            Objects::FlowerThree => "flower-3".to_string(),
            Objects::FlowerFour => "flower-4".to_string(),
            Objects::TreeOne => "tree-1".to_string(),
            Objects::TreeTwo => "tree-2".to_string(),
            Objects::GrassOne => "grass-1".to_string(),
            Objects::GrassTwo => "grass-2".to_string(),
            Objects::GrassThree => "grass-3".to_string(),
            Objects::GrassFour => "grass-4".to_string(),
            Objects::GrassFive => "grass-5".to_string(),
            Objects::GrassSix => "grass-6".to_string(),
            Objects::Dirt => "dirt".to_string(),
        }
    }

    pub fn to_path(&self) -> String {
        match self {
            Objects::FlowerOne => "flowers/1-1.png".to_string(),
            Objects::FlowerTwo => "flowers/1-2.png".to_string(),
            Objects::FlowerThree => "flowers/1-3.png".to_string(),
            Objects::FlowerFour => "flowers/1-4.png".to_string(),
            Objects::TreeOne => "objects/tree1.png".to_string(),
            Objects::TreeTwo => "objects/tree2.png".to_string(),
            Objects::GrassOne => "field/grass1.png".to_string(),
            Objects::GrassTwo => "field/grass2.png".to_string(),
            Objects::GrassThree => "field/grass3.png".to_string(),
            Objects::GrassFour => "field/grass4.png".to_string(),
            Objects::GrassFive => "field/grass5.png".to_string(),
            Objects::GrassSix => "field/grass6.png".to_string(),
            Objects::Dirt => "field/dirt2.png".to_string(),
        }
    }

    pub fn to_size(&self) -> (u32, u32) {
        match self {
            Objects::FlowerOne => (16, 16),
            Objects::FlowerTwo => (16, 16),
            Objects::FlowerThree => (16, 16),
            Objects::FlowerFour => (16, 16),
            Objects::TreeOne => (35, 60),
            Objects::TreeTwo => (35, 60),
            Objects::GrassOne => (16, 16),
            Objects::GrassTwo => (16, 16),
            Objects::GrassThree => (16, 16),
            Objects::GrassFour => (16, 16),
            Objects::GrassFive => (16, 16),
            Objects::GrassSix => (16, 16),
            Objects::Dirt => (16, 16),
        }
    }

    pub fn iter() -> impl Iterator<Item = Objects> {
        [
            Objects::FlowerOne,
            Objects::FlowerTwo,
            Objects::FlowerThree,
            Objects::FlowerFour,
            Objects::TreeOne,
            Objects::TreeTwo,
            Objects::GrassOne,
            Objects::GrassTwo,
            Objects::GrassThree,
            Objects::GrassFour,
            Objects::GrassFive,
            Objects::GrassSix,
            Objects::Dirt,
        ]
        .iter()
        .copied()
    }
}

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

fn generate_home(user_name: String) -> String {
    let (x, y) = generate_coordinate(user_name, (80.0, 730.0), (25.0, 70.0));
    let home = encode_from_path("objects/home.png");
    let road = encode_from_path("objects/stone_road.png");

    format!(
        "<image width=\"151\" height=\"155\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" /><image width=\"31\" height=\"89\" x=\"{}\" y=\"{}\" xlink:href=\"data:image/png;base64,{}\" />",
        x, y, home, x + 67.0, y + 152.0, road
    )
}

fn generate_grasses(
    user_name: String,
    width: u32,
    contributions_collection: ContributionsCollection,
) -> String {
    let mut grasses = String::new();
    let x_max = width as f64 - 16.0;
    let y_max = 300.0;

    for i in 0..contributions_collection.total_issue_contributions {
        let (x, y) = generate_coordinate(
            &format!("{}-grass-1-{}", user_name, i),
            (0.0, x_max),
            (0.0, y_max),
        );

        grasses.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x,
            y,
            Objects::GrassOne.to_string()
        ));
    }

    for i in 0..contributions_collection.total_pull_request_contributions {
        let (x, y) = generate_coordinate(
            &format!("{}-grass-2-{}", user_name, i),
            (0.0, x_max),
            (0.0, y_max),
        );

        grasses.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x,
            y,
            Objects::GrassTwo.to_string()
        ));
    }

    for i in 0..contributions_collection.total_pull_request_review_contributions {
        let (x, y) = generate_coordinate(
            &format!("{}-grass-3-{}", user_name, i),
            (0.0, x_max),
            (0.0, y_max),
        );

        grasses.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x,
            y,
            Objects::GrassThree.to_string()
        ));
    }

    for i in 0..contributions_collection.total_repositories_with_contributed_issues {
        let (x, y) = generate_coordinate(
            &format!("{}-grass-4-{}", user_name, i),
            (0.0, x_max),
            (0.0, y_max),
        );

        grasses.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x,
            y,
            Objects::GrassFour.to_string()
        ));
    }

    for i in 0..contributions_collection.total_repositories_with_contributed_pull_requests {
        let (x, y) = generate_coordinate(
            &format!("{}-grass-5-{}", user_name, i),
            (0.0, x_max),
            (0.0, y_max),
        );

        grasses.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x,
            y,
            Objects::GrassFive.to_string()
        ));
    }

    for i in 0..contributions_collection.total_repositories_with_contributed_pull_request_reviews {
        let (x, y) = generate_coordinate(
            &format!("{}-grass-6-{}", user_name, i),
            (0.0, x_max),
            (0.0, y_max),
        );

        grasses.push_str(&format!(
            "<use x=\"{}\" y=\"{}\" xlink:href=\"#{}\" />",
            x,
            y,
            Objects::GrassSix.to_string()
        ));
    }

    grasses
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
        let (tree_kind, _) = generate_coordinate("{}-tree-kind-{}", (1.0, 2.0), (1.0, 2.0));
        let tree = match tree_kind as u32 {
            1 => Objects::TreeOne.to_string(),
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
    user_name: String,
    year: i32,
    state: AppState,
    start_date: NaiveDate,
    weeks: usize,
    commits: HashMap<String, u32>,
) -> String {
    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    const HEIGHT: u32 = 465;

    let stats = get_stats(
        user_name.clone(),
        format!("{}-01-01T00:00:00Z", year),
        format!("{}-12-31T23:59:59Z", year),
        state.token.clone(),
    )
    .await;

    if stats.is_err() {
        return String::new();
    }

    let stats = stats.unwrap();

    println!("{:?}", stats);

    let objects = register_objects();
    let cells = generate_contribution_cells(year, start_date, weeks, commits);
    let home = generate_home(user_name.clone());
    let grasses = generate_grasses(
        user_name.clone(),
        width.clone(),
        stats.contributions_collection.clone(),
    );
    let trees = generate_trees(
        user_name,
        width.clone(),
        stats
            .clone()
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
