#![allow(dead_code)]

mod cli;
mod commands;
mod template;
mod tests;
mod util;

use crate::cli::{get_cli, Commands};
use crate::commands::completions;
use crate::commands::create;
use crate::commands::search;
use anyhow::Result;
use indoc::eprintdoc;
use yansi::Paint;

fn main() -> Result<()> {
    human_panic::setup_panic!();

    // Handle CLI command
    let result = match &get_cli().command {
        Some(Commands::Create(cmd)) => create::command(cmd),
        Some(Commands::Search) => search::command(),
        Some(Commands::Completions { shell }) => completions::command(shell),
        None => {
            use clap::CommandFactory;
            crate::cli::Cli::command().print_help()?;
            Ok(())
        }
    };

    // Handle error output and program termination
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintdoc! {"\n{title} {error}\n\n",
                title=" Error ".on_red().dim().white().bold(),
                error=format!("{:?}", e),
            }
            std::process::exit(1)
        }
    }
}
