//! Tram CLI starter kit demonstration binary.
//!
//! This demonstrates proper integration of clap and starbase without
//! unnecessary abstractions.

use async_trait::async_trait;
use clap::Parser;
use miette::Result;
use starbase::{App, AppSession};
use tram_config::Config;
use tram_workspace::{ProjectType, WorkspaceDetector};

/// CLI structure demonstrating clap + starbase patterns.
#[derive(Parser, Debug)]
#[command(name = "tram")]
#[command(about = "A batteries-included starter kit for building CLI applications in Rust")]
pub struct Cli {
    /// Global config options
    #[command(flatten)]
    pub global: GlobalOptions,

    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

/// Global CLI options that apply to all commands.
#[derive(Parser, Debug)]
pub struct GlobalOptions {
    /// Log level (debug, info, warn, error)
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Output format (json, yaml, table)
    #[arg(long, default_value = "table")]
    pub format: String,

    /// Disable colored output
    #[arg(long, default_value = "false")]
    pub no_color: bool,

    /// Config file path
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,
}

/// Available CLI commands.
#[derive(Parser, Debug)]
pub enum Commands {
    /// Initialize a new project
    Init {
        /// Project name
        name: String,
        /// Use verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show workspace information
    Workspace {
        /// Show detailed project information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Show configuration information
    Config,
}

/// Application session - directly implements starbase's AppSession.
#[derive(Clone, Debug)]
pub struct TramSession {
    pub config: Config,
    pub workspace: WorkspaceDetector,
    pub workspace_root: Option<std::path::PathBuf>,
    pub project_type: Option<ProjectType>,
}

impl TramSession {
    pub fn new() -> tram_core::AppResult<Self> {
        Ok(Self {
            config: Config::default(),
            workspace: WorkspaceDetector::new()?,
            workspace_root: None,
            project_type: None,
        })
    }
}

#[async_trait]
impl AppSession for TramSession {
    async fn startup(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Load configuration - in a real app, you'd pass CLI args here
        self.config = Config::load_from_args(&GlobalOptions {
            log_level: "info".to_string(),
            format: "table".to_string(),
            no_color: false,
            config: None,
        })?;
        self.config.validate()?;

        // Detect workspace
        if let Ok(root) = self.workspace.detect_root() {
            self.workspace_root = Some(root.clone());
            self.project_type = ProjectType::detect(&root);
        }

        Ok(None)
    }

    async fn analyze(&mut self) -> tram_core::AppResult<Option<u8>> {
        // This phase would typically validate the environment,
        // check dependencies, build task graphs, etc.

        if let Some(root) = &self.workspace_root {
            println!("Working in {} workspace", root.display());

            if let Some(project_type) = &self.project_type {
                println!("Detected {:?} project", project_type);
            }
        }

        Ok(None)
    }

    async fn shutdown(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Cleanup - save caches, write state, etc.
        println!("Done!");
        Ok(None)
    }
}

/// Execute a CLI command with the session.
async fn execute_command(command: Commands, session: &TramSession) -> tram_core::AppResult<()> {
    match command {
        Commands::Init { name, verbose } => {
            println!("🚀 Initializing project: {}", name);

            if verbose {
                println!("Verbose mode enabled");
                if let Some(root) = &session.workspace_root {
                    println!("Workspace root: {}", root.display());
                }
                println!("Config: {:?}", session.config);
            }

            // In a real implementation, you'd create project files here
            println!("Project '{}' initialized!", name);
        }

        Commands::Workspace { detailed } => {
            if let Some(root) = &session.workspace_root {
                println!("Workspace root: {}", root.display());

                if let Some(project_type) = &session.project_type {
                    println!("Project type: {:?}", project_type);

                    if detailed {
                        println!("Ignore patterns: {:?}", project_type.ignore_patterns());
                    }
                }
            } else {
                return Err(tram_core::TramError::WorkspaceNotFound.into());
            }
        }

        Commands::Config => {
            println!("Current configuration:");
            println!("   Log level: {}", session.config.log_level);
            println!("   Output format: {}", session.config.output_format);
            println!("   Colors: {}", session.config.color);

            if let Some(config_file) = &session.config.config_file {
                println!("   Config file: {}", config_file.display());
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Create application session
    let mut session = TramSession::new()?;

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
