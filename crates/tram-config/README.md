# tram-config

Configuration management using schematic.

Provides robust configuration loading from multiple sources with proper validation, type safety, and precedence using the schematic framework. Includes hot reload functionality for development workflows.

## Overview

`tram-config` provides a robust configuration system built on the schematic framework. It loads settings from multiple sources with proper precedence and demonstrates best practices for CLI configuration management.

## Key Features

### Multi-Source Configuration Loading

Loads configuration with this precedence order:
1. **CLI arguments** (highest priority) 
2. **Environment variables**
3. **Configuration files** (JSON, YAML, TOML)
4. **Default values** (lowest priority)

```rust
use tram_config::TramConfig;

// Load from environment and defaults
let config = TramConfig::load()?;

// Load from specific file
let config = TramConfig::load_from_file("./config.json")?;

// Load from common config file locations
let config = TramConfig::load_from_common_paths()?;
```

### Multiple File Format Support

Automatically detects and loads configuration files:

- **JSON** - `.tram.json`, `tram.json`
- **YAML** - `.tram.yaml`, `.tram.yml`, `tram.yaml`, `tram.yml`
- **TOML** - `.tram.toml`, `tram.toml`

**Important**: Configuration files must use camelCase field names:

```json
{
  "logLevel": "debug",
  "outputFormat": "json",
  "color": false
}
```

```yaml
logLevel: warn
outputFormat: table
color: true
```

```toml
logLevel = "info"
outputFormat = "table"
color = true
```

### Environment Variable Support

Loads configuration from environment variables with `TRAM_` prefix:

```bash
export TRAM_LOG_LEVEL=debug
export TRAM_OUTPUT_FORMAT=json
export TRAM_COLOR=false
export TRAM_WORKSPACE_ROOT=/path/to/workspace
```

### Built-in Validation

Configuration is validated automatically by schematic with helpful error messages for invalid values.

### Hot Reload Support

Real-time configuration reloading during development:

```rust
use tram_config::{ConfigWatcher, ConfigChangeHandler};
use async_trait::async_trait;

// Create a config watcher
let config_watcher = ConfigWatcher::new(initial_config, None).await?;

// Implement a custom change handler
struct MyConfigHandler;

#[async_trait]
impl ConfigChangeHandler for MyConfigHandler {
    async fn handle_config_change(&self, new_config: &TramConfig) {
        println!("Configuration reloaded successfully!");
        println!("New log level: {}", new_config.log_level);
    }

    async fn handle_config_error(&self, error: Box<dyn std::error::Error + Send + Sync>) {
        eprintln!("Failed to reload config: {}", error);
    }
}

// Start watching with the handler
config_watcher.start_with_handler(MyConfigHandler).await?;
```

**Hot reload features:**
- **Automatic detection** of changes to any supported config file format
- **Thread-safe config updates** using `Arc<RwLock<TramConfig>>`
- **Custom change handlers** for responding to config updates
- **Error handling** for invalid config changes (keeps previous valid config)
- **Graceful shutdown** with proper resource cleanup

## Configuration Structure

The main `TramConfig` struct provides common CLI configuration patterns:

```rust
use schematic::Config;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
}

#[derive(Clone, Debug, Deserialize, Serialize, Config)]
pub struct TramConfig {
    /// Log level (debug, info, warn, error)
    #[setting(default = "info", env = "TRAM_LOG_LEVEL")]
    pub log_level: LogLevel,

    /// Output format (json, yaml, table)  
    #[setting(default = "table", env = "TRAM_OUTPUT_FORMAT")]
    pub output_format: OutputFormat,

    /// Whether to use colors in output
    #[setting(default = true, env = "TRAM_COLOR")]
    pub color: bool,

    /// Workspace root directory
    #[setting(env = "TRAM_WORKSPACE_ROOT")]
    pub workspace_root: Option<PathBuf>,
}
```

## Integration with Your CLI

### 1. Load Configuration in Application Startup

```rust
use tram_config::TramConfig;

#[async_trait]
impl AppSession for TramSession {
    async fn startup(&mut self) -> AppResult<Option<u8>> {
        // Load base configuration (env + defaults)
        let mut config = TramConfig::load_from_common_paths()?;
        
        // Apply CLI overrides (highest precedence)
        if cli.global.log_level != "info" {
            config.log_level = cli.global.log_level.parse()?;
        }
        if cli.global.format != "table" {
            config.output_format = cli.global.format.parse()?;
        }
        if cli.global.no_color {
            config.color = false;
        }
        
        self.config = config;
        Ok(None)
    }
}
```

### 2. Use Configuration Throughout Your Application

```rust
// Access configuration in command handlers
fn execute_command(cmd: Commands, session: &TramSession) -> AppResult<()> {
    // Use log level for conditional output
    if matches!(session.config.log_level, LogLevel::Debug) {
        println!("Debug info...");
    }
    
    // Use output format for structured data
    match session.config.output_format {
        OutputFormat::Json => println!("{}", serde_json::to_string(&data)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&data)?),
        OutputFormat::Table => println!("{:#?}", data),
    }
    
    Ok(())
}
```

## Configuration Loading Methods

### `TramConfig::load()`

Loads configuration from environment variables and defaults only:

```rust
let config = TramConfig::load()?;
```

### `TramConfig::load_from_file(path)`

Loads configuration from a specific file:

```rust
let config = TramConfig::load_from_file("./my-config.json")?;
```

Supported file extensions: `.json`, `.yaml`, `.yml`, `.toml`

### `TramConfig::load_from_common_paths()`

Searches for config files in standard locations:

```rust
let config = TramConfig::load_from_common_paths()?;
```

Searches for these files in order:
- `tram.json`
- `tram.yaml`, `tram.yml`
- `tram.toml`
- `.tram.json`
- `.tram.yaml`, `.tram.yml`
- `.tram.toml`

## Hot Reload API

### `ConfigWatcher`

Creates a file system watcher for automatic configuration reloading:

```rust
use tram_config::{ConfigWatcher, TramConfig};

// Watch default config file locations
let watcher = ConfigWatcher::new(initial_config, None).await?;

// Watch specific config files
let custom_paths = vec![PathBuf::from("./custom-config.json")];
let watcher = ConfigWatcher::new(initial_config, Some(custom_paths)).await?;

// Get current config (thread-safe)
let current_config = watcher.get_config().await;

// Stop watching
watcher.stop().await;
```

### `ConfigChangeHandler` Trait

Implement this trait to handle configuration changes:

```rust
use async_trait::async_trait;
use tram_config::{ConfigChangeHandler, TramConfig};

struct LoggingHandler;

#[async_trait]
impl ConfigChangeHandler for LoggingHandler {
    async fn handle_config_change(&self, new_config: &TramConfig) {
        // Called when config is successfully reloaded
        println!("✓ Config updated: log_level={}", new_config.log_level);
    }

    async fn handle_config_error(&self, error: Box<dyn std::error::Error + Send + Sync>) {
        // Called when config reload fails
        eprintln!("✗ Config reload failed: {}", error);
    }
}

// Use with watcher
watcher.start_with_handler(LoggingHandler).await?;
```

## Enum Types

### LogLevel

Valid values: `debug`, `info`, `warn`, `error`

Implements `FromStr`, `Display`, and `From<&str>` for easy conversion.

### OutputFormat

Valid values: `json`, `yaml`, `table`

Implements `FromStr`, `Display`, and `From<&str>` for easy conversion.

## Design Patterns Demonstrated

### Schematic Integration

Uses schematic's `Config` derive macro with `#[setting]` attributes to define:
- Default values
- Environment variable mapping
- Field validation

### Type Safety

Strongly-typed enums prevent invalid configuration values and provide compile-time guarantees.

### Configuration Precedence

Environment variables automatically override defaults through schematic's built-in precedence handling. CLI arguments are applied manually as the highest precedence layer.

### Error Handling

Schematic provides detailed error messages for:
- Invalid file formats
- Unknown configuration fields
- Type conversion failures
- Missing required values

## Dependencies

- `schematic` - Configuration management framework
- `serde` - Configuration serialization/deserialization
- `serde_json` - JSON configuration support (via schematic)
- `notify` - File system watching for hot reload
- `tokio` - Async runtime for hot reload functionality
- `async-trait` - Async trait support
- `tracing` - Structured logging
- `tram-core` - Error handling and common types

## Testing

Configuration loading includes comprehensive tests covering:
- File format detection and loading (JSON, YAML, TOML)
- Environment variable parsing
- Default value handling
- Configuration merging and precedence
- Validation error cases
- Enum parsing and display

This ensures your CLI's configuration system works reliably across different deployment scenarios.