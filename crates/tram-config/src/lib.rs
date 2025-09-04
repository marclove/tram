//! Configuration management for CLI applications.
//!
//! Provides utilities for loading configuration from multiple sources
//! (CLI args, environment variables, config files) with proper precedence.

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tram_core::{AppResult, TramError};

/// Example configuration structure showing common CLI app configuration patterns.
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

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            output_format: "table".to_string(),
            color: true,
            config_file: None,
            workspace_root: None,
        }
    }
}

impl Config {
    /// Load configuration with precedence: CLI args > env vars > config file > defaults
    pub fn load_from_args<T: Parser>(_cli_args: &T) -> AppResult<Self> {
        let mut config = Self::default();

        // 1. Start with defaults (already set)

        // 2. Load from config file if specified
        // This would normally use clap fields to get the config path
        // For now, check standard locations
        if let Some(file_config) = Self::load_from_file()? {
            config = Self::merge(config, file_config);
        }

        // 3. Override with environment variables
        config = Self::load_from_env(config);

        // 4. Override with CLI args
        // This would normally use clap fields to get the actual CLI values
        // For demonstration purposes, we'll skip this step

        Ok(config)
    }

    /// Load configuration from a file (JSON, YAML, or TOML)
    fn load_from_file() -> AppResult<Option<Self>> {
        // Try common config file locations
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

        for path in &config_paths {
            if Path::new(path).exists() {
                return Ok(Some(Self::load_config_file(path)?));
            }
        }

        Ok(None)
    }

    /// Load a specific config file
    fn load_config_file(path: &str) -> AppResult<Self> {
        let contents = fs::read_to_string(path).map_err(|_| TramError::ConfigNotFound {
            path: path.to_string(),
        })?;

        let config = match Path::new(path).extension().and_then(|ext| ext.to_str()) {
            Some("json") => {
                serde_json::from_str(&contents).map_err(|e| TramError::InvalidConfig {
                    message: format!("Failed to parse JSON config file {}: {}", path, e),
                })?
            }
            Some("yaml") | Some("yml") => {
                serde_yaml::from_str(&contents).map_err(|e| TramError::InvalidConfig {
                    message: format!("Failed to parse YAML config file {}: {}", path, e),
                })?
            }
            Some("toml") => toml::from_str(&contents).map_err(|e| TramError::InvalidConfig {
                message: format!("Failed to parse TOML config file {}: {}", path, e),
            })?,
            _ => {
                return Err(TramError::InvalidConfig {
                    message: format!("Unsupported config file format: {}", path),
                }
                .into());
            }
        };

        Ok(config)
    }

    /// Load configuration from environment variables
    fn load_from_env(mut config: Self) -> Self {
        if let Ok(log_level) = env::var("TRAM_LOG_LEVEL") {
            config.log_level = log_level;
        }

        if let Ok(output_format) = env::var("TRAM_OUTPUT_FORMAT") {
            config.output_format = output_format;
        }

        if let Ok(color) = env::var("TRAM_COLOR") {
            config.color = color.parse().unwrap_or(config.color);
        }

        if let Ok(workspace_root) = env::var("TRAM_WORKSPACE_ROOT") {
            config.workspace_root = Some(PathBuf::from(workspace_root));
        }

        config
    }

    /// Merge two configurations (right takes precedence)
    fn merge(mut base: Self, override_config: Self) -> Self {
        if !override_config.log_level.is_empty() {
            base.log_level = override_config.log_level;
        }

        if !override_config.output_format.is_empty() {
            base.output_format = override_config.output_format;
        }

        // For bool, we can't easily tell if it was explicitly set,
        // so we just take the override value
        base.color = override_config.color;

        if override_config.config_file.is_some() {
            base.config_file = override_config.config_file;
        }

        if override_config.workspace_root.is_some() {
            base.workspace_root = override_config.workspace_root;
        }

        base
    }

    /// Validate the configuration
    pub fn validate(&self) -> AppResult<()> {
        // Validate log level
        match self.log_level.as_str() {
            "debug" | "info" | "warn" | "error" => {}
            _ => {
                return Err(TramError::InvalidConfig {
                    message: format!(
                        "Invalid log level: {}. Must be debug, info, warn, or error",
                        self.log_level
                    ),
                }
                .into());
            }
        }

        // Validate output format
        match self.output_format.as_str() {
            "json" | "yaml" | "table" => {}
            _ => {
                return Err(TramError::InvalidConfig {
                    message: format!(
                        "Invalid output format: {}. Must be json, yaml, or table",
                        self.output_format
                    ),
                }
                .into());
            }
        }

        Ok(())
    }
}
