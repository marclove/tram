<img width="1280" height="640" alt="tram" src="https://github.com/user-attachments/assets/69138346-5acb-4ac8-9092-a2ff83ea949c" />

# Tram

A batteries-included starter kit for building production-ready CLI applications in Rust.

Tram combines the power of [clap](https://github.com/clap-rs/clap) for command-line argument parsing with [starbase](https://github.com/moonrepo/starbase) for enhanced developer experience, providing you with a solid foundation to build robust CLI tools.

## Features

- **Powerful CLI parsing** with clap's derive macros
- **Session-based architecture** for complex application lifecycles
- **Rich error handling** with beautiful diagnostics via miette
- **Structured logging** with console and JSON output formats
- **Project initialization** with support for Rust, Node.js, Python, Go, and generic projects
- **Multi-crate workspace** structure for organizing complex applications
- **Moon task runner** integration for efficient development workflows
- **Proto toolchain management** for consistent development environments
- **Built-in configuration management** with multiple source support (CLI, env, files)
- **Workspace detection** utilities for project-aware tools
- **Claude Code hooks** for automated quality checks
- **Behavior-driven development** patterns with comprehensive test coverage
- **Terminal UI components** for interactive experiences

## Quick Start

1. **Fork this repository** or use it as a template
2. **Clone your fork** and navigate to the directory
3. **Install dependencies and setup toolchain**:

```bash
just setup
```

4. **Build and run** the starter application:

```bash
just run -- --help
```

5. **Try creating a new project**:

```bash
# Create a new Rust project
just run -- new my-rust-app --project-type rust --description "My awesome CLI tool"

# Create a Node.js project  
just run -- new my-node-app --project-type nodejs --description "My Node.js CLI"

# See all available project types and options
just run -- new --help
```

## Project Structure

```
├── src/
│   └── main.rs                 # Application entry point with clap + starbase integration
├── crates/
│   ├── tram-core/              # Core types, error handling, logging, and project initialization
│   ├── tram-config/            # Multi-source configuration management (CLI, env, files)
│   └── tram-workspace/         # Workspace detection and project type identification
├── .claude/
│   ├── settings.json           # Claude Code hooks configuration
│   └── hooks/                  # Automated quality check scripts
├── Justfile                    # Development workflow recipes
├── moon.yml                    # Moon task runner configuration
└── .moon/
    └── workspace.yml           # Moon workspace settings
```

## Development Workflow

Tram uses [just](https://github.com/casey/just) for development commands and [moon](https://moonrepo.dev) for task orchestration:

```bash
# Quick development check (format, lint, build, test)
just check

# Run tests
just test

# Run your CLI with arguments
just run -- new my-project --project-type rust --description "My CLI tool"

# Legacy init command
just run -- init my-project --verbose

# Show workspace information
just run -- workspace --detailed

# Build the project
just build

# Watch for changes and run checks automatically
just watch

# Show all available commands
just --list
```

## Available CLI Commands

Tram includes several example commands to demonstrate common CLI patterns:

### `new` - Create New Projects
```bash
# Create a Rust project (default)
tram new my-rust-app

# Create projects for different languages
tram new my-node-app --project-type nodejs
tram new my-python-app --project-type python  
tram new my-go-app --project-type go

# Add description and skip interactive prompts
tram new my-app --description "My awesome CLI" --skip-prompts

# Supported project types: rust, nodejs, python, go, java, generic
```

### `workspace` - Workspace Information
```bash
# Show current workspace information
tram workspace

# Show detailed project information and ignore patterns
tram workspace --detailed
```

### `config` - Configuration Display
```bash
# Show current configuration
tram config
```

### Global Options
```bash
# Control logging output
tram --log-level debug workspace
tram --log-level info --format json config

# Use custom configuration file
tram --config ./my-config.toml workspace

# Disable colored output
tram --no-color workspace
```

## Building Your CLI

### 1. Define Your Commands

Use clap's derive API to define your CLI structure in `src/main.rs`:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mytool")]
#[command(about = "A CLI tool built with Tram")]
pub struct Cli {
    /// Global options
    #[command(flatten)]
    pub global: GlobalOptions,

    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Initialize a new project
    Init {
        name: String,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show workspace information
    Workspace {
        #[arg(short, long)]
        detailed: bool,
    },
}
```

### 2. Implement Your Session

Create your application session by implementing starbase's `AppSession` trait:

```rust
use async_trait::async_trait;
use starbase::AppSession;
use tram_core::AppResult;

#[derive(Clone, Debug)]
pub struct MySession {
    pub config: tram_config::TramConfig,
    pub workspace: tram_workspace::WorkspaceDetector,
    pub workspace_root: Option<PathBuf>,
}

#[async_trait]
impl AppSession for MySession {
    async fn startup(&mut self) -> AppResult<Option<u8>> {
        // Load configuration from multiple sources
        self.config = TramConfig::load_from_common_paths()?;

        // Detect workspace root
        if let Ok(root) = self.workspace.detect_root() {
            self.workspace_root = Some(root);
        }

        Ok(None)
    }

    async fn analyze(&mut self) -> AppResult<Option<u8>> {
        // Validate environment, check dependencies
        println!("Working in {} workspace", self.workspace_root.display());
        Ok(None)
    }

    async fn shutdown(&mut self) -> AppResult<Option<u8>> {
        // Cleanup resources
        println!("Done!");
        Ok(None)
    }
}
```

### 3. Extend with Additional Crates

Add new functionality by creating additional crates:

```bash
just new-crate my-feature
```

This creates a new crate in `crates/my-feature/` with moon configuration and basic structure.

## Architecture

### Multi-Crate Structure

Tram uses a multi-crate workspace to organize functionality:

- **`tram-core`** - Core types, error handling, and common utilities
- **`tram-config`** - Configuration management with multiple source support (CLI, env, files)
- **`tram-workspace`** - Workspace detection and project type identification

### Moon Task Runner Integration

Moon manages tasks across the workspace with intelligent dependency resolution and caching:

```yaml
# moon.yml - Root project configuration
tasks:
  build:
    command: 'cargo build --workspace --all-targets --all-features'
    deps:
      - 'tram-core:build'
      - 'tram-config:build'
      - 'tram-workspace:build'
```

### Configuration Management

Load configuration from multiple sources with proper precedence:

```rust
use tram_config::TramConfig;

// CLI args > environment variables > config files > defaults
let mut config = TramConfig::load_from_common_paths()?;
// Apply CLI overrides manually (highest precedence)
```

### Workspace Detection

Automatically detect project roots and types:

```rust
use tram_workspace::{WorkspaceDetector, ProjectType};

let detector = WorkspaceDetector::new()?;
let root = detector.detect_root()?;
let project_type = ProjectType::detect(&root);
```

### Core Utilities

Tram provides essential utilities for building robust CLI applications:

#### Error Handling
```rust
use tram_core::{TramError, AppResult};

fn my_command() -> AppResult<()> {
    // Rich error messages with diagnostic codes
    Err(TramError::ConfigNotFound { 
        path: "config.toml".to_string() 
    }.into())
}
```

#### Structured Logging
```rust
use tram_core::init_tracing;
use tracing::{info, debug, warn, error};

// Initialize with configurable output format
init_tracing("debug", false)?;  // Console output
init_tracing("info", true)?;    // JSON output for structured logging

info!("Application started");
debug!("Debug information: {:?}", data);
```

#### Project Creation
```rust
use tram_core::{ProjectInitializer, InitConfig, InitProjectType};

let config = InitConfig {
    name: "my-project".to_string(),
    path: PathBuf::from("./my-project"),
    project_type: InitProjectType::Rust,
    description: Some("A new CLI tool".to_string()),
    author: None,
};

let initializer = ProjectInitializer::new();
initializer.create_project(&config)?;
```

#### Multi-Source Configuration
```rust
use tram_config::TramConfig;

// Load from CLI args > env vars > config files > defaults
let config = TramConfig::load_from_common_paths()?;

// Use throughout your application
match config.output_format {
    OutputFormat::Json => println!("{}", serde_json::to_string(&data)?),
    OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&data)?),
    OutputFormat::Table => println!("{:#?}", data),
}
```

### Quality Assurance

Claude Code hooks automatically check for issues:

- **Rust compiler warnings** - Detected immediately after file edits
- **Code formatting** - Integrated with moon's format tasks
- **Linting** - Clippy runs with `-D warnings` to catch all issues
- **Behavior-driven testing** - Focus on user expectations, not implementation details

## Development Environment

### Setup

```bash
# Install proto and set up toolchain
proto install

# Initialize moon workspace
moon setup && moon sync
```

### Commands

```bash
# Development workflow
just check          # Format, lint, build, test
just build [CRATE]   # Build workspace or specific crate
just test [CRATE]    # Run tests
just run [ARGS]      # Run the CLI

# Project management
just new-crate NAME  # Create new crate with moon config
just clean           # Clean cargo + moon caches
just graph           # Generate project dependency graph
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
