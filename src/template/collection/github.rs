use crate::template::item::Template;
use crate::util::fs::{
    cache_exists, cache_filepath, cache_is_invalidated, read_json_file, write_json_file,
};
use crate::util::http::http;
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const CACHE_PATH: &str = "collections/github.json";
pub const GITHUB_RAW: &str = "https://raw.githubusercontent.com";
pub const GITHUB_API_ENDPOINT: &str = "https://api.github.com";
pub const GITHUB_API_ACCEPT: &str = "application/vnd.github+json";

/// Static instance of GitHub templates to prevent unnecessary fetching or cache reads
static TEMPLATES: Lazy<Result<GithubTemplates>> = Lazy::new(GithubTemplates::new);

/// The different GitHub ignore template collections
#[derive(Debug, Deserialize, Serialize)]
pub struct GithubTemplates {
    /// When the templates were last updated
    updated: SystemTime,
    /// Root directory templates
    root: Vec<Template>,
    /// Global directory templates
    global: Vec<Template>,
    /// Community directory templates
    community: Vec<Template>,
}

impl GithubTemplates {
    /// Create GitHub templates list by Initializing from cache or fetching if necessary
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
                root: Vec::new(),
                global: Vec::new(),
                community: Vec::new(),
            }
        };

        // Refresh cache if it's older than 1 hour and save it to the cache file
        if _self.updated.elapsed()?.as_secs() > 60 * 60 {
            _self = Self::fetch()?;
            write_json_file(&cache_path, &_self)?;
        }

        Ok(_self)
    }

    /// Fetch the templates from the GitHub API
    fn fetch() -> Result<Self> {
        let mut root: Vec<Template> = Vec::new();
        let mut global: Vec<Template> = Vec::new();
        let mut community: Vec<Template> = Vec::new();

        // Get the repo tree and filter anything that's not a blob (file)
        let tree = gh_tree("github", "gitignore", "main", true)?;
        let tree = tree
            .tree
            .iter()
            .filter(|i| i.kind != TreeItemKind::Tree)
            .collect::<Vec<_>>();

        for item in tree.iter() {
            // Skip anything that's not a .gitignore file
            let path = match item.path.strip_suffix(".gitignore") {
                Some(path) => path,
                _ => continue,
            };

            // Root template
            if !path.contains('/') {
                let template = Template::new(&format!("gh:{}", path))?;
                root.push(template);
            }
            // Global template
            else if let Some(path) = path.strip_prefix("Global/") {
                let template = Template::new(&format!("ghg:{}", path))?;
                global.push(template);
            }
            // Community template
            else if let Some(path) = path.strip_prefix("community/") {
                let template = Template::new(&format!("ghc:{}", path))?;
                community.push(template);
            }
        }

        let updated = SystemTime::now();

        Ok(Self {
            updated,
            root,
            global,
            community,
        })
    }

    /// Get the root GitHub ignore templates
    pub fn root() -> Result<Vec<Template>> {
        match &*TEMPLATES {
            Ok(templates) => Ok(templates.root.clone()),
            Err(e) => Err(anyhow!(format!("{:?}", e))),
        }
    }

    /// Get the global GitHub ignore templates
    pub fn global() -> Result<Vec<Template>> {
        match &*TEMPLATES {
            Ok(templates) => Ok(templates.global.clone()),
            Err(e) => Err(anyhow!(format!("{:?}", e))),
        }
    }

    /// Get the community GitHub ignore templates
    pub fn community() -> Result<Vec<Template>> {
        match &*TEMPLATES {
            Ok(templates) => Ok(templates.community.clone()),
            Err(e) => Err(anyhow!(format!("{:?}", e))),
        }
    }
}

/// GitHub tree API response.
///
/// https://docs.github.com/en/rest/git/trees?apiVersion=2022-11-28#get-a-tree
#[derive(Debug, Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
    pub truncated: bool,
    pub tree: Vec<TreeItem>,
}

/// GitHub tree item type
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TreeItemKind {
    Tree,
    Blob,
}

/// GitHub tree item
#[derive(Debug, Deserialize)]
pub struct TreeItem {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub kind: TreeItemKind,
    pub sha: String,
    pub size: Option<i64>,
    pub url: String,
}

/// Get the GitHub tree for a repository
pub fn gh_tree(owner: &str, repo: &str, branch: &str, recursive: bool) -> Result<Tree> {
    let mut url = format!(
        "{}/repos/{owner}/{repo}/git/trees/{branch}",
        GITHUB_API_ENDPOINT
    );

    if recursive {
        url = format!("{}?recursive=true", url)
    }

    let res = http()
        .get(&url)
        .set("Accept", GITHUB_API_ACCEPT)
        .call()
        .with_context(|| format!("GitHub API error when fetching repo tree\n\n{}", url))?
        .into_string()
        .with_context(|| format!("Failed to parse GitHub API response to string\n\n{}", url))?;

    let parsed = serde_json::from_str(&res)
        .with_context(|| format!("Failed to parse GitHub API response to JSON\n\n{}", url))?;

    Ok(parsed)
}

/*
// ======================
// Branch API (not used)
//  https://docs.github.com/en/rest/branches/branches?apiVersion=2022-11-28#get-a-branch
// ======================

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
}

pub fn gh_branch(owner: &str, repo: &str, branch: &str) -> Result<Branch> {
    let url = format!(
        "{}/repos/{owner}/{repo}/branches/{branch}",
        GITHUB_API_ENDPOINT
    );

    let res = http()
        .get(&url)
        .set("Accept", GITHUB_API_ACCEPT)
        .call()
        .with_context(|| format!("GitHub API error when fetching repo branch\n\n{}", url))?
        .into_string()
        .with_context(|| format!("Failed to parse GitHub API response to string\n\n{}", url))?;

    serde_json::from_str(&res)
        .with_context(|| format!("Failed to parse GitHub API response to JSON\n\n{}", url))
}
*/
