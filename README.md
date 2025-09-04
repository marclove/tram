# Tram

A batteries-included starter kit for building production-ready CLI applications in Rust.

Tram combines the power of [clap](https://github.com/clap-rs/clap) for command-line argument parsing with [starbase](https://github.com/moonrepo/starbase) for enhanced developer experience, providing you with a solid foundation to build robust CLI tools.

## Features

- **Powerful CLI parsing** with clap's derive macros
- **Session-based architecture** for complex application lifecycles
- **Rich error handling** with beautiful diagnostics via miette
- **Terminal UI components** for interactive experiences
- **Built-in tracing and logging** for debugging and monitoring
- **Shell integration** for profile management and detection
- **File system utilities** with glob patterns and locking
- **Archive handling** for compression and extraction
- **Event-driven architecture** for extensible applications

## Quick Start

1. **Fork this repository** or use it as a template
2. **Clone your fork** and navigate to the directory
3. **Rename the project** in `Cargo.toml` to match your CLI tool
4. **Build and run** the starter application:

```bash
cargo run -- --help
```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── cli.rs           # Command-line interface definitions
├── session.rs       # Application session and lifecycle
├── commands/        # CLI command implementations
└── lib.rs           # Library code and utilities
```

## Building Your CLI

### 1. Define Your Commands

Use clap's derive API to define your CLI structure in `src/cli.rs`:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mytool")]
#[command(about = "A CLI tool built with Tram")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init {
        /// Project name
        name: String,
        /// Use verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },
}
```

### 2. Implement Your Session

Create your application session in `src/session.rs`:

```rust
use async_trait::async_trait;
use starbase::{AppResult, AppSession};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct MySession {
    pub workspace_root: PathBuf,
    pub config: MyConfig,
}

#[async_trait]
impl AppSession for MySession {
    async fn startup(&mut self) -> AppResult {
        // Initialize configuration, detect workspace
        self.workspace_root = detect_workspace_root()?;
        self.config = load_config(&self.workspace_root)?;
        Ok(None)
    }

    async fn analyze(&mut self) -> AppResult {
        // Analyze environment, validate inputs
        validate_workspace(&self.workspace_root)?;
        Ok(None)
    }

    async fn shutdown(&mut self) -> AppResult {
        // Cleanup resources
        save_cache(&self.workspace_root)?;
        Ok(None)
    }
}
```

### 3. Handle Commands

Implement your command logic in `src/commands/`:

```rust
use crate::{cli::Commands, session::MySession};
use starbase::AppResult;

pub async fn execute_command(
    command: Commands,
    session: &mut MySession,
) -> AppResult {
    match command {
        Commands::Init { name, verbose } => {
            init_project(&name, verbose, session).await
        }
        Commands::Build { release } => {
            build_project(release, session).await
        }
    }
}
```

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run your CLI with arguments
cargo run -- init my-project --verbose

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## Advanced Features

### Terminal UI

Use starbase's console components for rich terminal interactions:

```rust
use starbase_console::Console;

let console = Console::new();
console.print_header("Building Project");

let confirmed = console.confirm("Continue with build?")?;
if confirmed {
    let progress = console.progress_bar(100);
    // ... build logic with progress updates
}
```

### Error Handling

Define custom error types with rich diagnostics:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum MyError {
    #[error("Configuration file not found")]
    #[diagnostic(code(config::not_found), help("Run 'mytool init' first"))]
    ConfigNotFound,

    #[error("Invalid project structure")]
    #[diagnostic(code(project::invalid))]
    InvalidProject,
}
```

### Event System

Use starbase's event system for extensibility:

```rust
use starbase_events::{Event, EventEmitter};

#[derive(Event)]
pub struct ProjectBuilt {
    pub name: String,
    pub duration: Duration,
}

// Emit events
emitter.emit(ProjectBuilt {
    name: "my-project".into(),
    duration: build_duration,
})?;
```

## Examples

Check out the `examples/` directory for complete CLI application examples showing different patterns and use cases.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
