# tram-config

Configuration management utilities for CLI applications with multi-source loading and validation.

## Overview

`tram-config` provides a robust configuration system that loads settings from multiple sources with proper precedence. It demonstrates best practices for CLI configuration management, supporting file formats commonly used in modern development workflows.

## Key Features

### Multi-Source Configuration Loading

Loads configuration with this precedence order:
1. **CLI arguments** (highest priority)
2. **Environment variables**
3. **Configuration files** (JSON, YAML, TOML)
4. **Default values** (lowest priority)

```rust
use tram_config::{Config, GlobalOptions};

// Create from CLI arguments
let global_options = GlobalOptions {
    log_level: "debug".to_string(),
    format: "json".to_string(),
    no_color: false,
    config: Some(PathBuf::from("./custom-config.toml")),
};

let config = Config::load_from_args(&global_options)?;
```

### Multiple File Format Support

Automatically detects and loads configuration files:

- **JSON** - `.tram.json`, `tram.json`
- **YAML** - `.tram.yaml`, `.tram.yml`, `tram.yaml`, `tram.yml`
- **TOML** - `.tram.toml`, `tram.toml`

Example configuration files:

```toml
# tram.toml
log_level = "info"
output_format = "table"
color = true

[workspace]
root = "/path/to/workspace"
```

```json
{
  "log_level": "debug",
  "output_format": "json",
  "color": false
}
```

```yaml
log_level: warn
output_format: table
color: true
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

Configuration is validated on load with helpful error messages:

```rust
config.validate()?;  // Validates log levels, output formats, etc.
```

## Configuration Structure

The main `Config` struct provides common CLI configuration patterns:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Log level (debug, info, warn, error)
    pub log_level: String,
    
    /// Output format (json, yaml, table)
    pub output_format: String,
    
    /// Whether to use colors in output
    pub color: bool,
    
    /// Config file path
    pub config_file: Option<PathBuf>,
    
    /// Workspace root directory
    pub workspace_root: Option<PathBuf>,
}
```

## Integration with Your CLI

### 1. Define Global Options

Create a structure to bridge clap arguments to configuration loading:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GlobalOptions {
    #[arg(long, default_value = "info")]
    pub log_level: String,
    
    #[arg(long, default_value = "table")]
    pub format: String,
    
    #[arg(long)]
    pub no_color: bool,
    
    #[arg(long)]
    pub config: Option<PathBuf>,
}
```

### 2. Load Configuration in Application Startup

```rust
use tram_config::{Config, GlobalOptions};

#[async_trait]
impl AppSession for MySession {
    async fn startup(&mut self) -> AppResult<Option<u8>> {
        // Convert clap args to config loading format
        let global_options = GlobalOptions {
            log_level: cli.global.log_level.clone(),
            format: cli.global.format.clone(),
            no_color: cli.global.no_color,
            config: cli.global.config.clone(),
        };
        
        // Load with precedence: CLI > env > file > defaults
        self.config = Config::load_from_args(&global_options)?;
        self.config.validate()?;
        
        Ok(None)
    }
}
```

### 3. Use Configuration Throughout Your Application

```rust
// Access configuration in command handlers
fn execute_command(cmd: Commands, session: &MySession) -> AppResult<()> {
    // Use log level for conditional output
    if session.config.log_level == "debug" {
        println!("Debug info...");
    }
    
    // Use output format for structured data
    match session.config.output_format.as_str() {
        "json" => println!("{}", serde_json::to_string(&data)?),
        "yaml" => println!("{}", serde_yaml::to_string(&data)?),
        _ => println!("{:#?}", data),
    }
    
    Ok(())
}
```

## Extending Configuration

### Adding New Configuration Fields

1. Add fields to the `Config` struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ... existing fields ...
    
    /// Your new configuration option
    pub my_option: String,
}
```

2. Update the default implementation:

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            // ... existing defaults ...
            my_option: "default_value".to_string(),
        }
    }
}
```

3. Add environment variable support:

```rust
// In load_from_env method
if let Ok(my_option) = env::var("TRAM_MY_OPTION") {
    config.my_option = my_option;
}
```

4. Add CLI argument integration and validation as needed.

### Custom Configuration Loading

For specialized needs, you can implement custom loading logic:

```rust
impl Config {
    pub fn load_with_custom_sources() -> AppResult<Self> {
        let mut config = Self::default();
        
        // Load from custom sources
        if let Some(custom_config) = load_from_custom_source()? {
            config = Self::merge(config, custom_config);
        }
        
        // Apply environment and CLI overrides
        config = Self::load_from_env(config);
        
        config.validate()?;
        Ok(config)
    }
}
```

## Design Patterns Demonstrated

### Configuration Precedence

The loading system demonstrates the standard precedence pattern used by most CLI tools:
- CLI arguments override everything (user's immediate intent)
- Environment variables override files (deployment/runtime configuration)
- Configuration files override defaults (project-specific settings)
- Defaults provide sensible fallbacks

### Validation Separation

Configuration loading and validation are separate steps, allowing for:
- Early error detection with clear messages
- Flexible loading strategies
- Easy testing of validation logic

### Multiple Format Support

Supporting multiple configuration formats (JSON, YAML, TOML) allows users to:
- Use their preferred format
- Integrate with existing toolchains
- Choose the right format for their use case

## Dependencies

- `serde` - Configuration serialization/deserialization
- `serde_json` - JSON configuration support
- `serde_yaml` - YAML configuration support  
- `toml` - TOML configuration support
- `tram-core` - Error handling and common types

## Testing

Configuration loading includes comprehensive tests covering:
- File format detection and loading
- Environment variable parsing
- Configuration merging and precedence
- Validation error cases
- Default value handling

This ensures your CLI's configuration system works reliably across different deployment scenarios.