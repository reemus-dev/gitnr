mod cache;

use crate::template::collection::github::GITHUB_RAW;
use crate::template::collection::toptal::TOPTAL_API;
use crate::template::item::cache::TemplateCache;
use crate::util::http::http;
use crate::util::string::{strip_prefixes, strip_suffixes};
use anyhow::{bail, Context, Result};
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use url::Url;

/// The prefixes used to identify the template kind
pub struct TemplatePrefixes {
    pub url: &'static str,
    pub file: &'static str,
    pub github_repo: &'static str,
    pub github_community: &'static str,
    pub github_global: &'static str,
    pub github: &'static str,
    pub toptal: &'static str,
}

pub const PREFIXES: TemplatePrefixes = TemplatePrefixes {
    url: "url:",
    file: "file:",
    github_repo: "repo:",
    github_community: "ghc:",
    github_global: "ghg:",
    github: "gh:",
    toptal: "tt:",
};

/// The available predefined .gitignore template types
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TemplateValue {
    Url(String),
    File(String),
    GitHubRepo(String),
    GitHubGlobal(String),
    GitHubCommunity(String),
    GitHub(String),
    TopTal(String),
}

impl TemplateValue {
    /// Create a new template value from a string value
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            // Match with prefix
            _ if s.starts_with(PREFIXES.github) => Self::GitHub(s.to_string()),
            _ if s.starts_with(PREFIXES.github_global) => Self::GitHubGlobal(s.to_string()),
            _ if s.starts_with(PREFIXES.github_community) => Self::GitHubCommunity(s.to_string()),
            _ if s.starts_with(PREFIXES.toptal) => Self::TopTal(s.to_string()),
            _ if s.starts_with(PREFIXES.github_repo) => Self::GitHubRepo(s.to_string()),
            _ if s.starts_with(PREFIXES.url) => Self::Url(s.to_string()),
            _ if s.starts_with(PREFIXES.file) => Self::File(s.to_string()),
            // Match with best effort
            _ if Url::parse(s).is_ok() => Self::Url(s.to_string()),
            _ if Path::new(s).exists() => Self::File(s.to_string()),
            _ if s.matches('/').count() >= 3 => Self::GitHubRepo(s.to_string()),
            _ if s.to_lowercase().starts_with("community/") => Self::GitHubCommunity(s.to_string()),
            _ if s.to_lowercase().starts_with("global/") => Self::GitHubGlobal(s.to_string()),
            _ => Self::GitHub(s.to_string()),
        })
    }

    /// Return the template type prefix
    pub fn prefix(&self) -> &str {
        match self {
            Self::Url(_) => PREFIXES.url,
            Self::File(_) => PREFIXES.file,
            Self::GitHubRepo(_) => PREFIXES.github_repo,
            Self::GitHubCommunity(_) => PREFIXES.github_community,
            Self::GitHubGlobal(_) => PREFIXES.github_global,
            Self::GitHub(_) => PREFIXES.github,
            Self::TopTal(_) => PREFIXES.toptal,
        }
    }

    /// Returns the name of the template, could be a URL, file path of the name of the template in a collection
    pub(crate) fn name(&self) -> Result<String> {
        Ok(match self {
            Self::Url(url) => url.strip_prefix(self.prefix()).unwrap_or(url),
            Self::File(path) => path.strip_prefix(self.prefix()).unwrap_or(path),
            Self::GitHubRepo(repo) => repo.strip_prefix(self.prefix()).unwrap_or(repo),
            Self::GitHubGlobal(repo) => {
                let repo =
                    strip_prefixes(repo, &[self.prefix(), "global/", "Global/"]).unwrap_or(repo);
                repo.strip_suffix(".gitignore").unwrap_or(repo)
            }
            Self::GitHubCommunity(repo) => {
                let repo = strip_prefixes(repo, &[self.prefix(), "community/", "Community/"])
                    .unwrap_or(repo);
                repo.strip_suffix(".gitignore").unwrap_or(repo)
            }
            Self::GitHub(repo) => {
                let repo = repo.strip_prefix(self.prefix()).unwrap_or(repo);
                repo.strip_suffix(".gitignore").unwrap_or(repo)
            }
            Self::TopTal(name) => {
                let name = name.strip_prefix(self.prefix()).unwrap_or(name);
                strip_suffixes(name, &[".gitignore", ".patch", ".stack"]).unwrap_or(name)
            }
        }
        .to_string())
    }

    /// Returns the title of the template
    pub(crate) fn title(&self) -> Result<String> {
        let prefix = match self {
            TemplateValue::Url(_) => "URL",
            TemplateValue::File(_) => "File",
            TemplateValue::GitHubRepo(_) => "Repo",
            TemplateValue::GitHubCommunity(_) => "GitHub Community",
            TemplateValue::GitHubGlobal(_) => "GitHub Global",
            TemplateValue::GitHub(_) => "GitHub",
            TemplateValue::TopTal(_) => "TopTal",
        };
        Ok(format!("{}: {}", prefix, self.name()?))
    }

    /// Returns the URL used to fetch the template
    pub(crate) fn url(&self) -> Result<String> {
        match self {
            Self::Url(_) => {
                let url = self.name()?;
                let url = Url::parse(&url)
                    .with_context(|| format!("[Ignore Template] Invalid URL: {}", url))?;
                Ok(url.to_string())
            }
            Self::File(_) => {
                let path = self.name()?;
                if !Path::new(&path).exists() {
                    bail!(
                        "[Ignore Template] Invalid or non-existent file path: {}",
                        path
                    )
                }
                Ok(path)
            }
            Self::GitHubRepo(_) => {
                let repo = self.name()?;
                let url = format!("{}/{}", GITHUB_RAW, repo);
                Ok(url)
            }
            Self::GitHubGlobal(_) => {
                let repo = self.name()?;
                let url = format!(
                    "{}/github/gitignore/main/Global/{}.gitignore",
                    GITHUB_RAW, repo
                );
                Ok(url)
            }
            Self::GitHubCommunity(_) => {
                let repo = self.name()?;
                let url = format!(
                    "{}/github/gitignore/main/community/{}.gitignore",
                    GITHUB_RAW, repo
                );
                Ok(url)
            }
            Self::GitHub(_) => {
                let repo = self.name()?;
                let url = format!("{}/github/gitignore/main/{}.gitignore", GITHUB_RAW, repo);
                Ok(url)
            }
            Self::TopTal(_) => {
                let name = self.name()?;
                let url = format!("{}/{}", TOPTAL_API, name);
                Ok(url)
            }
        }
    }
}

/// Represents a .gitignore template created from an input string
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Template {
    /// The original input string
    pub input: String,
    /// The parsed template type / value
    pub value: TemplateValue,
}

impl Template {
    pub fn new(input: &str) -> Result<Self> {
        Ok(Self {
            input: input.to_string(),
            value: TemplateValue::from_str(input)?,
        })
    }

    pub fn new_as(input: &str, kind: TemplateValue) -> Result<Self> {
        Ok(Self {
            input: input.to_string(),
            value: kind,
        })
    }

    /// Returns the title section of the template content
    pub fn content_title(&self) -> Result<String> {
        let title = format!("###  {}  ###", self.value.title()?);
        let seperator = "-".repeat(title.len() - 4);
        Ok(formatdoc! {"
            ###{seperator}###
            {title}
            ###{seperator}###
        ", seperator = seperator, title = title})
    }

    /// Returns the body section of the template content
    pub fn content_body(&self) -> Result<String> {
        match &self.value {
            TemplateValue::File(_) => {
                let path = self.value.url()?;
                let content = fs::read_to_string(&path).with_context(|| {
                    format!("Failed to read ignore file template at path\n{}", path)
                })?;
                Ok(content)
            }
            _ => {
                let url = self.value.url()?;
                return match TemplateCache::get(&url)? {
                    Some(content) => Ok(content),
                    None => {
                        let content: String = http().get(&url)
                            .call()
                            .with_context(|| {
                                format!("Failed to fetch ignore template at URL. The template might not exist...\n{}", url)
                            })?
                            .into_string()
                            .with_context(|| {
                                format!("Failed to parse response when fetching ignore template at URL\n{}", url)
                            })?;
                        let content = content.trim();
                        TemplateCache::set(&url, content)?;
                        Ok(content.to_string())
                    }
                };
            }
        }
    }

    /// Returns the full content of the template, optionally overriding the body content
    pub fn content(&self, content: Option<&str>) -> Result<String> {
        let title = self.content_title()?;
        let content = match content {
            Some(content) => content.to_string(),
            None => self.content_body()?,
        };
        let full = formatdoc! {"
            {}
            {}
        ", title, content};
        Ok(full.trim().to_string())
    }
}
