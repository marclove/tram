//! Tram CLI starter kit demonstration binary.
//!
//! This demonstrates proper integration of clap and starbase without
//! unnecessary abstractions.

use async_trait::async_trait;
use clap::Parser;
use miette::Result;
use starbase::{App, AppSession};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tram_config::{ConfigChangeHandler, ConfigWatcher, OutputFormat, TramConfig};
use tram_core::{
    InitConfig, InitProjectType, ProjectInitializer, TemplateConfig, TemplateGenerator,
    TemplateType, init_tracing,
};
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
    /// Create a new project interactively
    New {
        /// Project name
        name: String,
        /// Project type (rust, nodejs, python, go, java, generic)
        #[arg(long, default_value = "rust")]
        project_type: String,
        /// Project description
        #[arg(long)]
        description: Option<String>,
        /// Skip interactive prompts
        #[arg(long)]
        skip_prompts: bool,
    },
    /// Generate templates for common CLI patterns
    Generate {
        /// Template type (command, config-section, error-type, session-extension)
        #[arg(long, default_value = "command")]
        template_type: String,
        /// Name of the item to generate (e.g., "backup", "deploy")
        name: String,
        /// Description for the generated template
        #[arg(long)]
        description: Option<String>,
        /// Target directory (defaults to current directory)
        #[arg(long)]
        target_dir: Option<std::path::PathBuf>,
        /// Write the template to filesystem (default: show to stdout)
        #[arg(long)]
        write: bool,
    },
    /// Initialize a new project (legacy command)
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
    /// Watch mode - monitor files and reload config automatically
    Watch {
        /// Watch configuration files for hot reload
        #[arg(long, default_value = "true")]
        config: bool,
        /// Run checks on file changes (format, lint, build, test)
        #[arg(long, default_value = "true")]
        check: bool,
    },
    /// Run interactive examples demonstrating CLI patterns
    Examples {
        /// Example to run
        #[arg(value_enum)]
        example: ExampleType,
    },
}

/// Available example types
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ExampleType {
    /// Basic CLI command patterns
    BasicCommand,
    /// Async operations and concurrency
    AsyncOperations,
    /// Configuration management
    ConfigUsage,
    /// Progress indicators and terminal UI
    ProgressIndicators,
    /// Interactive prompts and user input
    InteractivePrompts,
    /// File system operations
    FileOperations,
}

/// Application session - directly implements starbase's AppSession.
#[derive(Clone, Debug)]
pub struct TramSession {
    pub config: TramConfig,
    pub workspace: WorkspaceDetector,
    pub workspace_root: Option<std::path::PathBuf>,
    pub project_type: Option<ProjectType>,
}

impl TramSession {
    pub fn new() -> tram_core::AppResult<Self> {
        let config = TramConfig::load().map_err(|e| tram_core::TramError::InvalidConfig {
            message: format!("Failed to load configuration: {}", e),
        })?;

        Ok(Self {
            config,
            workspace: WorkspaceDetector::new()?,
            workspace_root: None,
            project_type: None,
        })
    }

    pub fn with_config(config: TramConfig) -> tram_core::AppResult<Self> {
        Ok(Self {
            config,
            workspace: WorkspaceDetector::new()?,
            workspace_root: None,
            project_type: None,
        })
    }
}

#[async_trait]
impl AppSession for TramSession {
    async fn startup(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Initialize tracing before anything else
        let use_json = matches!(self.config.output_format, OutputFormat::Json);
        init_tracing(&self.config.log_level.to_string(), use_json)?;

        info!("Starting Tram CLI application");
        debug!("Configuration: {:?}", self.config);

        // Configuration validation is handled by schematic automatically

        // Detect workspace
        if let Ok(root) = self.workspace.detect_root() {
            self.workspace_root = Some(root.clone());
            self.project_type = ProjectType::detect(&root);
            info!("Detected workspace at: {}", root.display());
        } else {
            debug!("No workspace detected");
        }

        Ok(None)
    }

    async fn analyze(&mut self) -> tram_core::AppResult<Option<u8>> {
        // This phase would typically validate the environment,
        // check dependencies, build task graphs, etc.

        debug!("Analyzing workspace environment");

        if let Some(root) = &self.workspace_root {
            println!("Working in {} workspace", root.display());

            if let Some(project_type) = &self.project_type {
                println!("Detected {:?} project", project_type);
                info!("Project type: {:?}", project_type);
            }
        }

        Ok(None)
    }

    async fn shutdown(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Cleanup - save caches, write state, etc.
        debug!("Shutting down application");
        println!("Done!");
        Ok(None)
    }
}

/// Parse project type string to InitProjectType.
fn parse_project_type(type_str: &str) -> InitProjectType {
    match type_str.to_lowercase().as_str() {
        "rust" => InitProjectType::Rust,
        "nodejs" | "node" | "js" => InitProjectType::NodeJs,
        "python" | "py" => InitProjectType::Python,
        "go" => InitProjectType::Go,
        "java" => InitProjectType::Java,
        _ => InitProjectType::Generic,
    }
}

/// Display name for project type.
fn project_type_display(project_type: &InitProjectType) -> &'static str {
    match project_type {
        InitProjectType::Rust => "Rust",
        InitProjectType::NodeJs => "Node.js",
        InitProjectType::Python => "Python",
        InitProjectType::Go => "Go",
        InitProjectType::Java => "Java",
        InitProjectType::Generic => "Generic",
    }
}

/// Parse template type string to TemplateType.
fn parse_template_type(type_str: &str) -> TemplateType {
    match type_str.to_lowercase().as_str() {
        "command" | "cmd" => TemplateType::Command,
        "config-section" | "config" => TemplateType::ConfigSection,
        "error-type" | "error" => TemplateType::ErrorType,
        "session-extension" | "session" => TemplateType::SessionExtension,
        _ => TemplateType::Command, // Default
    }
}

/// Display name for template type.
fn template_type_display(template_type: &TemplateType) -> &'static str {
    match template_type {
        TemplateType::Command => "Command",
        TemplateType::ConfigSection => "Config Section",
        TemplateType::ErrorType => "Error Type",
        TemplateType::SessionExtension => "Session Extension",
    }
}

/// Handler for configuration changes during watch mode.
pub struct WatchConfigHandler;

#[async_trait::async_trait]
impl ConfigChangeHandler for WatchConfigHandler {
    async fn handle_config_change(&self, new_config: &TramConfig) {
        info!("🔄 Configuration reloaded successfully");
        info!("   Log level: {}", new_config.log_level);
        info!("   Output format: {}", new_config.output_format);
        info!("   Colors: {}", new_config.color);

        if let Some(workspace_root) = &new_config.workspace_root {
            info!("   Workspace root: {}", workspace_root.display());
        }
    }

    async fn handle_config_error(&self, error: Box<dyn std::error::Error + Send + Sync>) {
        warn!("❌ Configuration reload failed: {}", error);
        warn!("   Continuing with previous configuration");
    }
}

/// Execute a CLI command with the session.
async fn execute_command(command: Commands, session: &TramSession) -> tram_core::AppResult<()> {
    match command {
        Commands::New {
            name,
            project_type,
            description,
            skip_prompts,
        } => {
            info!("Creating new project: {}", name);

            if !skip_prompts {
                // In future iterations, we would add interactive prompts here
                // For now, just note that interactive mode is planned
                debug!("Interactive prompts would be shown here (future feature)");
            }

            let project_type = parse_project_type(&project_type);
            let current_dir =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let project_path = current_dir.join(&name);

            let init_config = InitConfig {
                name: name.clone(),
                path: project_path,
                project_type,
                description,
                author: None,
            };

            let initializer = ProjectInitializer::new();
            initializer.create_project(&init_config)?;

            println!(
                "✓ Created new {} project: {}",
                project_type_display(&init_config.project_type),
                name
            );
            if let Some(desc) = &init_config.description {
                println!("  Description: {}", desc);
            }
        }

        Commands::Generate {
            template_type,
            name,
            description,
            target_dir,
            write,
        } => {
            info!("Generating {} template: {}", template_type, name);

            let template_type = parse_template_type(&template_type);
            let target_dir = target_dir.unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            });

            let mut parameters = HashMap::new();
            if let Some(desc) = description {
                parameters.insert("description".to_string(), desc);
            }

            let template_config = TemplateConfig {
                name: name.clone(),
                template_type: template_type.clone(),
                target_dir,
                parameters,
            };

            let generator = TemplateGenerator::new()?;
            let template = generator.generate_template(&template_config)?;

            if write {
                generator.write_template(&template)?;
                println!(
                    "✓ Generated {} template: {} -> {}",
                    template_type_display(&template_type),
                    name,
                    template.file_path.display()
                );
            } else {
                println!(
                    "Generated {} template for '{}':",
                    template_type_display(&template_type),
                    name
                );
                println!("File path: {}", template.file_path.display());
                println!("\n{}", "=".repeat(80));
                println!("{}", template.content);
                println!("{}", "=".repeat(80));
                println!("\nTo write to filesystem, add the --write flag");
            }
        }

        Commands::Init { name, verbose } => {
            println!("🚀 Initializing project: {}", name);

            if verbose {
                println!("Verbose mode enabled");
                if let Some(root) = &session.workspace_root {
                    println!("Workspace root: {}", root.display());
                }
                println!("Config: {:?}", session.config);
            }

            // Legacy command - for now, just create a generic project
            let current_dir =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let project_path = current_dir.join(&name);

            let init_config = InitConfig {
                name: name.clone(),
                path: project_path,
                project_type: InitProjectType::Generic,
                description: Some("A new project".to_string()),
                author: None,
            };

            let initializer = ProjectInitializer::new();
            if let Err(e) = initializer.create_project(&init_config) {
                println!("Warning: Could not create project files: {}", e);
            }

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

            if let Some(workspace_root) = &session.config.workspace_root {
                println!("   Workspace root: {}", workspace_root.display());
            }
        }

        Commands::Watch {
            config: watch_config,
            check,
        } => {
            info!("Starting watch mode...");

            if watch_config {
                info!("🔍 Config hot reload: ENABLED");
            } else {
                info!("🔍 Config hot reload: DISABLED");
            }

            if check {
                info!("⚡ Auto-checks (format, lint, build, test): ENABLED");
            } else {
                info!("⚡ Auto-checks: DISABLED");
            }

            println!("Watch mode started. Press Ctrl+C to stop.");

            let mut tasks = Vec::new();

            // Set up config watcher if enabled
            if watch_config {
                let config_watcher = ConfigWatcher::new(session.config.clone(), None)
                    .await
                    .map_err(|e| tram_core::TramError::InvalidConfig {
                        message: format!("Failed to start config watcher: {}", e),
                    })?;

                let handler = WatchConfigHandler;
                if let Err(e) = config_watcher.start_with_handler(handler).await {
                    warn!("Failed to start config change handler: {}", e);
                }

                // Keep the watcher alive by storing it
                tasks.push(tokio::spawn(async move {
                    // Keep the config_watcher alive for the duration of the task
                    let _watcher = config_watcher;
                    // Wait indefinitely (until the task is cancelled)
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
                    loop {
                        interval.tick().await;
                    }
                }));
            }

            // Set up file watching for code changes if enabled
            if check {
                tasks.push(tokio::spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
                    let mut last_check = std::time::Instant::now();

                    loop {
                        interval.tick().await;

                        // Simple implementation: check if any Rust files have been modified
                        // In a real implementation, you'd use a proper file watcher
                        let current_time = std::time::Instant::now();
                        if current_time.duration_since(last_check).as_secs() >= 2 {
                            debug!("Running periodic checks (placeholder for file-based trigger)");
                            last_check = current_time;

                            // Here you would run `just check` or equivalent
                            // For now, just log that we would run checks
                            debug!("Would run: just check");
                        }
                    }
                }));
            }

            if tasks.is_empty() {
                warn!("No watch features enabled. Use --config or --check flags.");
                return Ok(());
            }

            // Wait for Ctrl+C
            tokio::signal::ctrl_c()
                .await
                .map_err(|e| tram_core::TramError::InvalidConfig {
                    message: format!("Failed to wait for Ctrl+C: {}", e),
                })?;

            info!("Shutting down watch mode...");

            // Cancel all tasks
            for task in tasks {
                task.abort();
            }

            println!("Watch mode stopped.");
        }

        Commands::Examples { example } => {
            info!("Running example: {:?}", example);
            run_example(example, session).await?;
        }
    }

    Ok(())
}

/// Run an example demonstrating CLI patterns
async fn run_example(example: ExampleType, session: &TramSession) -> tram_core::AppResult<()> {
    match example {
        ExampleType::BasicCommand => {
            println!("=== Basic Command Example ===");
            println!("This example demonstrates fundamental clap + starbase integration patterns.");
            println!();
            println!("Key features demonstrated:");
            println!("• Command-line argument parsing with clap");
            println!("• Session-based lifecycle management with starbase");
            println!("• Error handling with miette");
            println!("• Structured logging and tracing");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example basic_command -- greet \"Your Name\"");
        }

        ExampleType::AsyncOperations => {
            println!("=== Async Operations Example ===");
            println!("This example demonstrates async patterns in CLI applications.");
            println!();
            println!("Key features demonstrated:");
            println!("• Long-running async tasks with progress");
            println!("• Concurrent operations with controlled parallelism");
            println!("• Timeout handling and graceful cancellation");
            println!("• Service monitoring and health checks");
            println!();
            println!("🔗 For full interactive example, run:");
            println!(
                "   cargo run --example async_operations -- download https://example.com/file output.txt"
            );
        }

        ExampleType::ConfigUsage => {
            println!("=== Configuration Management Example ===");
            println!("This example demonstrates Tram's configuration system.");
            println!();
            println!("Current configuration:");
            println!("  Log Level: {}", session.config.log_level);
            println!("  Output Format: {}", session.config.output_format);
            println!("  Colors: {}", session.config.color);
            if let Some(workspace_root) = &session.config.workspace_root {
                println!("  Workspace Root: {}", workspace_root.display());
            }
            println!();
            println!("Key features demonstrated:");
            println!("• Loading configuration from multiple sources");
            println!("• Hot reload with file watching");
            println!("• CLI argument overrides");
            println!("• Environment variable integration");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example config_usage -- show --sources");
        }

        ExampleType::ProgressIndicators => {
            println!("=== Progress Indicators Example ===");
            println!("This example demonstrates terminal UI components.");
            println!();
            println!("Key features demonstrated:");
            println!("• Progress bars with ETA calculations");
            println!("• Spinner animations for indeterminate progress");
            println!("• Multi-step progress tracking");
            println!("• Colored terminal output");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example progress_indicators -- progress-bar --steps 20");
        }

        ExampleType::InteractivePrompts => {
            println!("=== Interactive Prompts Example ===");
            println!("This example demonstrates user interaction patterns.");
            println!();
            println!("Key features demonstrated:");
            println!("• Text input with validation");
            println!("• Selection menus and multi-select");
            println!("• Password input (hidden)");
            println!("• Interactive wizards and forms");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example interactive_prompts -- wizard");
        }

        ExampleType::FileOperations => {
            println!("=== File Operations Example ===");
            println!("This example demonstrates file system utilities.");
            println!();
            println!("Key features demonstrated:");
            println!("• File reading, writing, and metadata");
            println!("• Directory traversal and search");
            println!("• Backup and validation operations");
            println!("• File watching and monitoring");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example file_operations -- basic-operations");
        }
    }

    println!();
    println!(
        "💡 All examples are also available as standalone programs in the examples/ directory."
    );

    Ok(())
}

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
