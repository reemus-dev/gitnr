pub mod github;
pub mod toptal;

use crate::template::collection::github::GithubTemplates;
use crate::template::collection::toptal::TopTalTemplates;
use crate::template::item::Template;
use anyhow::Result;

/// The available predefined .gitignore template collections
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateCollectionKind {
    TopTal,
    GitHub,
    GitHubCommunity,
    GitHubGlobal,
}

impl TemplateCollectionKind {
    /// Display name of the template collection
    pub fn name(&self) -> &str {
        match self {
            TemplateCollectionKind::TopTal => "TopTal",
            TemplateCollectionKind::GitHub => "GitHub",
            TemplateCollectionKind::GitHubCommunity => "GitHub Community",
            TemplateCollectionKind::GitHubGlobal => "GitHub Global",
        }
    }
    /// Get a vector of the templates in the collection
    pub fn get(&self) -> Result<Vec<Template>> {
        Ok(match self {
            TemplateCollectionKind::TopTal => TopTalTemplates::templates()?,
            TemplateCollectionKind::GitHub => GithubTemplates::root()?,
            TemplateCollectionKind::GitHubCommunity => GithubTemplates::community()?,
            TemplateCollectionKind::GitHubGlobal => GithubTemplates::global()?,
        })
    }
}

/// A collection of .gitignore templates
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateCollection {
    pub kind: TemplateCollectionKind,
    pub items: Vec<Template>,
}

impl TemplateCollection {
    pub fn new(kind: TemplateCollectionKind) -> Result<Self> {
        let items = kind.get()?;
        Ok(Self { kind, items })
    }
}
