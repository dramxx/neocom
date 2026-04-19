#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static SYSTEM_CACHE: std::sync::OnceLock<SystemCache> = std::sync::OnceLock::new();

#[allow(dead_code)]
pub fn get_system_cache() -> &'static SystemCache {
    SYSTEM_CACHE.get_or_init(|| {
        SystemCache::new().expect("Failed to initialize system cache")
    })
}

const ESI_BASE: &str = "https://esi.evetech.net/latest";
const CACHE_TTL_SECS: u64 = 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemCache {
    #[serde(rename = "systems")]
    systems: Vec<SystemEntry>,
    #[serde(rename = "cached_at")]
    cached_at: u64,
    #[serde(rename = "expires_at")]
    expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemEntry {
    pub id: i64,
    pub name: String,
}

impl SystemCache {
    pub fn new() -> Result<Self> {
        let cache_path = Self::cache_path();

        if cache_path.exists() {
            let content = fs::read_to_string(&cache_path)
                .with_context(|| format!("Failed to read cache: {}", cache_path.display()))?;
            let cache: SystemCache = serde_json::from_str(&content)
                .with_context(|| "Failed to parse cache")?;

            let now = current_timestamp();
            if cache.expires_at > now {
                return Ok(cache);
            }
            eprintln!("Cache expired, refreshing...");
        }

        Self::fetch_and_cache()
    }

    pub fn refresh(&self) -> Result<Self> {
        Self::fetch_and_cache()
    }

    pub fn get_completions(&self, prefix: &str) -> Vec<String> {
        let prefix_lower = prefix.to_lowercase();
        let mut names: Vec<String> = self
            .systems
            .iter()
            .filter(|s| s.name.to_lowercase().starts_with(&prefix_lower))
            .map(|s| s.name.clone())
            .collect();
        names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        names
    }

    fn cache_path() -> PathBuf {
        dirs::data_local_dir()
            .expect("No local data dir")
            .join("neocom")
            .join("systems.json")
    }

    fn fetch_and_cache() -> Result<Self> {
        let ids_url = format!("{}/universe/systems/", ESI_BASE);
        let client = ureq::Agent::new_with_defaults();
        let mut ids_response = client
            .get(&ids_url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| "Failed to fetch system IDs")?;

        let expires_header = ids_response
            .headers()
            .get("expires")
            .and_then(|v| v.to_str().ok())
            .map(|s| http_date_to_timestamp(s))
            .unwrap_or(current_timestamp() + CACHE_TTL_SECS);

        let system_ids: Vec<i64> = ids_response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse system IDs")?;

        if system_ids.is_empty() {
            anyhow::bail!("No system IDs returned from ESI");
        }

        let systems = fetch_system_details(&system_ids)?;
        let cached_at = current_timestamp();
        let expires_at = expires_header.max(cached_at + CACHE_TTL_SECS);

        let cache = SystemCache {
            systems,
            cached_at,
            expires_at,
        };

        let cache_path = Self::cache_path();
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create cache dir: {}", parent.display()))?;
        }

        let content = serde_json::to_string_pretty(&cache)
            .context("Failed to serialize cache")?;
        fs::write(&cache_path, content)
            .with_context(|| format!("Failed to write cache: {}", cache_path.display()))?;

        Ok(cache)
    }
}

fn fetch_system_details(system_ids: &[i64]) -> Result<Vec<SystemEntry>> {
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const MAX_CONCURRENT: usize = 50;

    let client = Arc::new(ureq::Agent::new_with_defaults());
    let (tx, rx) = mpsc::channel();
    let active = Arc::new(AtomicUsize::new(0));
    let mut results = Vec::new();

    for chunk in system_ids.chunks(MAX_CONCURRENT) {
        let mut handles = Vec::new();

        for &id in chunk {
            let tx = tx.clone();
            let client = Arc::clone(&client);
            let active = Arc::clone(&active);

            let handle = std::thread::spawn(move || {
                active.fetch_add(1, Ordering::SeqCst);

                let url = format!("{}/universe/systems/{}/", ESI_BASE, id);
                let result = client
                    .get(&url)
                    .header("User-Agent", "neocom/0.1")
                    .call()
                    .ok()
                    .and_then(|mut r| r.body_mut().read_json::<SystemDetails>().ok())
                    .map(|details| SystemEntry {
                        id,
                        name: details.name,
                    });

                let _ = tx.send(result);
                active.fetch_sub(1, Ordering::SeqCst);
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join();
        }
    }

    drop(tx);

    while let Ok(result) = rx.try_recv() {
        if let Some(entry) = result {
            results.push(entry);
        }
    }

    results.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(results)
}

#[derive(Deserialize)]
struct SystemDetails {
    name: String,
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

fn http_date_to_timestamp(_http_date: &str) -> u64 {
    current_timestamp() + CACHE_TTL_SECS
}