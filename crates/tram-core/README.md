# tram-core

Core utilities and patterns for building CLI applications with clap and starbase integration.

## Overview

`tram-core` provides the foundational utilities that CLI applications built with Tram need. It acts as the integration layer between clap (command-line parsing) and starbase (application lifecycle management), while providing essential services like logging, error handling, and project initialization.

## Key Components

### Error Handling (`error.rs`)

Provides `TramError` enum with rich diagnostic messages using `miette` and `thiserror`:

```rust
use tram_core::{TramError, AppResult};

// Structured errors with helpful diagnostic messages
return Err(TramError::ConfigNotFound { 
    path: "config.toml".to_string() 
}.into());
```

**Features:**
- Rich diagnostics with help text and error codes
- Integration with miette for beautiful terminal output
- Common CLI application error patterns

### Logging and Tracing (`logging.rs`)

Centralized logging setup with structured output support:

```rust
use tram_core::init_tracing;

// Initialize with configurable format and level
init_tracing("debug", false)?;  // Console output
init_tracing("info", true)?;    // JSON output
```

**Features:**
- Console and JSON output formats
- Configurable log levels (debug, info, warn, error)
- Thread-safe initialization (can be called multiple times)
- Integration with starbase's tracing infrastructure

### Project Initialization (`project_init.rs`)

Utilities for creating new projects with templates:

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

**Supported Project Types:**
- **Rust** - `Cargo.toml`, `src/main.rs`
- **Node.js** - `package.json`, `index.js`  
- **Python** - `pyproject.toml`, main module
- **Go** - `go.mod`, `main.go`
- **Generic** - `README.md`

## Integration with Your CLI

### 1. Error Handling

Use `AppResult<T>` for all fallible operations:

```rust
use tram_core::AppResult;

pub fn my_command() -> AppResult<()> {
    // Your logic here
    Ok(())
}
```

### 2. Logging Setup

Initialize logging in your application startup:

```rust
use tram_core::init_tracing;

#[async_trait]
impl AppSession for MySession {
    async fn startup(&mut self) -> AppResult<Option<u8>> {
        // Initialize logging based on configuration
        let use_json = self.config.output_format == "json";
        init_tracing(&self.config.log_level, use_json)?;
        
        info!("Application starting");
        Ok(None)
    }
}
```

### 3. Project Creation

For CLIs that create projects or scaffolding:

```rust
use tram_core::{ProjectInitializer, InitConfig, InitProjectType};

async fn create_project(name: String, project_type: String) -> AppResult<()> {
    let project_type = match project_type.as_str() {
        "rust" => InitProjectType::Rust,
        "node" => InitProjectType::NodeJs,
        _ => InitProjectType::Generic,
    };
    
    let config = InitConfig {
        name,
        path: std::env::current_dir()?.join(&name),
        project_type,
        description: None,
        author: None,
    };
    
    let initializer = ProjectInitializer::new();
    initializer.create_project(&config)?;
    Ok(())
}
```

## Design Philosophy

### Minimal Abstractions

`tram-core` avoids unnecessary abstractions over clap and starbase. Instead, it provides utilities that complement these libraries without hiding their APIs.

### Behavior-Driven Testing

All functionality is tested with behavior-driven tests that focus on user expectations rather than implementation details:

```rust
#[test]
fn test_create_rust_project() {
    // Test the behavior users expect, not internal implementation
    let result = initializer.create_project(&config);
    assert!(result.is_ok());
    assert!(project_path.join("Cargo.toml").exists());
}
```

### Error-First Design

Every operation that can fail returns `AppResult<T>`, making error handling explicit and providing rich diagnostic information to users.

## Dependencies

- `miette` - Rich diagnostic error messages
- `thiserror` - Error type definitions
- `tracing` + `tracing-subscriber` - Structured logging
- `starbase` - Application lifecycle management
- `serde` - Configuration serialization

## Usage in Multi-Crate Workspaces

`tram-core` is designed to be the foundation crate that other workspace crates depend on:

```toml
# In your other crates' Cargo.toml
[dependencies]
tram-core = { path = "../tram-core" }
```

This ensures consistent error handling, logging, and utilities across your entire CLI application.