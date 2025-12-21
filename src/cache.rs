use std::collections::HashMap;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use crate::api::{
    languages::MostUsedLanguage,
    stats::User,
    structures::GithubGraphQLError,
};

#[derive(Clone)]
pub struct CachedUserData {
    pub commits: HashMap<String, u32>,
    pub languages: Vec<MostUsedLanguage>,
    pub stats: Result<User, Vec<GithubGraphQLError>>,
}

struct CacheEntry {
    data: CachedUserData,
    expires_at: Instant,
}

pub struct GithubCache {
    entries: RwLock<HashMap<(String, i32), CacheEntry>>,
    ttl: Duration,
}

impl GithubCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            ttl,
        }
    }

    pub async fn get(&self, username: &str, year: i32) -> Option<CachedUserData> {
        let entries = self.entries.read().await;
        let key = (username.to_lowercase(), year);

        entries.get(&key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    pub async fn set(&self, username: &str, year: i32, data: CachedUserData) {
        let mut entries = self.entries.write().await;
        let key = (username.to_lowercase(), year);

        entries.insert(key, CacheEntry {
            data,
            expires_at: Instant::now() + self.ttl,
        });
    }
}
