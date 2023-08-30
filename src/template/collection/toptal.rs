use crate::template::item::Template;
use crate::util::fs::{
    cache_exists, cache_filepath, cache_is_invalidated, read_json_file, write_json_file,
};
use crate::util::http::http;
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const CACHE_PATH: &str = "collections/toptal.json";
pub const TOPTAL_API: &str = "https://www.toptal.com/developers/gitignore/api";

/// Static instance of TopTal templates to prevent unnecessary fetching or cache reads
static TEMPLATES: Lazy<Result<TopTalTemplates>> = Lazy::new(TopTalTemplates::new);

#[derive(Debug, Deserialize, Serialize)]
pub struct TopTalTemplates {
    /// When the templates were last updated
    updated: SystemTime,
    templates: Vec<Template>,
}

impl TopTalTemplates {
    /// Create TopTal templates list by initializing from cache or fetching if necessary
    fn new() -> Result<Self> {
        let cache_path = cache_filepath(CACHE_PATH);
        let cache_exists = cache_exists(&cache_path);
        let cache_invalidated = cache_is_invalidated(&cache_path);
        let check_cache = cache_exists && !cache_invalidated;

        // If the cache exists and is not invalidated, read it, otherwise create a new instance
        let mut _self = if check_cache {
            read_json_file(&cache_path)?
        } else {
            Self {
                updated: SystemTime::UNIX_EPOCH,
                templates: Vec::new(),
            }
        };

        // Refresh cache if it's older than 1 hour and save it to the cache file
        if _self.updated.elapsed()?.as_secs() > 60 * 60 {
            _self = Self::fetch()?;
            write_json_file(&cache_path, &_self)?;
        }

        Ok(_self)
    }

    /// Fetch the templates from the TopTal API
    fn fetch() -> Result<Self> {
        let url = format!("{}/list?format=lines", TOPTAL_API);

        let list = http()
            .get(&url)
            .call()
            .with_context(|| "Failed to fetch template list from TopTal")?
            .into_string()
            .with_context(|| "Failed to parse TopTal template list response to string")?;

        let templates = list
            .lines()
            .map(|s| Template::new(&format!("tt:{}", s)))
            .collect::<Result<Vec<Template>>>()?;

        let updated = SystemTime::now();

        Ok(Self { updated, templates })
    }

    /// Get TopTal templates list
    pub fn templates() -> Result<Vec<Template>> {
        match &*TEMPLATES {
            Ok(templates) => Ok(templates.templates.clone()),
            Err(e) => Err(anyhow!(format!("{:?}", e))),
        }
    }
}
