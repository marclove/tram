//! Template generation for common CLI patterns.
//!
//! Provides utilities for generating boilerplate code for common CLI patterns,
//! helping developers quickly add new functionality to their applications.

use crate::{AppResult, TramError};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Supported template types for CLI applications.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateType {
    /// Generate a new CLI command module
    Command,
    /// Generate configuration section
    ConfigSection,
    /// Generate custom error type
    ErrorType,
    /// Generate session extension
    SessionExtension,
}

/// Configuration for template generation.
#[derive(Debug, Clone)]
pub struct TemplateConfig {
    /// Name of the item to generate (e.g., "backup", "deploy")
    pub name: String,
    /// Type of template to generate
    pub template_type: TemplateType,
    /// Target directory for generation
    pub target_dir: PathBuf,
    /// Additional parameters for template customization
    pub parameters: HashMap<String, String>,
}

/// Service for generating templates from common CLI patterns.
pub struct TemplateGenerator {
    // Future: Cache for preloaded templates
    #[allow(dead_code)]
    templates_cache: HashMap<TemplateType, String>,
}

impl TemplateGenerator {
    pub fn new() -> Self {
        Self {
            templates_cache: HashMap::new(),
        }
    }

    /// Generate a template based on the provided configuration.
    /// This is the main behavior users expect when generating templates.
    pub fn generate_template(&self, config: &TemplateConfig) -> AppResult<GeneratedTemplate> {
        // Behavior: Should validate template name
        if config.name.is_empty() {
            return Err(TramError::InvalidConfig {
                message: "Template name cannot be empty".to_string(),
            }
            .into());
        }

        // Behavior: Should validate target directory exists
        if !config.target_dir.exists() {
            return Err(TramError::InvalidConfig {
                message: format!(
                    "Target directory {} does not exist",
                    config.target_dir.display()
                ),
            }
            .into());
        }

        // Behavior: Should generate appropriate content based on template type
        let content = self.generate_content(config)?;
        let file_path = self.determine_file_path(config)?;

        // Behavior: Should not overwrite existing files without confirmation
        if file_path.exists() {
            return Err(TramError::InvalidConfig {
                message: format!("File {} already exists", file_path.display()),
            }
            .into());
        }

        Ok(GeneratedTemplate {
            content,
            file_path,
            template_type: config.template_type.clone(),
            name: config.name.clone(),
        })
    }

    /// Write the generated template to the filesystem.
    pub fn write_template(&self, template: &GeneratedTemplate) -> AppResult<()> {
        // Behavior: Should create parent directories if needed
        if let Some(parent) = template.file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| TramError::InvalidConfig {
                message: format!("Failed to create directory {}: {}", parent.display(), e),
            })?;
        }

        // Behavior: Should write content to file
        fs::write(&template.file_path, &template.content).map_err(|e| {
            TramError::InvalidConfig {
                message: format!(
                    "Failed to write file {}: {}",
                    template.file_path.display(),
                    e
                ),
            }
        })?;

        Ok(())
    }

    /// Generate content based on template type and configuration.
    fn generate_content(&self, config: &TemplateConfig) -> AppResult<String> {
        match config.template_type {
            TemplateType::Command => self.generate_command_template(config),
            TemplateType::ConfigSection => self.generate_config_section_template(config),
            TemplateType::ErrorType => self.generate_error_type_template(config),
            TemplateType::SessionExtension => self.generate_session_extension_template(config),
        }
    }

    /// Determine the appropriate file path for the generated template.
    fn determine_file_path(&self, config: &TemplateConfig) -> AppResult<PathBuf> {
        match config.template_type {
            TemplateType::Command => Ok(config
                .target_dir
                .join("src")
                .join("commands")
                .join(format!("{}.rs", config.name))),
            TemplateType::ConfigSection => Ok(config
                .target_dir
                .join("src")
                .join("config")
                .join(format!("{}.rs", config.name))),
            TemplateType::ErrorType => Ok(config
                .target_dir
                .join("src")
                .join("errors")
                .join(format!("{}.rs", config.name))),
            TemplateType::SessionExtension => Ok(config
                .target_dir
                .join("src")
                .join("session")
                .join(format!("{}.rs", config.name))),
        }
    }

    fn generate_command_template(&self, config: &TemplateConfig) -> AppResult<String> {
        let command_name = &config.name;
        let command_name_pascal = to_pascal_case(command_name);
        let default_description = format!("{} command", command_name);
        let description = config
            .parameters
            .get("description")
            .unwrap_or(&default_description);

        let template = format!(
            r#"//! {} command implementation.

use clap::Parser;
use tracing::{{info, debug}};
use crate::{{AppResult, TramError}};

/// {} command arguments.
#[derive(Parser, Debug)]
pub struct {}Args {{
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Dry run mode - show what would be done without executing
    #[arg(long)]
    pub dry_run: bool,
}}

/// Execute the {} command.
pub async fn execute(args: {}Args) -> AppResult<()> {{
    info!("Executing {} command");
    
    if args.verbose {{
        debug!("Verbose mode enabled");
        debug!("Arguments: {{:?}}", args);
    }}
    
    if args.dry_run {{
        println!("DRY RUN: Would execute {} command");
        return Ok(());
    }}
    
    // TODO: Implement {} command logic here
    println!("Running {} command...");
    
    // Example error handling
    // return Err(TramError::InvalidConfig {{
    //     message: "Example error message".to_string(),
    // }}.into());
    
    println!("{} command completed successfully!");
    Ok(())
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[tokio::test]
    async fn test_{}_command_success() {{
        let args = {}Args {{
            verbose: false,
            dry_run: false,
        }};
        
        let result = execute(args).await;
        assert!(result.is_ok(), "Command should execute successfully");
    }}
    
    #[tokio::test]
    async fn test_{}_command_dry_run() {{
        let args = {}Args {{
            verbose: true,
            dry_run: true,
        }};
        
        let result = execute(args).await;
        assert!(result.is_ok(), "Dry run should complete successfully");
    }}
}}
"#,
            description,         // comment
            command_name_pascal, // struct comment
            command_name_pascal, // struct name
            command_name,        // function comment
            command_name_pascal, // function parameter type
            command_name,        // info log
            command_name,        // dry run message
            command_name,        // TODO comment
            command_name,        // Running message
            command_name_pascal, // Success message
            command_name,        // test function name
            command_name_pascal, // test args type
            command_name,        // second test function name
            command_name_pascal, // second test args type
        );

        Ok(template)
    }

    fn generate_config_section_template(&self, config: &TemplateConfig) -> AppResult<String> {
        let section_name = &config.name;
        let section_name_pascal = to_pascal_case(section_name);

        let template = format!(
            r#"//! {} configuration section.

use serde::{{Deserialize, Serialize}};
use std::path::PathBuf;
use crate::{{AppResult, TramError}};

/// Configuration for {} functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {}Config {{
    /// Enable {} functionality
    pub enabled: bool,
    
    /// {} timeout in seconds
    pub timeout: u64,
    
    /// {} working directory
    pub working_dir: Option<PathBuf>,
    
    /// Additional {} options
    pub options: Vec<String>,
}}

impl Default for {}Config {{
    fn default() -> Self {{
        Self {{
            enabled: true,
            timeout: 30,
            working_dir: None,
            options: Vec::new(),
        }}
    }}
}}

impl {}Config {{
    /// Validate the {} configuration.
    pub fn validate(&self) -> AppResult<()> {{
        if self.timeout == 0 {{
            return Err(TramError::InvalidConfig {{
                message: "{} timeout must be greater than 0".to_string(),
            }}.into());
        }}
        
        if let Some(dir) = &self.working_dir {{
            if !dir.exists() {{
                return Err(TramError::InvalidConfig {{
                    message: format!("{} working directory does not exist: {{}}", dir.display()),
                }}.into());
            }}
        }}
        
        Ok(())
    }}
    
    /// Load {} configuration from environment variables.
    pub fn load_from_env(&mut self) {{
        if let Ok(enabled) = std::env::var("TRAM_{}_ENABLED") {{
            self.enabled = enabled.parse().unwrap_or(self.enabled);
        }}
        
        if let Ok(timeout) = std::env::var("TRAM_{}_TIMEOUT") {{
            self.timeout = timeout.parse().unwrap_or(self.timeout);
        }}
        
        if let Ok(working_dir) = std::env::var("TRAM_{}_WORKING_DIR") {{
            self.working_dir = Some(PathBuf::from(working_dir));
        }}
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_{}_config_default() {{
        let config = {}Config::default();
        assert!(config.enabled);
        assert_eq!(config.timeout, 30);
        assert!(config.working_dir.is_none());
        assert!(config.options.is_empty());
    }}
    
    #[test]
    fn test_{}_config_validation_success() {{
        let config = {}Config::default();
        assert!(config.validate().is_ok());
    }}
    
    #[test]
    fn test_{}_config_validation_timeout_error() {{
        let mut config = {}Config::default();
        config.timeout = 0;
        
        let result = config.validate();
        assert!(result.is_err());
    }}
    
    #[test]
    fn test_{}_config_validation_directory_error() {{
        let mut config = {}Config::default();
        config.working_dir = Some(PathBuf::from("/nonexistent/directory"));
        
        let result = config.validate();
        assert!(result.is_err());
    }}
}}
"#,
            section_name,                // comment
            section_name,                // struct comment
            section_name_pascal,         // struct name
            section_name,                // enabled field comment
            section_name,                // timeout field comment
            section_name,                // working_dir field comment
            section_name,                // options field comment
            section_name_pascal,         // Default impl
            section_name_pascal,         // validate impl
            section_name,                // validate comment
            section_name_pascal,         // timeout error
            section_name_pascal,         // working directory error
            section_name,                // load_from_env comment
            section_name.to_uppercase(), // ENABLED env var
            section_name.to_uppercase(), // TIMEOUT env var
            section_name.to_uppercase(), // WORKING_DIR env var
            section_name,                // test function name
            section_name_pascal,         // test default type
            section_name,                // test validation success
            section_name_pascal,         // test validation type
            section_name,                // test validation timeout
            section_name_pascal,         // test validation timeout type
            section_name,                // test validation directory
            section_name_pascal,         // test validation directory type
        );

        Ok(template)
    }

    fn generate_error_type_template(&self, config: &TemplateConfig) -> AppResult<String> {
        let error_name = &config.name;
        let error_name_pascal = to_pascal_case(error_name);

        let template = format!(
            r#"//! {} specific error types.

use miette::Diagnostic;
use thiserror::Error;

/// Errors specific to {} functionality.
#[derive(Debug, Diagnostic, Error)]
pub enum {}Error {{
    #[error("{} operation failed: {{message}}")]
    #[diagnostic(
        code(tram::{}_operation_failed),
        help("Check the {} configuration and try again")
    )]
    OperationFailed {{ message: String }},
    
    #[error("{} resource not found: {{resource}}")]
    #[diagnostic(
        code(tram::{}_resource_not_found),
        help("Ensure the {} resource exists and is accessible")
    )]
    ResourceNotFound {{ resource: String }},
    
    #[error("{} timeout after {{timeout}}s")]
    #[diagnostic(
        code(tram::{}_timeout),
        help("Increase the timeout value or check {} service availability")
    )]
    Timeout {{ timeout: u64 }},
    
    #[error("{} configuration invalid: {{message}}")]
    #[diagnostic(
        code(tram::{}_invalid_config),
        help("Review the {} configuration file and fix any errors")
    )]
    InvalidConfig {{ message: String }},
}}

impl {}Error {{
    /// Create an operation failed error with a custom message.
    pub fn operation_failed<S: Into<String>>(message: S) -> Self {{
        Self::OperationFailed {{
            message: message.into(),
        }}
    }}
    
    /// Create a resource not found error.
    pub fn resource_not_found<S: Into<String>>(resource: S) -> Self {{
        Self::ResourceNotFound {{
            resource: resource.into(),
        }}
    }}
    
    /// Create a timeout error.
    pub fn timeout(timeout: u64) -> Self {{
        Self::Timeout {{ timeout }}
    }}
    
    /// Create an invalid configuration error.
    pub fn invalid_config<S: Into<String>>(message: S) -> Self {{
        Self::InvalidConfig {{
            message: message.into(),
        }}
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_{}_error_operation_failed() {{
        let error = {}Error::operation_failed("test message");
        assert!(matches!(error, {}Error::OperationFailed {{ .. }}));
        assert_eq!(error.to_string(), "{} operation failed: test message");
    }}
    
    #[test]
    fn test_{}_error_resource_not_found() {{
        let error = {}Error::resource_not_found("test.txt");
        assert!(matches!(error, {}Error::ResourceNotFound {{ .. }}));
        assert_eq!(error.to_string(), "{} resource not found: test.txt");
    }}
    
    #[test]
    fn test_{}_error_timeout() {{
        let error = {}Error::timeout(30);
        assert!(matches!(error, {}Error::Timeout {{ .. }}));
        assert_eq!(error.to_string(), "{} timeout after 30s");
    }}
    
    #[test]
    fn test_{}_error_invalid_config() {{
        let error = {}Error::invalid_config("bad value");
        assert!(matches!(error, {}Error::InvalidConfig {{ .. }}));
        assert_eq!(error.to_string(), "{} configuration invalid: bad value");
    }}
}}
"#,
            error_name,        // comment
            error_name,        // enum comment
            error_name_pascal, // enum name
            error_name_pascal, // OperationFailed error message
            error_name,        // diagnostic code
            error_name,        // diagnostic help
            error_name_pascal, // ResourceNotFound error message
            error_name,        // diagnostic code
            error_name,        // diagnostic help
            error_name_pascal, // Timeout error message
            error_name,        // diagnostic code
            error_name,        // diagnostic help
            error_name_pascal, // InvalidConfig error message
            error_name,        // diagnostic code
            error_name,        // diagnostic help
            error_name_pascal, // impl block
            error_name,        // test operation failed
            error_name_pascal, // test operation failed type
            error_name_pascal, // test operation failed match
            error_name_pascal, // test operation failed message
            error_name,        // test resource not found
            error_name_pascal, // test resource not found type
            error_name_pascal, // test resource not found match
            error_name_pascal, // test resource not found message
            error_name,        // test timeout
            error_name_pascal, // test timeout type
            error_name_pascal, // test timeout match
            error_name_pascal, // test timeout message
            error_name,        // test invalid config
            error_name_pascal, // test invalid config type
            error_name_pascal, // test invalid config match
            error_name_pascal, // test invalid config message
        );

        Ok(template)
    }

    fn generate_session_extension_template(&self, config: &TemplateConfig) -> AppResult<String> {
        let extension_name = &config.name;
        let extension_name_pascal = to_pascal_case(extension_name);

        let template = format!(
            r#"//! Session extension for {} functionality.

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{{AppResult, TramError}};

/// {} session extension data.
#[derive(Debug, Clone)]
pub struct {}Extension {{
    /// {} state
    pub state: Arc<RwLock<{}State>>,
    
    /// {} configuration
    pub config: {}Config,
}}

/// Internal state for {} functionality.
#[derive(Debug, Default)]
pub struct {}State {{
    /// Whether {} is initialized
    pub initialized: bool,
    
    /// {} operation count
    pub operation_count: u64,
    
    /// Last {} operation timestamp
    pub last_operation: Option<std::time::SystemTime>,
}}

/// Configuration for {} session extension.
#[derive(Debug, Clone)]
pub struct {}Config {{
    /// Enable {} extension
    pub enabled: bool,
    
    /// {} operation timeout in seconds
    pub timeout: u64,
    
    /// Maximum number of concurrent {} operations
    pub max_concurrent: u32,
}}

impl Default for {}Config {{
    fn default() -> Self {{
        Self {{
            enabled: true,
            timeout: 30,
            max_concurrent: 10,
        }}
    }}
}}

impl {}Extension {{
    /// Create a new {} extension.
    pub fn new(config: {}Config) -> Self {{
        Self {{
            state: Arc::new(RwLock::new({}State::default())),
            config,
        }}
    }}
    
    /// Initialize the {} extension.
    pub async fn initialize(&self) -> AppResult<()> {{
        let mut state = self.state.write().await;
        
        if state.initialized {{
            return Ok(());
        }}
        
        // TODO: Add {} initialization logic here
        
        state.initialized = true;
        state.last_operation = Some(std::time::SystemTime::now());
        
        Ok(())
    }}
    
    /// Execute a {} operation.
    pub async fn execute_operation(&self, operation_name: &str) -> AppResult<()> {{
        if !self.config.enabled {{
            return Err(TramError::InvalidConfig {{
                message: "{} extension is disabled".to_string(),
            }}.into());
        }}
        
        let mut state = self.state.write().await;
        
        if !state.initialized {{
            return Err(TramError::InvalidConfig {{
                message: "{} extension not initialized".to_string(),
            }}.into());
        }}
        
        // TODO: Add {} operation logic here
        println!("Executing {} operation: {{}}", operation_name);
        
        state.operation_count += 1;
        state.last_operation = Some(std::time::SystemTime::now());
        
        Ok(())
    }}
    
    /// Get {} statistics.
    pub async fn get_stats(&self) -> AppResult<{}Stats> {{
        let state = self.state.read().await;
        
        Ok({}Stats {{
            initialized: state.initialized,
            operation_count: state.operation_count,
            last_operation: state.last_operation,
        }})
    }}
}}

/// {} statistics.
#[derive(Debug, Clone)]
pub struct {}Stats {{
    /// Whether {} is initialized
    pub initialized: bool,
    
    /// Number of {} operations performed
    pub operation_count: u64,
    
    /// Timestamp of last {} operation
    pub last_operation: Option<std::time::SystemTime>,
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[tokio::test]
    async fn test_{}_extension_creation() {{
        let config = {}Config::default();
        let extension = {}Extension::new(config);
        
        let stats = extension.get_stats().await.unwrap();
        assert!(!stats.initialized);
        assert_eq!(stats.operation_count, 0);
    }}
    
    #[tokio::test]
    async fn test_{}_extension_initialization() {{
        let config = {}Config::default();
        let extension = {}Extension::new(config);
        
        extension.initialize().await.unwrap();
        
        let stats = extension.get_stats().await.unwrap();
        assert!(stats.initialized);
    }}
    
    #[tokio::test]
    async fn test_{}_extension_operation() {{
        let config = {}Config::default();
        let extension = {}Extension::new(config);
        
        extension.initialize().await.unwrap();
        extension.execute_operation("test").await.unwrap();
        
        let stats = extension.get_stats().await.unwrap();
        assert_eq!(stats.operation_count, 1);
    }}
    
    #[tokio::test]
    async fn test_{}_extension_operation_without_init() {{
        let config = {}Config::default();
        let extension = {}Extension::new(config);
        
        let result = extension.execute_operation("test").await;
        assert!(result.is_err());
    }}
}}
"#,
            extension_name,        // comment
            extension_name_pascal, // Extension struct comment
            extension_name_pascal, // Extension struct name
            extension_name,        // state field comment
            extension_name_pascal, // State type
            extension_name,        // config field comment
            extension_name_pascal, // Config type
            extension_name,        // State struct comment
            extension_name_pascal, // State struct name
            extension_name,        // initialized field comment
            extension_name,        // operation_count field comment
            extension_name,        // last_operation field comment
            extension_name,        // Config struct comment
            extension_name_pascal, // Config struct name
            extension_name,        // enabled field comment
            extension_name,        // timeout field comment
            extension_name,        // max_concurrent field comment
            extension_name_pascal, // Config default impl
            extension_name_pascal, // Extension impl
            extension_name,        // new function comment
            extension_name_pascal, // new function config param
            extension_name_pascal, // State default
            extension_name,        // initialize comment
            extension_name,        // TODO comment
            extension_name,        // execute_operation comment
            extension_name_pascal, // extension disabled error
            extension_name_pascal, // not initialized error
            extension_name,        // TODO operation comment
            extension_name,        // operation println
            extension_name,        // get_stats comment
            extension_name_pascal, // Stats return type
            extension_name_pascal, // Stats struct creation
            extension_name_pascal, // Stats struct comment
            extension_name_pascal, // Stats struct name
            extension_name,        // initialized field comment
            extension_name,        // operation_count field comment
            extension_name,        // last_operation field comment
            extension_name,        // test creation function name
            extension_name_pascal, // test Config type
            extension_name_pascal, // test Extension type
            extension_name,        // test initialization function name
            extension_name_pascal, // test Config type
            extension_name_pascal, // test Extension type
            extension_name,        // test operation function name
            extension_name_pascal, // test Config type
            extension_name_pascal, // test Extension type
            extension_name,        // test operation without init function name
            extension_name_pascal, // test Config type
            extension_name_pascal, // test Extension type
        );

        Ok(template)
    }
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of template generation.
#[derive(Debug, Clone)]
pub struct GeneratedTemplate {
    /// Generated content
    pub content: String,
    /// File path where template should be written
    pub file_path: PathBuf,
    /// Template type that was generated
    pub template_type: TemplateType,
    /// Name of the generated item
    pub name: String,
}

/// Convert a string to PascalCase.
fn to_pascal_case(s: &str) -> String {
    s.split(['-', '_'])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_command_template() {
        let temp_dir = TempDir::new().unwrap();

        let config = TemplateConfig {
            name: "backup".to_string(),
            template_type: TemplateType::Command,
            target_dir: temp_dir.path().to_path_buf(),
            parameters: [("description".to_string(), "Backup data command".to_string())]
                .into_iter()
                .collect(),
        };

        let generator = TemplateGenerator::new();
        let result = generator.generate_template(&config);

        assert!(
            result.is_ok(),
            "Should generate command template successfully"
        );
        let template = result.unwrap();

        assert!(template.content.contains("BackupArgs"));
        assert!(template.content.contains("Backup data command"));
        assert!(template.content.contains("pub async fn execute"));
        assert!(template.content.contains("#[tokio::test]"));
    }

    #[test]
    fn test_generate_config_section_template() {
        let temp_dir = TempDir::new().unwrap();

        let config = TemplateConfig {
            name: "database".to_string(),
            template_type: TemplateType::ConfigSection,
            target_dir: temp_dir.path().to_path_buf(),
            parameters: HashMap::new(),
        };

        let generator = TemplateGenerator::new();
        let result = generator.generate_template(&config);

        assert!(
            result.is_ok(),
            "Should generate config section template successfully"
        );
        let template = result.unwrap();

        assert!(template.content.contains("DatabaseConfig"));
        assert!(template.content.contains("pub fn validate"));
        assert!(template.content.contains("load_from_env"));
        assert!(template.content.contains("TRAM_DATABASE_"));
    }

    #[test]
    fn test_generate_template_fails_with_empty_name() {
        let temp_dir = TempDir::new().unwrap();

        let config = TemplateConfig {
            name: "".to_string(),
            template_type: TemplateType::Command,
            target_dir: temp_dir.path().to_path_buf(),
            parameters: HashMap::new(),
        };

        let generator = TemplateGenerator::new();
        let result = generator.generate_template(&config);

        assert!(result.is_err(), "Should fail with empty template name");
    }

    #[test]
    fn test_generate_template_fails_with_nonexistent_directory() {
        let config = TemplateConfig {
            name: "test".to_string(),
            template_type: TemplateType::Command,
            target_dir: PathBuf::from("/nonexistent/directory"),
            parameters: HashMap::new(),
        };

        let generator = TemplateGenerator::new();
        let result = generator.generate_template(&config);

        assert!(result.is_err(), "Should fail with nonexistent directory");
    }

    #[test]
    fn test_write_template_creates_directories() {
        let temp_dir = TempDir::new().unwrap();

        let template = GeneratedTemplate {
            content: "test content".to_string(),
            file_path: temp_dir.path().join("src").join("commands").join("test.rs"),
            template_type: TemplateType::Command,
            name: "test".to_string(),
        };

        let generator = TemplateGenerator::new();
        let result = generator.write_template(&template);

        assert!(result.is_ok(), "Should write template successfully");
        assert!(template.file_path.exists(), "Template file should exist");

        let content = std::fs::read_to_string(&template.file_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello"), "Hello");
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("backup-manager"), "BackupManager");
        assert_eq!(to_pascal_case(""), "");
    }
}
