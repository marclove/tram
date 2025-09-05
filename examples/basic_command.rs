//! Basic CLI command example
//!
//! This example demonstrates the fundamental pattern of integrating clap and starbase
//! in a Tram-based CLI application. It shows:
//! - Simple command structure with subcommands
//! - Session-based lifecycle management
//! - Configuration integration
//! - Error handling with miette

use async_trait::async_trait;
use clap::Parser;
use miette::Result;
use starbase::{App, AppSession};
use tracing::info;

/// Basic CLI demonstrating clap + starbase integration
#[derive(Parser, Debug)]
#[command(name = "basic-example")]
#[command(about = "A basic example of Tram CLI patterns")]
struct BasicCli {
    /// Global verbosity flag
    #[arg(short, long)]
    verbose: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: BasicCommand,
}

/// Available subcommands
#[derive(Parser, Debug)]
enum BasicCommand {
    /// Say hello to someone
    Greet {
        /// Name to greet
        name: String,
        /// Number of times to greet
        #[arg(short, long, default_value = "1")]
        count: u32,
    },
    /// Show current status
    Status {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Initialize something
    Init {
        /// Target directory
        #[arg(short, long, default_value = ".")]
        target: String,
        /// Force initialization
        #[arg(short, long)]
        force: bool,
    },
}

/// Basic application session
#[derive(Debug, Clone)]
struct BasicSession {
    verbose: bool,
}

impl BasicSession {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

#[async_trait]
impl AppSession for BasicSession {
    async fn startup(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Starting basic CLI application");
        }

        // Initialize any resources here (config, connections, etc.)
        Ok(None)
    }

    async fn analyze(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Analyzing environment");
        }

        // Validate environment, check prerequisites, etc.
        Ok(None)
    }

    async fn shutdown(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Shutting down gracefully");
        }

        // Clean up resources here
        Ok(None)
    }
}

/// Execute the parsed command
async fn execute_command(command: BasicCommand, session: &BasicSession) -> Result<()> {
    match command {
        BasicCommand::Greet { name, count } => {
            if session.verbose {
                info!("Greeting {} {} time(s)", name, count);
            }

            for i in 1..=count {
                if count > 1 {
                    println!("{}: Hello, {}!", i, name);
                } else {
                    println!("Hello, {}!", name);
                }
            }
        }

        BasicCommand::Status { detailed } => {
            println!("Status: Running");

            if detailed {
                println!("Version: 0.1.0");
                println!("Build: release");
                println!("Features: basic-commands");

                if session.verbose {
                    info!("Detailed status information displayed");
                }
            }
        }

        BasicCommand::Init { target, force } => {
            if session.verbose {
                info!("Initializing in directory: {}", target);
            }

            if target == "." {
                println!("Initializing in current directory");
            } else {
                println!("Initializing in directory: {}", target);
            }

            if force {
                println!("Force mode: overwriting existing files");
            }

            // Simulate initialization work
            println!("✓ Created configuration file");
            println!("✓ Set up directory structure");
            println!("✓ Initialization complete!");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = BasicCli::parse();

    // Create session with parsed options
    let mut session = BasicSession::new(cli.verbose);

    // Create starbase app
    let app = App::default();

    // Run the application with session lifecycle
    app.run_with_session(&mut session, |session| async move {
        // Execute the command
        execute_command(cli.command, &session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
