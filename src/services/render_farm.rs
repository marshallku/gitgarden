use std::collections::HashMap;

use tokio::task;
use tracing::error;

use crate::{
    api::{
        contributions::get_daily_commits,
        languages::get_most_used_languages,
        stats::{get_stats, User},
        structures::GithubGraphQLError,
    },
    constants::render::{CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING},
    env::state::AppState,
    render::{
        contribution_cells::ContributionCells, farm::Farm, grasses::Grasses, home::Home,
        trees::Trees,
    },
    utils::date::{calculate_day_passed_in_year, calculate_weeks, get_year_range},
};

async fn fetch_data(
    user_name: &str,
    year: i32,
    state: AppState,
) -> Result<
    (
        HashMap<String, u32>,
        Result<User, Vec<GithubGraphQLError>>,
        Vec<crate::api::languages::MostUsedLanguage>,
    ),
    Box<dyn std::error::Error>,
> {
    let commits = task::spawn({
        let user_name = user_name.to_string();

        async move { get_daily_commits(&user_name, year).await.unwrap() }
    });
    let most_used_languages = task::spawn({
        let user_name = user_name.to_string();
        let token = state.token.clone();

        async move { get_most_used_languages(&user_name, year, &token).await }
    });
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

    let commits = commits.await?;
    let stats = stats.await?;
    let most_used_languages = match most_used_languages.await? {
        Ok(languages) => languages,
        Err(err) => {
            error!("Failed to get most used languages: {:?}", err);
            vec![]
        }
    };

    Ok((commits, stats, most_used_languages))
}

pub async fn render_farm_service(
    user_name: &str,
    year: i32,
    state: AppState,
) -> Result<String, Box<dyn std::error::Error>> {
    let (commits, stats, most_used_languages) = fetch_data(user_name, year, state).await?;

    let (start_date, end_date) = get_year_range(year).unwrap();
    let weeks = calculate_weeks(start_date, end_date);

    let width = weeks as u32 * (CELL_SIZE + CELL_SPACING) + GRID_LEFT_PADDING * 2;
    let height = 465;

    let mut farm = Farm::new(width, height);
    let progress =
        (commits.len() as f32 / (calculate_day_passed_in_year(year) / 2.0) * 100.0).min(100.0);

    let home = Home::new(user_name);

    farm.set_progress(progress);
    farm.add_object(ContributionCells::new(
        year,
        start_date,
        weeks,
        commits,
        most_used_languages,
    ));

    if stats.is_ok() {
        let stats = stats.unwrap();

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
    }

    farm.add_object(home);

    Ok(farm.render())
}
