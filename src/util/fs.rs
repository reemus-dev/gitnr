use crate::cli::get_cli;
use crate::util::package;
use anyhow::Context;
use anyhow::Result;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Reads a JSON file and parses it into the given type
pub fn read_json_file<T: DeserializeOwned>(filepath: &str) -> Result<T> {
    let content = fs::read_to_string(filepath)
        .with_context(|| format!("Failed to read JSON file: {}", filepath))?;
    let data = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from file: {}", filepath))?;
    Ok(data)
}

/// Writes the given data to a JSON file
pub fn write_json_file<T: Serialize>(filepath: &str, data: &T) -> Result<()> {
    let cache_json = serde_json::to_string(data)?;
    fs::write(filepath, cache_json)
        .with_context(|| format!("Failed to write JSON data to file: {}", filepath))?;
    Ok(())
}

static CACHE_RECHECKED: Lazy<Mutex<HashMap<String, bool>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Returns the cache filepath for the given name
pub fn cache_filepath(name: &str) -> String {
    let cache_dir: PathBuf = match dirs::cache_dir() {
        Some(dir) => dir.join(package::NAME.to_lowercase()),
        None => std::env::current_dir()
            .unwrap()
            .join(format!(".{}", package::NAME.to_lowercase())),
    };

    let cache_path = name.split('/').fold(cache_dir, |acc, part| acc.join(part));

    fs::create_dir_all(cache_path.parent().unwrap()).unwrap();
    cache_path.to_str().unwrap().to_string()
}

/// Verifies the filepath exists and is a file
pub fn cache_exists(filepath: &str) -> bool {
    Path::new(filepath).is_file()
}

/// Checks if a particular key in the cache should be invalidated
/// Used to invalidate the cache when a user passes the relevant CLI flag
pub fn cache_is_invalidated(key: &str) -> bool {
    let cli = get_cli();
    if cli.refresh {
        let mut cache = CACHE_RECHECKED.lock().unwrap();
        return *cache.entry(key.to_string()).or_insert(true);
    }
    false
}
