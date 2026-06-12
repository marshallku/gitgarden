#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;

    use crate::cache::{CachedUserData, GithubCache};

    const SOFT_TTL: Duration = Duration::from_millis(50);
    const HARD_TTL: Duration = Duration::from_millis(200);

    fn user_data() -> CachedUserData {
        CachedUserData {
            commits: HashMap::new(),
            languages: vec![],
            stats: Err(vec![]),
        }
    }

    fn cache() -> Arc<GithubCache> {
        Arc::new(GithubCache::new(SOFT_TTL, HARD_TTL))
    }

    #[tokio::test]
    async fn test_fresh_entry_is_not_stale() {
        let cache = cache();
        cache.set("user", 2026, user_data()).await;

        let lookup = cache.get("user", 2026).await.unwrap();

        assert!(!lookup.is_stale);
    }

    #[tokio::test]
    async fn test_entry_past_soft_ttl_is_stale() {
        let cache = cache();
        cache.set("user", 2026, user_data()).await;

        tokio::time::sleep(SOFT_TTL + Duration::from_millis(10)).await;
        let lookup = cache.get("user", 2026).await.unwrap();

        assert!(lookup.is_stale);
    }

    #[tokio::test]
    async fn test_entry_past_hard_ttl_is_evicted_on_get() {
        let cache = cache();
        cache.set("user", 2026, user_data()).await;

        tokio::time::sleep(HARD_TTL + Duration::from_millis(10)).await;

        assert!(cache.get("user", 2026).await.is_none());
    }

    #[tokio::test]
    async fn test_key_is_case_insensitive() {
        let cache = cache();
        cache.set("User", 2026, user_data()).await;

        assert!(cache.get("uSeR", 2026).await.is_some());
    }

    #[tokio::test]
    async fn test_evict_expired_removes_only_expired_entries() {
        let cache = cache();
        cache.set("old", 2026, user_data()).await;

        tokio::time::sleep(HARD_TTL + Duration::from_millis(10)).await;
        cache.set("new", 2026, user_data()).await;
        cache.evict_expired().await;

        assert_eq!(cache.len().await, 1);
        assert!(cache.get("new", 2026).await.is_some());
    }

    #[tokio::test]
    async fn test_revalidation_requires_existing_entry() {
        let cache = cache();

        assert!(cache
            .try_begin_revalidation("missing", 2026)
            .await
            .is_none());
    }

    #[tokio::test]
    async fn test_revalidation_excludes_concurrent_attempts() {
        let cache = cache();
        cache.set("user", 2026, user_data()).await;

        let guard = cache.try_begin_revalidation("user", 2026).await;

        assert!(guard.is_some());
        assert!(cache.try_begin_revalidation("user", 2026).await.is_none());
    }

    #[tokio::test]
    async fn test_revalidation_cooldown_blocks_immediate_retry() {
        let cache = cache();
        cache.set("user", 2026, user_data()).await;

        let guard = cache.try_begin_revalidation("user", 2026).await;
        drop(guard);

        // The in-flight marker is released, but the 60s cooldown still applies.
        assert!(cache.try_begin_revalidation("user", 2026).await.is_none());
    }

    #[tokio::test]
    async fn test_successful_set_resets_cooldown() {
        let cache = cache();
        cache.set("user", 2026, user_data()).await;

        let guard = cache.try_begin_revalidation("user", 2026).await;
        drop(guard);
        cache.set("user", 2026, user_data()).await;

        assert!(cache.try_begin_revalidation("user", 2026).await.is_some());
    }
}
