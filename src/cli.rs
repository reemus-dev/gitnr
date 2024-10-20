use crate::template::item::Template;
use crate::template::list::TemplateList;
use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use once_cell::sync::Lazy;

const LONG_ABOUT: &str = r"

――――――――――――――――――――――――――――――――――――――――――――
                  gitnr
――――――――――――――――――――――――――――――――――――――――――――

Generate a '.gitignore' file using one or more templates from 
the GitHub & TopTal collections along with your own templates 
from local files or remote URLs. 

You can also browse the available templates at the GitHub & 
TopTal collections using the `search` command.";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    /// Refresh the cache (templates are cached for 1h)
    #[arg(short = 'r', long = "refresh", global = true)]
    pub refresh: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Args, Debug)]
pub struct CommandCreate {
    /// Write template to .gitignore file in current directory
    #[arg(short = 's', long = "save")]
    pub out_gitignore: bool,
    /// Write template to the specified file path
    #[arg(short = 'f', long = "file")]
    pub out_file: Option<String>,
    /// Space or comma separated list of templates to use. Templates can be prefixed with
    /// the provider name to avoid any ambiguity.
    ///
    /// Providers:
    ///  - "gh:"    GitHub templates
    ///  - "ghc:"   GitHub community templates
    ///  - "ghg:"   GitHub global templates
    ///  - "tt:"    TopTal templates
    ///  - "url:"   Remote URL to text file template
    ///  - "file:"  Local file path to a .gitignore file
    ///  - "repo:"  File from a any public GitHub repo
    ///
    /// If no prefix is specified, program will attempt to guess the provider if possible
    /// otherwise it will default to a GitHub template. The template name is case-sensitive.
    /// Meaning "Rust" is not the same as "rust". The order in which the templates are provided
    /// is the order in output content will be.
    ///
    /// Examples:
    ///  - gitnr create Rust
    ///  - gitnr create gh:Rust
    ///  - gitnr create gh:Rust tt:jetbrains+all
    #[arg(verbatim_doc_comment)]
    pub templates: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a .gitignore file from one or more templates
    Create(CommandCreate),
    /// Choose templates interactively from the GitHub & TopTal collections
    Search,
}

impl Cli {
    /// Get ignore templates passed in from the CLI arguments
    pub fn templates(&self) -> Result<TemplateList> {
        let templates = match &self.command {
            Some(Commands::Create(args)) => &args.templates,
            Some(Commands::Search) => {
                bail!("Cannot provide template arguments to 'search' command")
            }
            None => bail!("Cannot provide template arguments to an unknown command"),
        };

        // Split templates seperated by commas and spaces
        let templates = templates
            .iter()
            .flat_map(|f| f.split(',').map(|s| s.to_string()))
            .collect::<Vec<String>>();

        // Create the template structs from the templates provided
        let templates = templates
            .iter()
            .map(|name| Template::new(name))
            .collect::<Result<Vec<Template>>>()
            .with_context(|| "Failed to parse provided template arguments".to_string())?;

        Ok(TemplateList::new(templates))
    }
}

static CLI: Lazy<Cli> = Lazy::new(Cli::parse);

pub fn get_cli() -> &'static Cli {
    &CLI
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
