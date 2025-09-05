//! Configuration usage example
//!
//! This example demonstrates how to use Tram's configuration system, including:
//! - Loading configuration from multiple sources (files, environment variables, CLI args)
//! - Configuration validation and defaults
//! - Hot reloading configuration during runtime
//! - Custom configuration structures
//! - Error handling for configuration issues

use async_trait::async_trait;
use clap::Parser;
use miette::Result;
use starbase::{App, AppSession};
use std::path::PathBuf;
use tracing::info;
use tram_config::{ConfigChangeHandler, ConfigWatcher, LogLevel, OutputFormat, TramConfig};

/// Configuration usage CLI example
#[derive(Parser, Debug)]
#[command(name = "config-example")]
#[command(about = "Demonstrates configuration management patterns")]
struct ConfigCli {
    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Override log level
    #[arg(long)]
    log_level: Option<String>,

    /// Override output format
    #[arg(long)]
    format: Option<String>,

    /// Disable colors
    #[arg(long)]
    no_color: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: ConfigCommand,
}

/// Available configuration commands
#[derive(Parser, Debug)]
enum ConfigCommand {
    /// Show current configuration
    Show {
        /// Show configuration sources
        #[arg(short, long)]
        sources: bool,
    },
    /// Validate configuration
    Validate {
        /// Configuration file to validate
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Watch configuration for changes
    Watch {
        /// Configuration file to watch
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Duration to watch in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,
    },
    /// Test configuration loading from different sources
    Test {
        /// Test loading from environment variables
        #[arg(long)]
        env: bool,
        /// Test loading from file
        #[arg(long)]
        file: bool,
        /// Test CLI overrides
        #[arg(long)]
        cli: bool,
    },
}

/// Session with configuration management
#[derive(Debug, Clone)]
struct ConfigSession {
    config: TramConfig,
}

impl ConfigSession {
    fn new(config: TramConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl AppSession for ConfigSession {
    async fn startup(&mut self) -> Result<Option<u8>, miette::Error> {
        info!(
            "Starting config example with log level: {}",
            self.config.log_level
        );

        // Configuration is already loaded and validated at this point
        Ok(None)
    }

    async fn analyze(&mut self) -> Result<Option<u8>, miette::Error> {
        info!("Configuration analysis complete");
        Ok(None)
    }

    async fn shutdown(&mut self) -> Result<Option<u8>, miette::Error> {
        info!("Shutting down config example");
        Ok(None)
    }
}

/// Custom config change handler for the example
struct ExampleConfigHandler;

#[async_trait::async_trait]
impl ConfigChangeHandler for ExampleConfigHandler {
    async fn handle_config_change(&self, new_config: &TramConfig) {
        println!("\n🔄 Configuration changed!");
        println!("   Log level: {}", new_config.log_level);
        println!("   Output format: {}", new_config.output_format);
        println!("   Colors enabled: {}", new_config.color);

        if let Some(workspace_root) = &new_config.workspace_root {
            println!("   Workspace root: {}", workspace_root.display());
        }
    }

    async fn handle_config_error(&self, error: Box<dyn std::error::Error + Send + Sync>) {
        println!("❌ Configuration error: {}", error);
    }
}

/// Show current configuration details
fn show_config(config: &TramConfig, show_sources: bool) {
    println!("Current Configuration:");
    println!("  Log Level: {}", config.log_level);
    println!("  Output Format: {}", config.output_format);
    println!("  Colors Enabled: {}", config.color);

    if let Some(workspace_root) = &config.workspace_root {
        println!("  Workspace Root: {}", workspace_root.display());
    }

    if show_sources {
        println!("\nConfiguration Sources:");
        println!("  1. Default values");
        println!("  2. Configuration files (.tram.json, .tram.yaml, .tram.toml)");
        println!("  3. Environment variables (TRAM_*)");
        println!("  4. Command line arguments (highest priority)");

        println!("\nConfiguration File Search Paths:");
        println!("  - Current directory: ./.tram.{{json,yaml,toml}}");
        println!("  - Home directory: ~/.config/tram/config.{{json,yaml,toml}}");
        println!("  - System directory: /etc/tram/config.{{json,yaml,toml}}");
    }
}

/// Validate a configuration file
async fn validate_config(file: Option<PathBuf>) -> Result<()> {
    match file {
        Some(path) => {
            println!("Validating configuration file: {}", path.display());

            match TramConfig::load_from_file(&path) {
                Ok(config) => {
                    println!("✓ Configuration is valid");
                    println!("  Log Level: {}", config.log_level);
                    println!("  Output Format: {}", config.output_format);
                    println!("  Colors: {}", config.color);
                }
                Err(e) => {
                    println!("✗ Configuration validation failed:");
                    println!("  Error: {}", e);
                    return Err(miette::miette!("Invalid configuration: {}", e));
                }
            }
        }
        None => {
            println!("Validating default configuration sources...");

            match TramConfig::load() {
                Ok(config) => {
                    println!("✓ Configuration loaded successfully");
                    show_config(&config, false);
                }
                Err(e) => {
                    println!("✗ Configuration validation failed:");
                    println!("  Error: {}", e);
                    return Err(miette::miette!("Invalid configuration: {}", e));
                }
            }
        }
    }

    Ok(())
}

/// Watch configuration for changes
async fn watch_config(file: Option<PathBuf>, duration: u64) -> Result<()> {
    println!("Starting configuration watcher for {} seconds...", duration);

    let config = match file {
        Some(ref path) => TramConfig::load_from_file(path)
            .map_err(|e| miette::miette!("Failed to load config: {}", e))?,
        None => TramConfig::load().map_err(|e| miette::miette!("Failed to load config: {}", e))?,
    };

    println!("Initial configuration loaded:");
    show_config(&config, false);

    // Set up config watcher
    let watcher = ConfigWatcher::new(config, file.map(|f| vec![f]))
        .await
        .map_err(|e| miette::miette!("Failed to create config watcher: {}", e))?;

    let handler = ExampleConfigHandler;
    watcher
        .start_with_handler(handler)
        .await
        .map_err(|e| miette::miette!("Failed to start config watcher: {}", e))?;

    println!("\nWatching for configuration changes...");
    println!("Try modifying the configuration file to see hot reload in action!");
    println!("Press Ctrl+C to stop watching.\n");

    // Wait for the specified duration or Ctrl+C
    let duration_future = tokio::time::sleep(std::time::Duration::from_secs(duration));
    let ctrl_c_future = tokio::signal::ctrl_c();

    tokio::select! {
        _ = duration_future => {
            println!("Watch duration expired");
        }
        _ = ctrl_c_future => {
            println!("Received interrupt signal, stopping watcher...");
        }
    }

    Ok(())
}

/// Test configuration loading from different sources
async fn test_config_sources(test_env: bool, test_file: bool, test_cli: bool) -> Result<()> {
    println!("Testing configuration loading from different sources:");

    if test_env {
        println!("\n📝 Testing environment variables:");
        println!("   Set TRAM_LOG_LEVEL=debug to override log level");
        println!("   Set TRAM_OUTPUT_FORMAT=json to override format");

        // Show current environment variable values
        if let Ok(log_level) = std::env::var("TRAM_LOG_LEVEL") {
            println!("   ✓ Found TRAM_LOG_LEVEL={}", log_level);
        } else {
            println!("   - TRAM_LOG_LEVEL not set");
        }

        if let Ok(format) = std::env::var("TRAM_OUTPUT_FORMAT") {
            println!("   ✓ Found TRAM_OUTPUT_FORMAT={}", format);
        } else {
            println!("   - TRAM_OUTPUT_FORMAT not set");
        }
    }

    if test_file {
        println!("\n📁 Testing file-based configuration:");

        // Try to load from common paths
        let common_paths = [".tram.json", ".tram.yaml", ".tram.toml", "config/tram.json"];

        for path in &common_paths {
            if std::path::Path::new(path).exists() {
                println!("   ✓ Found config file: {}", path);
                match TramConfig::load_from_file(PathBuf::from(path)) {
                    Ok(_) => println!("     - Successfully loaded"),
                    Err(e) => println!("     - Load error: {}", e),
                }
            } else {
                println!("   - Config file not found: {}", path);
            }
        }
    }

    if test_cli {
        println!("\n🖥️ Testing CLI argument overrides:");
        println!("   CLI arguments have the highest precedence");
        println!("   Example: --log-level debug --format json --no-color");

        // Load final configuration
        let config =
            TramConfig::load().map_err(|e| miette::miette!("Failed to load config: {}", e))?;
        println!("\n   Final merged configuration:");
        show_config(&config, false);
    }

    Ok(())
}

/// Execute the parsed configuration command
async fn execute_command(command: ConfigCommand, session: &mut ConfigSession) -> Result<()> {
    match command {
        ConfigCommand::Show { sources } => {
            show_config(&session.config, sources);
        }

        ConfigCommand::Validate { file } => {
            validate_config(file).await?;
        }

        ConfigCommand::Watch { file, duration } => {
            watch_config(file, duration).await?;
        }

        ConfigCommand::Test { env, file, cli } => {
            // If no specific tests requested, run all
            let test_all = !env && !file && !cli;

            test_config_sources(env || test_all, file || test_all, cli || test_all).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = ConfigCli::parse();

    // Load configuration with CLI overrides
    let mut config = if let Some(config_path) = &cli.config {
        TramConfig::load_from_file(config_path)
    } else {
        TramConfig::load()
    }
    .map_err(|e| miette::miette!("Configuration error: {}", e))?;

    // Apply CLI overrides
    if let Some(log_level) = &cli.log_level {
        config.log_level = match log_level.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => return Err(miette::miette!("Invalid log level: {}", log_level)),
        };
    }

    if let Some(format) = &cli.format {
        config.output_format = match format.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            "table" => OutputFormat::Table,
            _ => return Err(miette::miette!("Invalid output format: {}", format)),
        };
    }

    if cli.no_color {
        config.color = false;
    }

    // Create session with configuration
    let mut session = ConfigSession::new(config);

    // Create starbase app
    let app = App::default();

    // Run the application with session lifecycle
    app.run_with_session(&mut session, |mut session| async move {
        // Execute the configuration command
        execute_command(cli.command, &mut session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
