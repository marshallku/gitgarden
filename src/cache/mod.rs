mod __tests__;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use crate::api::{
    languages::MostUsedLanguage,
    stats::User,
    structures::GithubGraphQLError,
};

/// Minimum gap between background revalidation attempts for the same key,
/// so upstream outages degrade to one retry per minute instead of one per request.
const REVALIDATION_COOLDOWN: Duration = Duration::from_secs(60);

type CacheKey = (String, i32);

#[derive(Clone)]
pub struct CachedUserData {
    pub commits: HashMap<String, u32>,
    pub languages: Vec<MostUsedLanguage>,
    pub stats: Result<User, Vec<GithubGraphQLError>>,
}

pub struct CacheLookup {
    pub data: CachedUserData,
    pub is_stale: bool,
}

struct CacheEntry {
    data: CachedUserData,
    created_at: Instant,
    last_refresh_attempt: Option<Instant>,
}

pub struct GithubCache {
    entries: RwLock<HashMap<CacheKey, CacheEntry>>,
    revalidating: Mutex<HashSet<CacheKey>>,
    soft_ttl: Duration,
    hard_ttl: Duration,
}

/// Owned in-flight marker for one key. Dropping it (including on panic of the
/// task that holds it) re-allows revalidation for that key.
pub struct RevalidationGuard {
    cache: Arc<GithubCache>,
    key: CacheKey,
}

impl Drop for RevalidationGuard {
    fn drop(&mut self) {
        self.cache
            .revalidating
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&self.key);
    }
}

impl GithubCache {
    pub fn new(soft_ttl: Duration, hard_ttl: Duration) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            revalidating: Mutex::new(HashSet::new()),
            soft_ttl,
            hard_ttl,
        }
    }

    pub async fn get(&self, username: &str, year: i32) -> Option<CacheLookup> {
        let entries = self.entries.read().await;
        let entry = entries.get(&cache_key(username, year))?;
        let age = entry.created_at.elapsed();

        if age >= self.hard_ttl {
            return None;
        }

        Some(CacheLookup {
            data: entry.data.clone(),
            is_stale: age >= self.soft_ttl,
        })
    }

    pub async fn set(&self, username: &str, year: i32, data: CachedUserData) {
        let mut entries = self.entries.write().await;

        entries.insert(
            cache_key(username, year),
            CacheEntry {
                data,
                created_at: Instant::now(),
                last_refresh_attempt: None,
            },
        );
    }

    /// Claims the right to revalidate a key. Returns `None` when the key is
    /// unknown, another revalidation is in flight, or the cooldown since the
    /// last attempt has not elapsed.
    pub async fn try_begin_revalidation(
        self: &Arc<Self>,
        username: &str,
        year: i32,
    ) -> Option<RevalidationGuard> {
        let key = cache_key(username, year);
        let mut entries = self.entries.write().await;
        let entry = entries.get_mut(&key)?;

        if let Some(last_attempt) = entry.last_refresh_attempt {
            if last_attempt.elapsed() < REVALIDATION_COOLDOWN {
                return None;
            }
        }

        {
            let mut revalidating = self
                .revalidating
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if !revalidating.insert(key.clone()) {
                return None;
            }
        }

        entry.last_refresh_attempt = Some(Instant::now());

        Some(RevalidationGuard {
            cache: Arc::clone(self),
            key,
        })
    }

    pub async fn evict_expired(&self) {
        let mut entries = self.entries.write().await;
        entries.retain(|_, entry| entry.created_at.elapsed() < self.hard_ttl);
    }

    #[cfg(test)]
    pub async fn len(&self) -> usize {
        self.entries.read().await.len()
    }
}

fn cache_key(username: &str, year: i32) -> CacheKey {
    (username.to_lowercase(), year)
}
