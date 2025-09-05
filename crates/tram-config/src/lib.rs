//! Configuration management using schematic.
//!
//! Provides robust configuration loading from multiple sources with proper
//! validation, type safety, and precedence using the schematic framework.
//! Includes hot reload functionality for development workflows.

use async_trait::async_trait;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use schematic::{Config, ConfigLoader};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{debug, error, info, warn};

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

/// Trait for handling configuration changes during hot reload.
#[async_trait]
pub trait ConfigChangeHandler: Send + Sync {
    /// Called when a configuration change is detected and successfully loaded.
    async fn handle_config_change(&self, new_config: &TramConfig);

    /// Called when a configuration change is detected but fails to load.
    async fn handle_config_error(&self, error: Box<dyn std::error::Error + Send + Sync>);
}

/// Configuration watcher that provides hot reload functionality.
pub struct ConfigWatcher {
    config: Arc<RwLock<TramConfig>>,
    config_paths: Vec<PathBuf>,
    _watcher: RecommendedWatcher,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl ConfigWatcher {
    /// Create a new config watcher for the specified paths.
    /// If no paths are provided, watches common config file locations.
    pub async fn new(
        initial_config: TramConfig,
        config_paths: Option<Vec<PathBuf>>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let paths = config_paths.unwrap_or_else(|| {
            vec![
                "tram.json".into(),
                "tram.yaml".into(),
                "tram.yml".into(),
                "tram.toml".into(),
                ".tram.json".into(),
                ".tram.yaml".into(),
                ".tram.yml".into(),
                ".tram.toml".into(),
            ]
        });

        let config = Arc::new(RwLock::new(initial_config));
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        let (event_tx, mut event_rx) = mpsc::channel::<Result<Event, notify::Error>>(1000);

        // Create the file watcher
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = event_tx.blocking_send(res);
        })?;

        // Watch existing config files
        let existing_paths: Vec<_> = paths.iter().filter(|p| p.exists()).collect();

        for path in &existing_paths {
            debug!("Watching config file: {}", path.display());
            watcher.watch(path, RecursiveMode::NonRecursive)?;
        }

        if existing_paths.is_empty() {
            warn!("No existing config files found to watch");
        } else {
            info!(
                "Watching {} config file(s) for changes",
                existing_paths.len()
            );
        }

        // Clone config for the watch task
        let config_clone = Arc::clone(&config);
        let paths_clone = paths.clone();

        // Spawn the watch task
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(event_result) = event_rx.recv() => {
                        match event_result {
                            Ok(event) => {
                                if let Err(e) = Self::handle_file_event(&config_clone, &paths_clone, event).await {
                                    error!("Error handling config file event: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("File watcher error: {}", e);
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        debug!("Config watcher shutting down");
                        break;
                    }
                }
            }
        });

        Ok(Self {
            config,
            config_paths: paths,
            _watcher: watcher,
            shutdown_tx: Some(shutdown_tx),
        })
    }

    /// Get the current configuration (thread-safe).
    pub async fn get_config(&self) -> TramConfig {
        self.config.read().await.clone()
    }

    /// Start watching with a custom change handler.
    pub async fn start_with_handler<H>(
        &self,
        handler: H,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        H: ConfigChangeHandler + 'static,
    {
        let handler = Arc::new(handler);
        let config_clone = Arc::clone(&self.config);
        let paths_clone = self.config_paths.clone();
        let (event_tx, mut event_rx) = mpsc::channel::<Result<Event, notify::Error>>(1000);

        // Create a new watcher for this handler
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = event_tx.blocking_send(res);
        })?;

        // Watch existing config files
        for path in &paths_clone {
            if path.exists() {
                watcher.watch(path, RecursiveMode::NonRecursive)?;
            }
        }

        // Process events with the handler
        tokio::spawn(async move {
            while let Some(event_result) = event_rx.recv().await {
                match event_result {
                    Ok(event) => {
                        if let Err(e) = Self::handle_file_event_with_handler(
                            &config_clone,
                            &paths_clone,
                            event,
                            &handler,
                        )
                        .await
                        {
                            error!("Error handling config file event with handler: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("File watcher error: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Handle a file system event for config files.
    async fn handle_file_event(
        config: &Arc<RwLock<TramConfig>>,
        config_paths: &[PathBuf],
        event: Event,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
            return Ok(());
        }

        for path in &event.paths {
            if config_paths.iter().any(|p| p == path) {
                debug!("Config file changed: {}", path.display());

                match Self::reload_config_from_path(path).await {
                    Ok(new_config) => {
                        {
                            let mut config_guard = config.write().await;
                            *config_guard = new_config;
                        }
                        info!("Configuration reloaded from {}", path.display());
                    }
                    Err(e) => {
                        warn!("Failed to reload config from {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle a file system event with a custom handler.
    async fn handle_file_event_with_handler<H>(
        config: &Arc<RwLock<TramConfig>>,
        config_paths: &[PathBuf],
        event: Event,
        handler: &Arc<H>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        H: ConfigChangeHandler,
    {
        if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
            return Ok(());
        }

        for path in &event.paths {
            if config_paths.iter().any(|p| p == path) {
                debug!("Config file changed: {}", path.display());

                match Self::reload_config_from_path(path).await {
                    Ok(new_config) => {
                        {
                            let mut config_guard = config.write().await;
                            *config_guard = new_config.clone();
                        }
                        info!("Configuration reloaded from {}", path.display());
                        handler.handle_config_change(&new_config).await;
                    }
                    Err(e) => {
                        warn!("Failed to reload config from {}: {}", path.display(), e);
                        handler.handle_config_error(e).await;
                    }
                }
            }
        }

        Ok(())
    }

    /// Reload configuration from a specific path.
    async fn reload_config_from_path(
        path: &Path,
    ) -> Result<TramConfig, Box<dyn std::error::Error + Send + Sync>> {
        let path = path.to_owned();
        tokio::task::spawn_blocking(move || {
            TramConfig::load_from_file(path).map_err(
                |e| -> Box<dyn std::error::Error + Send + Sync> {
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Failed to load config: {}", e),
                    ))
                },
            )
        })
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { Box::new(e) })?
    }

    /// Stop watching for configuration changes.
    pub async fn stop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.try_send(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[serial]
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
    #[serial]
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
            "logLevel": "debug",
            "outputFormat": "json",
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
logLevel: warn
outputFormat: yaml
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
logLevel = "error"
outputFormat = "table"
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
    #[serial]
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
    #[serial]
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
            "logLevel": "debug",
            "outputFormat": "json",
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
    #[serial]
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
            "logLevel": "debug",
            "outputFormat": "json"
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
