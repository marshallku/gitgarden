use std::collections::HashMap;

use tokio::{spawn, task};
use tracing::{error, info, warn};

use crate::{
    api::{
        contributions::get_daily_commits,
        languages::{get_most_used_languages, MostUsedLanguage},
        stats::{get_stats, User},
        structures::{GithubGraphQLError, ERROR_TYPE_REQUEST, ERROR_TYPE_RESPONSE},
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

struct FetchOutcome {
    data: FetchedData,
    /// Whether every source either succeeded or failed in a cacheable way.
    /// Incomplete outcomes are served to the client but never cached, so a
    /// transient upstream failure cannot poison the cache until the next TTL.
    complete: bool,
}

pub struct UserData {
    pub data: FetchedData,
    pub is_cached: bool,
}

pub async fn get_data(user_name: &str, year: i32, state: AppState) -> Result<UserData, FarmError> {
    if let Some(lookup) = state.cache.get(user_name, year).await {
        if lookup.is_stale {
            revalidate_in_background(user_name, year, state.clone()).await;
        }

        return Ok(UserData {
            data: FetchedData {
                commits: lookup.data.commits,
                stats: lookup.data.stats,
                languages: lookup.data.languages,
            },
            is_cached: true,
        });
    }

    let outcome = fetch_data(user_name, year, state.clone()).await?;

    if outcome.complete {
        state
            .cache
            .set(
                user_name,
                year,
                CachedUserData {
                    commits: outcome.data.commits.clone(),
                    languages: outcome.data.languages.clone(),
                    stats: outcome.data.stats.clone(),
                },
            )
            .await;
    }

    Ok(UserData {
        data: outcome.data,
        is_cached: false,
    })
}

async fn revalidate_in_background(user_name: &str, year: i32, state: AppState) {
    let Some(guard) = state.cache.try_begin_revalidation(user_name, year).await else {
        return;
    };
    let user = user_name.to_string();

    spawn(async move {
        let _guard = guard;

        info!("Revalidating cache: user = {}, year = {}", user, year);

        match fetch_data(&user, year, state.clone()).await {
            Ok(outcome) if outcome.complete => {
                state.cache.set(&user, year, outcome.data.into()).await;
            }
            Ok(_) => {
                warn!(
                    "Skipped cache update with incomplete data: user = {}, year = {}",
                    user, year
                );
            }
            Err(err) => {
                error!(
                    "Failed to revalidate cache: user = {}, year = {}, error = {:?}",
                    user, year, err
                );
            }
        }
    });
}

async fn fetch_data(user_name: &str, year: i32, state: AppState) -> Result<FetchOutcome, FarmError> {
    let commits = task::spawn({
        let client = state.http.clone();
        let user_name = user_name.to_string();

        async move { get_daily_commits(&client, &user_name, year).await }
    });
    let most_used_languages = task::spawn({
        let client = state.http.clone();
        let user_name = user_name.to_string();
        let token = state.token.clone();

        async move { get_most_used_languages(&client, &user_name, year, &token).await }
    });
    let stats = task::spawn({
        let client = state.http.clone();
        let user_name = user_name.to_string();
        let token = state.token.clone();

        async move {
            get_stats(
                &client,
                &user_name,
                format!("{}-01-01T00:00:00Z", year),
                format!("{}-12-31T23:59:59Z", year),
                &token,
            )
            .await
        }
    });

    let (commits, commits_ok) = match commits.await? {
        Ok(commits) => (commits, true),
        Err(error) => {
            error!("Failed to get daily commits: {:?}", error);
            (HashMap::with_capacity(0), false)
        }
    };
    let stats = stats.await?;
    let (most_used_languages, languages_ok) = match most_used_languages.await? {
        Ok(languages) => (languages, true),
        Err(err) => {
            error!("Failed to get most used languages: {:?}", err);
            // A durable GraphQL error (e.g. NOT_FOUND) means the empty list IS
            // the correct result for this user, so it stays cacheable.
            let cacheable = is_cacheable_errors(&err);
            (vec![], cacheable)
        }
    };

    let complete = commits_ok && languages_ok && is_cacheable_stats(&stats);

    Ok(FetchOutcome {
        data: FetchedData {
            commits,
            stats,
            languages: most_used_languages,
        },
        complete,
    })
}

/// `Ok` results and genuine GraphQL errors (e.g. NOT_FOUND for a nonexistent
/// user) are durable and worth caching. `RequestError`/`ResponseError` are
/// synthesized locally for transport or parse failures and must not be.
fn is_cacheable_stats(stats: &Result<User, Vec<GithubGraphQLError>>) -> bool {
    match stats {
        Ok(_) => true,
        Err(errors) => is_cacheable_errors(errors),
    }
}

fn is_cacheable_errors(errors: &[GithubGraphQLError]) -> bool {
    errors
        .iter()
        .all(|e| e.error_type != ERROR_TYPE_REQUEST && e.error_type != ERROR_TYPE_RESPONSE)
}

impl From<FetchedData> for CachedUserData {
    fn from(data: FetchedData) -> Self {
        Self {
            commits: data.commits,
            languages: data.languages,
            stats: data.stats,
        }
    }
}
