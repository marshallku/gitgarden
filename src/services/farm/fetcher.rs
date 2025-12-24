use std::collections::HashMap;

use tokio::{spawn, task};
use tracing::error;

use crate::{
    api::{
        contributions::get_daily_commits,
        languages::{get_most_used_languages, MostUsedLanguage},
        stats::{get_stats, User},
        structures::GithubGraphQLError,
    },
    cache::CachedUserData,
    env::state::AppState,
};

type FarmError = Box<dyn std::error::Error + Send + Sync>;

pub struct FetchedData {
    pub commits: HashMap<String, u32>,
    pub stats: Result<User, Vec<GithubGraphQLError>>,
    pub languages: Vec<MostUsedLanguage>,
}

pub struct UserData {
    pub data: FetchedData,
    pub is_cached: bool,
}

pub async fn get_data(user_name: &str, year: i32, state: AppState) -> Result<UserData, FarmError> {
    if let Some(cached) = state.cache.get(user_name, year).await {
        let user = user_name.to_string();
        let state_clone = state.clone();

        spawn(async move {
            if let Ok(data) = fetch_data(&user, year, state_clone.clone()).await {
                let FetchedData {
                    commits,
                    stats,
                    languages,
                } = data;
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

        return Ok(UserData {
            data: FetchedData {
                commits: cached.commits,
                stats: cached.stats,
                languages: cached.languages,
            },
            is_cached: true,
        });
    }

    let FetchedData {
        commits,
        stats,
        languages: most_used_languages,
    } = fetch_data(user_name, year, state.clone()).await?;

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

    Ok(UserData {
        data: FetchedData {
            commits,
            stats,
            languages: most_used_languages,
        },
        is_cached: false,
    })
}

async fn fetch_data(user_name: &str, year: i32, state: AppState) -> Result<FetchedData, FarmError> {
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

    Ok(FetchedData {
        commits,
        stats,
        languages: most_used_languages,
    })
}
