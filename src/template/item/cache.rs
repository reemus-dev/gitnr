use crate::util::fs::{
    cache_exists, cache_filepath, cache_is_invalidated, read_json_file, write_json_file,
};
use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

const CACHE_FILENAME: &str = "template-content.json";
static CACHE: Lazy<Mutex<HashMap<String, TemplateCacheItem>>> =
    Lazy::new(|| TemplateCache::initialize().expect("Failed to initialize template cache"));

/// Represents a cached .gitignore template
#[derive(Debug, Deserialize, Serialize)]
struct TemplateCacheItem {
    updated: SystemTime,
    content: String,
}

/// The cache interface for .gitignore templates
#[derive(Debug)]
pub struct TemplateCache {}

impl TemplateCache {
    fn get_cache_path() -> String {
        cache_filepath(CACHE_FILENAME)
    }

    /// Initialize the template cache with an existing cache file or an empty cache
    fn initialize() -> Result<Mutex<HashMap<String, TemplateCacheItem>>> {
        let path = Self::get_cache_path();
        let exists = cache_exists(&path);
        let invalidated = cache_is_invalidated(&path);

        let map = if exists && !invalidated {
            read_json_file(&path)?
        } else {
            HashMap::new()
        };

        Ok(Mutex::new(map))
    }

    /// Save the template cache to disk
    fn save(map: &HashMap<String, TemplateCacheItem>) -> Result<()> {
        let path = cache_filepath(CACHE_FILENAME);
        write_json_file::<HashMap<String, TemplateCacheItem>>(&path, map)?;
        Ok(())
    }

    /// Get a cached template
    pub fn get(key: &str) -> Result<Option<String>> {
        let map = CACHE.lock().unwrap();
        match map.get(key) {
            Some(item) => {
                if item.updated.elapsed()?.as_secs() > 60 * 60 {
                    return Ok(None);
                }
                Ok(Some(item.content.clone()))
            }
            None => Ok(None),
        }
    }

    /// Set a cached template
    pub fn set(key: &str, content: &str) -> Result<()> {
        let mut map = CACHE.lock().unwrap();
        map.insert(
            key.to_string(),
            TemplateCacheItem {
                updated: SystemTime::now(),
                content: content.to_string(),
            },
        );
        TemplateCache::save(&map)?;
        Ok(())
    }
}
