use std::io;

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::cli::Cli;

pub fn command(shell: &Shell) -> Result<()> {
    let cmd = &mut Cli::command();

    generate(*shell, cmd, cmd.get_name().to_string(), &mut io::stdout());

    Ok(())
}
