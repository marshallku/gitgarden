use std::collections::HashMap;

use chrono::{DateTime, Timelike, Utc};
use tokio::task;

use crate::{
    api::{
        contributions::get_daily_commits,
        languages::get_most_used_languages,
        stats::{get_stats, ContributionsCollection},
    },
    constants::render::{CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING},
    env::state::AppState,
    render::{
        contribution_cells::ContributionCells, farm::Farm, grasses::Grasses, home::Home,
        trees::Trees,
    },
    utils::date::{calculate_weeks, get_year_range},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TimeRange {
    Dawn,
    Day,
    Night,
}

fn calculate_most_frequent_commit_time(
    contributions: &ContributionsCollection,
) -> Result<TimeRange, Box<dyn std::error::Error>> {
    let mut time_ranges = HashMap::new();

    for repo in &contributions.commit_contributions_by_repository {
        if let Some(ref branch) = repo.repository.default_branch_ref {
            for edge in &branch.target.history.edges {
                if let Ok(date) = DateTime::parse_from_rfc3339(&edge.node.committed_date) {
                    let hour = date.with_timezone(&Utc).hour();
                    let range = match hour {
                        4..=7 => TimeRange::Dawn,
                        8..=18 => TimeRange::Day,
                        _ => TimeRange::Night,
                    };

                    time_ranges
                        .entry(range)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
    }

    time_ranges
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(range, _)| range)
        .ok_or_else(|| "No commits found.".into())
}

pub async fn render_farm_service(
    user_name: &str,
    year: i32,
    state: AppState,
) -> Result<String, Box<dyn std::error::Error>> {
    let commits = task::spawn({
        let user_name = user_name.to_string();

        async move { get_daily_commits(&user_name, year).await.unwrap() }
    });
    let (start_date, end_date) = get_year_range(year).unwrap();
    let weeks = calculate_weeks(start_date, end_date);

    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    const HEIGHT: u32 = 465;

    let stats = task::spawn({
        let user_name = user_name.to_string();
        let token = state.token.clone();

        async move {
            get_stats(
                &user_name,
                format!("{}-01-01T00:00:00Z", year),
                format!("{}-12-31T23:59:59Z", year),
                &token,
            )
            .await
        }
    });

    let most_used_languages = task::spawn({
        let user_name = user_name.to_string();
        let token = state.token.clone();

        async move { get_most_used_languages(&user_name, &token).await }
    });

    let commits = commits.await?;
    let stats = match stats.await? {
        Ok(stats) => stats,
        Err(errors) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", errors[0].message),
            )));
        }
    };
    let most_used_languages = match most_used_languages.await? {
        Ok(languages) => languages,
        Err(errors) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", errors[0].message),
            )));
        }
    };

    let mut farm = Farm::new(width, HEIGHT);
    // length of key of commits / (365 / 2) * 100, upper bound 100
    let progress = (commits.len() as f32 / 182.5 * 100.0).min(100.0);

    let home = Home::new(user_name);

    let time = calculate_most_frequent_commit_time(&stats.contributions_collection)?;

    println!("Most frequent commit time: {:?}", time);

    farm.set_progress(progress);
    farm.add_object(ContributionCells::new(
        year,
        start_date,
        weeks,
        commits,
        most_used_languages,
    ));
    farm.add_object(Grasses::new(
        user_name,
        width,
        &stats.contributions_collection,
        &home.dead_zone,
    ));
    farm.add_object(Trees::new(
        user_name,
        width,
        stats
            .contributions_collection
            .total_repositories_with_contributed_commits,
        &home.dead_zone,
    ));
    farm.add_object(home);

    Ok(farm.render())
}
