//! Configuration management using schematic.
//!
//! Provides robust configuration loading from multiple sources with proper
//! validation, type safety, and precedence using the schematic framework.

use schematic::{Config, ConfigLoader};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Log level configuration.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or(LogLevel::Info)
    }
}

/// Output format configuration.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Table
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Table => write!(f, "table"),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "table" => Ok(OutputFormat::Table),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or(OutputFormat::Table)
    }
}

/// Main configuration structure using schematic.
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

impl TramConfig {
    /// Load configuration from environment variables and defaults only.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let loader = ConfigLoader::<Self>::new();
        let result = loader.load()?;
        Ok(result.config)
    }

    /// Load configuration from a specific file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();

        // Validate file extension
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") | Some("yaml") | Some("yml") | Some("toml") => {
                // Schematic supports these formats
            }
            _ => return Err(format!("Unsupported config file format: {}", path.display()).into()),
        }

        let mut loader = ConfigLoader::<Self>::new();
        loader.file(path)?;
        let result = loader.load()?;
        Ok(result.config)
    }

    /// Find and load from common config file locations.
    pub fn load_from_common_paths() -> Result<Self, Box<dyn std::error::Error>> {
        let config_paths = [
            "tram.json",
            "tram.yaml",
            "tram.yml",
            "tram.toml",
            ".tram.json",
            ".tram.yaml",
            ".tram.yml",
            ".tram.toml",
        ];

        let mut loader = ConfigLoader::<Self>::new();

        // Look for the first existing config file
        for path in &config_paths {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                loader.file(&path_buf)?;
                break;
            }
        }

        // Debug: removed for cleaner error messages

        // Load with whatever we found (or just env vars if no file found)
        let result = loader.load()?;
        Ok(result.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_load_defaults() {
        // Clean up any existing environment variables to test defaults
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
            env::remove_var("TRAM_WORKSPACE_ROOT");
        }

        let config = TramConfig::load().unwrap();
        assert_eq!(config.log_level, LogLevel::Info);
        assert_eq!(config.output_format, OutputFormat::Table);
        assert!(config.color);
        assert!(config.workspace_root.is_none());
    }

    #[test]
    fn test_config_load_from_json_file() {
        // Clean up environment variables so file values aren't overridden
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test-config.json");

        let config_content = r#"{
            "log_level": "debug",
            "output_format": "json",
            "color": false
        }"#;
        fs::write(&config_file, config_content).unwrap();

        let config = TramConfig::load_from_file(&config_file).unwrap();
        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.output_format, OutputFormat::Json);
        assert!(!config.color);
    }

    #[test]
    fn test_config_load_from_yaml_file() {
        // Clean up environment variables so file values aren't overridden
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test-config.yaml");

        let config_content = r#"
log_level: warn
output_format: yaml
color: false
"#;
        fs::write(&config_file, config_content).unwrap();

        let config = TramConfig::load_from_file(&config_file).unwrap();
        assert_eq!(config.log_level, LogLevel::Warn);
        assert_eq!(config.output_format, OutputFormat::Yaml);
        assert!(!config.color);
    }

    #[test]
    fn test_config_load_from_toml_file() {
        // Clean up environment variables so file values aren't overridden
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test-config.toml");

        let config_content = r#"
log_level = "error"
output_format = "table"
color = true
"#;
        fs::write(&config_file, config_content).unwrap();

        let config = TramConfig::load_from_file(&config_file).unwrap();
        assert_eq!(config.log_level, LogLevel::Error);
        assert_eq!(config.output_format, OutputFormat::Table);
        assert!(config.color);
    }

    #[test]
    fn test_unsupported_file_format() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test-config.txt");
        fs::write(&config_file, "some content").unwrap();

        let result = TramConfig::load_from_file(&config_file);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unsupported config file format")
        );
    }

    #[test]
    fn test_environment_variables() {
        // Set environment variables for testing
        unsafe {
            env::set_var("TRAM_LOG_LEVEL", "debug");
            env::set_var("TRAM_OUTPUT_FORMAT", "json");
            env::set_var("TRAM_COLOR", "false");
        }

        let config = TramConfig::load().unwrap();
        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.output_format, OutputFormat::Json);
        assert!(!config.color);

        // Clean up environment variables
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }
    }

    #[test]
    fn test_config_enum_display() {
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Error.to_string(), "error");
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Yaml.to_string(), "yaml");
        assert_eq!(OutputFormat::Table.to_string(), "table");
    }

    #[test]
    fn test_load_from_common_paths_no_config() {
        // Clean up environment variables to test defaults
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }

        // Test when no config files exist - should still work with defaults
        let config = TramConfig::load_from_common_paths().unwrap();
        assert_eq!(config.log_level, LogLevel::Info);
        assert_eq!(config.output_format, OutputFormat::Table);
        assert!(config.color);
    }

    #[test]
    fn test_load_from_common_paths_with_config() {
        // Clean up environment variables so file values aren't overridden
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("tram.json");

        let config_content = r#"{
            "log_level": "debug",
            "output_format": "json",
            "color": false
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Change to temp directory for this test
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();

        let config = TramConfig::load_from_common_paths().unwrap();
        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.output_format, OutputFormat::Json);
        assert!(!config.color);

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_file_and_env_var_merging() {
        // Clean up environment variables first
        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
            env::remove_var("TRAM_OUTPUT_FORMAT");
            env::remove_var("TRAM_COLOR");
        }

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test-config.json");

        // File sets some values
        let config_content = r#"{
            "log_level": "debug",
            "output_format": "json"
        }"#;
        fs::write(&config_file, config_content).unwrap();

        // Env var overrides one value
        unsafe {
            env::set_var("TRAM_LOG_LEVEL", "error");
        }

        let config = TramConfig::load_from_file(&config_file).unwrap();

        // Env var should override file value
        assert_eq!(config.log_level, LogLevel::Error);
        // File value should be used where env var not set
        assert_eq!(config.output_format, OutputFormat::Json);
        // Default should be used where neither file nor env var set
        assert!(config.color);

        unsafe {
            env::remove_var("TRAM_LOG_LEVEL");
        }
    }
}
