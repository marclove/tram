//! Tram CLI starter kit demonstration binary.
//!
//! This demonstrates proper integration of clap and starbase without
//! unnecessary abstractions.

use clap::Parser;
use miette::Result;
use starbase::App;
use tracing::debug;
use tram_config::{OutputFormat, TramConfig};

mod cli;
mod commands;
mod dev_tools;
mod examples;
mod session;
mod utils;

use cli::Cli;
use commands::execute_command;
use session::TramSession;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Debug CLI arguments
    debug!("CLI log_level: {}", cli.global.log_level);
    debug!("CLI format: {}", cli.global.format);
    debug!("CLI no_color: {}", cli.global.no_color);

    // Load base configuration using the methods we wrote in tram-config
    let mut config = if let Some(config_path) = &cli.global.config {
        TramConfig::load_from_file(config_path)
    } else {
        TramConfig::load_from_common_paths()
    }
    .map_err(|e| miette::miette!("Configuration error: {}", e))?;

    // Config loaded successfully

    // Apply CLI overrides directly to the config struct (highest precedence)
    if cli.global.log_level != "info" {
        match cli.global.log_level.to_lowercase().as_str() {
            "debug" => config.log_level = tram_config::LogLevel::Debug,
            "info" => config.log_level = tram_config::LogLevel::Info,
            "warn" => config.log_level = tram_config::LogLevel::Warn,
            "error" => config.log_level = tram_config::LogLevel::Error,
            _ => {
                return Err(miette::miette!(
                    "Invalid log level: {}",
                    cli.global.log_level
                ));
            }
        }
    }

    if cli.global.format != "table" {
        match cli.global.format.to_lowercase().as_str() {
            "json" => config.output_format = OutputFormat::Json,
            "yaml" => config.output_format = OutputFormat::Yaml,
            "table" => config.output_format = OutputFormat::Table,
            _ => {
                return Err(miette::miette!(
                    "Invalid output format: {}",
                    cli.global.format
                ));
            }
        }
    }

    if cli.global.no_color {
        config.color = false;
    }

    // Create application session with config
    let mut session = TramSession::with_config(config)?;

    // Create starbase app and run it with our session
    let app = App::default();

    app.run_with_session(&mut session, |session| async move {
        // Execute the command
        execute_command(cli.command, &session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
