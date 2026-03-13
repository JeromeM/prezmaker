use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ApiCache {
    inner: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

struct CacheEntry {
    data: String,
    created_at: Instant,
    ttl: Duration,
}

impl ApiCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let cache = self.inner.lock().ok()?;
        let entry = cache.get(key)?;
        if entry.created_at.elapsed() > entry.ttl {
            return None;
        }
        Some(entry.data.clone())
    }

    pub fn set(&self, key: String, data: String, ttl: Duration) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.insert(key, CacheEntry {
                data,
                created_at: Instant::now(),
                ttl,
            });
        }
    }

    pub fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let raw = self.get(key)?;
        serde_json::from_str(&raw).ok()
    }

    pub fn set_json<T: serde::Serialize>(&self, key: String, value: &T, ttl: Duration) {
        if let Ok(json) = serde_json::to_string(value) {
            self.set(key, json, ttl);
        }
    }
}

impl Default for ApiCache {
    fn default() -> Self {
        Self::new()
    }
}
