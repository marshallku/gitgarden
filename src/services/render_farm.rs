use std::collections::HashMap;

use tokio::{spawn, task};
use tracing::{error, info};

use crate::{
    api::{
        contributions::get_daily_commits,
        languages::get_most_used_languages,
        stats::{get_stats, User},
        structures::GithubGraphQLError,
    },
    cache::CachedUserData,
    constants::render::{CELL_SIZE, CELL_SPACING, GRID_LEFT_PADDING},
    env::state::AppState,
    render::{
        contribution_cells::ContributionCells, farm::Farm, grasses::Grasses, home::Home,
        trees::Trees,
    },
    utils::date::{calculate_day_passed_in_year, calculate_weeks, get_year_range},
};

async fn get_data(
    user_name: &str,
    year: i32,
    state: AppState,
) -> Result<
    (
        HashMap<String, u32>,
        Result<User, Vec<GithubGraphQLError>>,
        Vec<crate::api::languages::MostUsedLanguage>,
    ),
    Box<dyn std::error::Error + Send + Sync>,
> {
    if let Some(cached) = state.cache.get(user_name, year).await {
        let user = user_name.to_string();
        let state_clone = state.clone();

        spawn(async move {
            if let Ok((commits, stats, languages)) =
                fetch_data(&user, year, state_clone.clone()).await
            {
                info!("Cached data updated for user in background: {}", user);
                state_clone
                    .cache
                    .set(
                        &user,
                        year,
                        CachedUserData {
                            commits,
                            languages,
                            stats,
                        },
                    )
                    .await;
            }
        });

        info!("Cached data found for user: {}", user_name);
        return Ok((cached.commits, cached.stats, cached.languages));
    }

    let (commits, stats, most_used_languages) = fetch_data(user_name, year, state.clone()).await?;

    state
        .clone()
        .cache
        .set(
            user_name,
            year,
            CachedUserData {
                commits: commits.clone(),
                languages: most_used_languages.clone(),
                stats: stats.clone(),
            },
        )
        .await;

    info!("Response without cached data for user: {}", user_name);
    Ok((commits, stats, most_used_languages))
}

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
    Box<dyn std::error::Error + Send + Sync>,
> {
    let commits = task::spawn({
        let user_name = user_name.to_string();

        async move { get_daily_commits(&user_name, year).await }
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

    let commits = match commits.await? {
        Ok(commits) => commits,
        Err(error) => {
            error!("Failed to get daily commits: {:?}", error);
            HashMap::with_capacity(0)
        }
    };
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
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (commits, stats, most_used_languages) = get_data(user_name, year, state).await?;

    let (start_date, end_date) = match get_year_range(year) {
        Some((start_date, end_date)) => (start_date, end_date),
        None => {
            error!("Failed to get year range");
            return Err("Failed to get year range".into());
        }
    };
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
