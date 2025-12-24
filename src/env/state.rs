use std::sync::Arc;
use std::time::Duration;

use super::app::Env;
use crate::cache::GithubCache;
use dotenv::dotenv;

const CACHE_TTL_SECS: u64 = 24 * 60 * 60;

#[derive(Clone)]
pub struct AppState {
    pub host: String,
    pub port: u16,
    pub token: String,
    pub cache: Arc<GithubCache>,
}

impl AppState {
    pub fn from_env() -> Self {
        dotenv().ok();

        let env = Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            token: env.token.into_owned(),
            cache: Arc::new(GithubCache::new(Duration::from_secs(CACHE_TTL_SECS))),
        }
    }
}
