//! Developer tools for shell completions and manual page generation.
//!
//! This module provides functionality for generating shell completion scripts
//! and manual pages, which are essential for CLI tool distribution and usability.

use clap::CommandFactory;
use clap_complete::{generate, shells::Shell};
use clap_mangen::Man;
use std::io;

use crate::cli::Cli;

/// Generate shell completions to stdout
pub fn generate_completions(shell: Shell) -> tram_core::AppResult<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
    println!();

    // Print installation instructions
    match shell {
        Shell::Bash => {
            println!("# To install bash completions, add this to your ~/.bashrc:");
            println!("# eval \"$(tram completions bash)\"");
            println!("# Or save to a file:");
            println!("# tram completions bash > ~/.bash_completion.d/tram");
        }
        Shell::Zsh => {
            println!("# To install zsh completions, add this to your ~/.zshrc:");
            println!("# eval \"$(tram completions zsh)\"");
            println!("# Or save to a file in your fpath:");
            println!("# tram completions zsh > ~/.zsh/completions/_tram");
        }
        Shell::Fish => {
            println!("# To install fish completions:");
            println!("# tram completions fish > ~/.config/fish/completions/tram.fish");
        }
        Shell::PowerShell => {
            println!("# To install PowerShell completions, add this to your $PROFILE:");
            println!("# Invoke-Expression (& tram completions powershell)");
        }
        _ => {}
    }

    Ok(())
}

/// Generate manual pages
pub fn generate_man_pages(
    output_dir: &std::path::Path,
    section: Option<u8>,
) -> tram_core::AppResult<()> {
    use std::fs;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir).map_err(|e| tram_core::TramError::InvalidConfig {
        message: format!("Failed to create output directory: {}", e),
    })?;

    let cmd = Cli::command();
    let app_name = "tram";

    // Generate main command man page (section 1)
    if section.is_none() || section == Some(1) {
        let man = Man::new(cmd.clone());
        let mut buffer = Vec::new();
        man.render(&mut buffer)
            .map_err(|e| tram_core::TramError::InvalidConfig {
                message: format!("Failed to generate man page: {}", e),
            })?;

        let man_file = output_dir.join(format!("{}.1", app_name));
        fs::write(&man_file, buffer).map_err(|e| tram_core::TramError::InvalidConfig {
            message: format!("Failed to write man page: {}", e),
        })?;

        println!("Generated man page: {}", man_file.display());
    }

    // Generate subcommand man pages
    for subcommand in cmd.get_subcommands() {
        let subcommand_name = subcommand.get_name();

        if section.is_none() || section == Some(1) {
            let man = Man::new(subcommand.clone())
                .title(format!("{}-{}", app_name, subcommand_name))
                .section("1")
                .source(format!("{} {}", app_name, env!("CARGO_PKG_VERSION")))
                .manual("User Commands");

            let mut buffer = Vec::new();
            man.render(&mut buffer)
                .map_err(|e| tram_core::TramError::InvalidConfig {
                    message: format!("Failed to generate subcommand man page: {}", e),
                })?;

            let man_file = output_dir.join(format!("{}-{}.1", app_name, subcommand_name));
            fs::write(&man_file, buffer).map_err(|e| tram_core::TramError::InvalidConfig {
                message: format!("Failed to write subcommand man page: {}", e),
            })?;

            println!("Generated man page: {}", man_file.display());
        }
    }

    println!();
    println!("Manual pages generated in: {}", output_dir.display());
    println!();
    println!("To install system-wide:");
    println!(
        "  sudo cp {}/*.1 /usr/local/share/man/man1/",
        output_dir.display()
    );
    println!("  sudo mandb  # Update man database");
    println!();
    println!("To view locally:");
    println!("  man -M {} tram", output_dir.display());
    println!("  man -M {} tram-new", output_dir.display());

    Ok(())
}
