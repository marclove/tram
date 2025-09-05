//! Template generation for common CLI patterns using Handlebars.
//!
//! Provides utilities for generating boilerplate code for common CLI patterns,
//! helping developers quickly add new functionality to their applications.

use crate::{AppResult, TramError};
use handlebars::Handlebars;
use serde_json::{Value, json};
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

/// Service for generating templates from common CLI patterns using Handlebars.
pub struct TemplateGenerator {
    /// Handlebars instance for template rendering
    handlebars: Handlebars<'static>,
}

impl TemplateGenerator {
    pub fn new() -> AppResult<Self> {
        let mut handlebars = Handlebars::new();

        // Register built-in templates
        Self::register_templates(&mut handlebars)?;

        Ok(Self { handlebars })
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
        let content = self.render_template(config)?;
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

    /// Register all built-in templates with Handlebars.
    fn register_templates(handlebars: &mut Handlebars) -> AppResult<()> {
        // Register command template
        handlebars
            .register_template_string("command", include_str!("templates/command.hbs"))
            .map_err(|e| TramError::InvalidConfig {
                message: format!("Failed to register command template: {}", e),
            })?;

        // Register config section template
        handlebars
            .register_template_string(
                "config_section",
                include_str!("templates/config_section.hbs"),
            )
            .map_err(|e| TramError::InvalidConfig {
                message: format!("Failed to register config section template: {}", e),
            })?;

        // Register error type template
        handlebars
            .register_template_string("error_type", include_str!("templates/error_type.hbs"))
            .map_err(|e| TramError::InvalidConfig {
                message: format!("Failed to register error type template: {}", e),
            })?;

        // Register session extension template
        handlebars
            .register_template_string(
                "session_extension",
                include_str!("templates/session_extension.hbs"),
            )
            .map_err(|e| TramError::InvalidConfig {
                message: format!("Failed to register session extension template: {}", e),
            })?;

        Ok(())
    }

    /// Render template using Handlebars with the provided configuration.
    fn render_template(&self, config: &TemplateConfig) -> AppResult<String> {
        let template_name = self.get_template_name(&config.template_type);
        let context = self.build_template_context(config);

        self.handlebars
            .render(template_name, &context)
            .map_err(|e| {
                TramError::InvalidConfig {
                    message: format!("Failed to render {} template: {}", template_name, e),
                }
                .into()
            })
    }

    /// Get the template name for a given template type.
    fn get_template_name(&self, template_type: &TemplateType) -> &'static str {
        match template_type {
            TemplateType::Command => "command",
            TemplateType::ConfigSection => "config_section",
            TemplateType::ErrorType => "error_type",
            TemplateType::SessionExtension => "session_extension",
        }
    }

    /// Build the context data for template rendering.
    fn build_template_context(&self, config: &TemplateConfig) -> Value {
        let name = &config.name;
        let name_pascal = to_pascal_case(name);
        let name_upper = name.to_uppercase();
        let description = config
            .parameters
            .get("description")
            .unwrap_or(&format!("{} functionality", name))
            .clone();

        json!({
            "name": name,
            "name_pascal": name_pascal,
            "name_upper": name_upper,
            "description": description,
            "parameters": config.parameters
        })
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
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default TemplateGenerator")
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

        let generator = TemplateGenerator::new().unwrap();
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

        let generator = TemplateGenerator::new().unwrap();
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

        let generator = TemplateGenerator::new().unwrap();
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

        let generator = TemplateGenerator::new().unwrap();
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

        let generator = TemplateGenerator::new().unwrap();
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
