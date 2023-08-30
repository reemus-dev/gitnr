use crate::cli::{get_cli, CommandCreate};
use anyhow::{bail, Context, Result};
use indoc::{formatdoc, printdoc};
use std::path::PathBuf;
use std::{env, fs};
use yansi::Paint;

pub fn command(cmd: &CommandCreate) -> Result<()> {
    let cli = get_cli();
    let templates = cli.templates()?;

    if templates.is_empty() {
        bail!(formatdoc! {"
            No template arguments provided
            
            Provide templates to the create command using the following syntax:
             gitnr create [TEMPLATE]...

            For more information, see the help:
             gitnr create --help"})
    }

    let output = templates.content()?;

    // Write template to .gitignore file in current directory
    if cmd.out_gitignore {
        let cwd = env::current_dir().with_context(|| "Failed to get current directory")?;
        let path = cwd.join(".gitignore");
        fs::write(&path, output + "\n").with_context(|| {
            format!(
                "Failed to write template to .gitignore file at path\n{}",
                path.to_str().unwrap_or("...unknown path")
            )
        })?;

        success_msg(path);
        return Ok(());
    }

    // Write template to file path
    if let Some(path) = &cmd.out_file {
        let path = PathBuf::from(path);
        let path = if path.is_relative() {
            path.canonicalize().with_context(|| {
                format!(
                    "Failed to get absolute path for relative path\n{}",
                    path.to_str().unwrap_or("...unknown path")
                )
            })?
        } else {
            path
        };

        if path.is_dir() {
            bail!(
                "The output path provided is a directory.\n\
                Provide a file path to write the template to a file.\n\
                Path: {}",
                path.to_str().unwrap_or("...unknown path")
            )
        }

        fs::write(&path, output + "\n").with_context(|| {
            format!(
                "Failed to write template to file at path\n{}",
                path.to_str().unwrap_or("...unknown path")
            )
        })?;

        success_msg(path);
        return Ok(());
    }

    // Print template to stdout
    println!("{}", output);

    Ok(())
}

fn success_msg(path: PathBuf) {
    printdoc! {"\n{title} {path}\n\n",
        title=" Success ".on_green().dim().white().bold(),
        path=format!("Template written to path: {}", path.to_str().unwrap_or("...unknown path")),
    }
}
