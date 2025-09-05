//! Build script for generating man pages and other build-time artifacts.
//!
//! This script automatically generates manual pages during the build process,
//! ensuring they're always up-to-date with the current CLI interface.

use clap::CommandFactory;
use clap_mangen::Man;
use std::env;
use std::fs;
use std::path::PathBuf;

// Import the CLI structure - we need to replicate the structure here
// since we can't import directly from main.rs during build
use clap::Parser;

/// CLI structure for build-time man page generation.
/// This must match the structure in main.rs exactly.
#[derive(Parser, Debug)]
#[command(name = "tram")]
#[command(about = "A batteries-included starter kit for building CLI applications in Rust")]
struct Cli {
    /// Global config options
    #[command(flatten)]
    pub global: GlobalOptions,

    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

/// Global CLI options that apply to all commands.
#[derive(Parser, Debug)]
struct GlobalOptions {
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
    pub config: Option<PathBuf>,
}

/// Available CLI commands.
#[derive(Parser, Debug)]
enum Commands {
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
        target_dir: Option<PathBuf>,
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
        shell: clap_complete::shells::Shell,
    },
    /// Generate manual pages
    Man {
        /// Output directory for man pages
        #[arg(short, long, default_value = "./man")]
        output_dir: PathBuf,
        /// Generate only specific section (1-9, default: all)
        #[arg(short, long)]
        section: Option<u8>,
    },
}

/// Available example types
#[derive(clap::ValueEnum, Clone, Debug)]
enum ExampleType {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only generate man pages in release builds or when explicitly requested
    let generate_man_pages = env::var("TRAM_GENERATE_MAN").unwrap_or_default() == "1"
        || env::var("PROFILE").unwrap_or_default() == "release";

    if generate_man_pages {
        generate_man_pages_to_out_dir()?;
    }

    // Rerun build script if CLI structure changes
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TRAM_GENERATE_MAN");

    Ok(())
}

/// Generate man pages to the OUT_DIR during build.
fn generate_man_pages_to_out_dir() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let man_dir = PathBuf::from(&out_dir).join("man");

    // Create man pages directory
    fs::create_dir_all(&man_dir)?;

    let cmd = Cli::command();
    let app_name = "tram";

    // Generate main command man page
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let man_file = man_dir.join(format!("{}.1", app_name));
    fs::write(&man_file, buffer)?;

    println!("cargo:warning=Generated man page: {}", man_file.display());

    // Generate subcommand man pages
    for subcommand in cmd.get_subcommands() {
        let subcommand_name = subcommand.get_name();

        let man = Man::new(subcommand.clone())
            .title(format!("{}-{}", app_name, subcommand_name))
            .section("1")
            .source(format!("{} {}", app_name, env!("CARGO_PKG_VERSION")))
            .manual("User Commands");

        let mut buffer = Vec::new();
        man.render(&mut buffer)?;

        let man_file = man_dir.join(format!("{}-{}.1", app_name, subcommand_name));
        fs::write(&man_file, buffer)?;

        println!("cargo:warning=Generated man page: {}", man_file.display());
    }

    // Set environment variable so the binary can find the man pages
    println!("cargo:rustc-env=TRAM_MAN_DIR={}", man_dir.display());

    Ok(())
}
