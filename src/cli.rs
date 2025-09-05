//! CLI argument parsing and structure definitions.
//!
//! This module defines the command-line interface structure using clap's derive API,
//! including all commands, options, and argument types.

use clap::Parser;
use clap_complete::shells::Shell;

/// CLI structure demonstrating clap + starbase patterns.
#[derive(Parser, Debug)]
#[command(name = "tram")]
#[command(about = "A batteries-included starter kit for building CLI applications in Rust")]
#[command(version)]
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
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Generate manual pages
    Man {
        /// Output directory for man pages
        #[arg(short, long, default_value = "./man")]
        output_dir: std::path::PathBuf,
        /// Generate only specific section (1-9, default: all)
        #[arg(short, long)]
        section: Option<u8>,
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
