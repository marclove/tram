//! Project initialization utilities for CLI applications.
//!
//! Provides functionality for creating new projects with templates
//! and interactive prompts.

use crate::{AppResult, TramError};
use std::fs;
use std::path::PathBuf;

/// Supported project types for initialization.
#[derive(Debug, Clone, PartialEq)]
pub enum InitProjectType {
    Rust,
    NodeJs,
    Python,
    Go,
    Java,
    Generic,
}

/// Configuration for project initialization.
#[derive(Debug, Clone)]
pub struct InitConfig {
    pub name: String,
    pub path: PathBuf,
    pub project_type: InitProjectType,
    pub description: Option<String>,
    pub author: Option<String>,
}

/// Service for creating new projects.
pub struct ProjectInitializer;

impl ProjectInitializer {
    pub fn new() -> Self {
        Self
    }

    /// Create a new project with the given configuration.
    /// This is the main behavior users expect when initializing a project.
    pub fn create_project(&self, config: &InitConfig) -> AppResult<()> {
        // Behavior: Should create project directory
        if config.path.exists() {
            return Err(TramError::InvalidConfig {
                message: format!("Directory {} already exists", config.path.display()),
            }
            .into());
        }

        fs::create_dir_all(&config.path).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to create project directory: {}", e),
        })?;

        // Behavior: Should create appropriate project files based on type
        self.create_project_files(config)?;

        Ok(())
    }

    /// Create the basic project structure based on project type.
    fn create_project_files(&self, config: &InitConfig) -> AppResult<()> {
        match config.project_type {
            InitProjectType::Rust => self.create_rust_project(config),
            InitProjectType::NodeJs => self.create_nodejs_project(config),
            InitProjectType::Python => self.create_python_project(config),
            InitProjectType::Go => self.create_go_project(config),
            InitProjectType::Java => self.create_java_project(config),
            InitProjectType::Generic => self.create_generic_project(config),
        }
    }

    fn create_rust_project(&self, config: &InitConfig) -> AppResult<()> {
        // Create Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
{}

[dependencies]
"#,
            config.name,
            config
                .description
                .as_ref()
                .map(|d| format!("description = \"{}\"", d))
                .unwrap_or_default()
        );

        let cargo_path = config.path.join("Cargo.toml");
        fs::write(cargo_path, cargo_toml).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write Cargo.toml: {}", e),
        })?;

        // Create src directory and main.rs
        let src_dir = config.path.join("src");
        fs::create_dir(&src_dir).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to create src directory: {}", e),
        })?;

        let main_rs = r#"fn main() {
    println!("Hello, world!");
}
"#;

        let main_path = src_dir.join("main.rs");
        fs::write(main_path, main_rs).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write main.rs: {}", e),
        })?;

        Ok(())
    }

    fn create_nodejs_project(&self, config: &InitConfig) -> AppResult<()> {
        // Create package.json
        let package_json = format!(
            r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "{}",
  "main": "index.js",
  "scripts": {{
    "start": "node index.js"
  }}
}}
"#,
            config.name,
            config.description.as_deref().unwrap_or("")
        );

        let package_path = config.path.join("package.json");
        fs::write(package_path, package_json).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write package.json: {}", e),
        })?;

        // Create index.js
        let index_js = r#"console.log('Hello, world!');
"#;

        let index_path = config.path.join("index.js");
        fs::write(index_path, index_js).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write index.js: {}", e),
        })?;

        Ok(())
    }

    fn create_python_project(&self, config: &InitConfig) -> AppResult<()> {
        // Create pyproject.toml
        let pyproject_toml = format!(
            r#"[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "{}"
version = "0.0.1"
description = "{}"

[project.scripts]
{} = "{}:main"
"#,
            config.name,
            config.description.as_deref().unwrap_or(""),
            config.name,
            config.name.replace("-", "_")
        );

        let pyproject_path = config.path.join("pyproject.toml");
        fs::write(pyproject_path, pyproject_toml).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write pyproject.toml: {}", e),
        })?;

        // Create main module
        let main_py = r#"def main():
    print("Hello, world!")

if __name__ == "__main__":
    main()
"#;

        let main_path = config
            .path
            .join(format!("{}.py", config.name.replace("-", "_")));
        fs::write(main_path, main_py).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write main module: {}", e),
        })?;

        Ok(())
    }

    fn create_go_project(&self, config: &InitConfig) -> AppResult<()> {
        // Create go.mod
        let go_mod = format!("module {}\n\ngo 1.21\n", config.name);

        let go_mod_path = config.path.join("go.mod");
        fs::write(go_mod_path, go_mod).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write go.mod: {}", e),
        })?;

        // Create main.go
        let main_go = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}
"#;

        let main_path = config.path.join("main.go");
        fs::write(main_path, main_go).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write main.go: {}", e),
        })?;

        Ok(())
    }

    fn create_java_project(&self, _config: &InitConfig) -> AppResult<()> {
        // For simplicity, create a basic project structure
        // In a real implementation, this would use Maven/Gradle templates
        Ok(())
    }

    fn create_generic_project(&self, config: &InitConfig) -> AppResult<()> {
        // Create a simple README
        let readme = format!(
            "# {}\n\n{}\n",
            config.name,
            config.description.as_deref().unwrap_or("A new project")
        );

        let readme_path = config.path.join("README.md");
        fs::write(readme_path, readme).map_err(|e| TramError::InvalidConfig {
            message: format!("Failed to write README.md: {}", e),
        })?;

        Ok(())
    }
}

impl Default for ProjectInitializer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-project");

        let config = InitConfig {
            name: "test-project".to_string(),
            path: project_path.clone(),
            project_type: InitProjectType::Rust,
            description: Some("A test project".to_string()),
            author: None,
        };

        let initializer = ProjectInitializer::new();
        let result = initializer.create_project(&config);

        assert!(result.is_ok(), "Should create Rust project successfully");
        assert!(project_path.exists(), "Project directory should exist");
        assert!(
            project_path.join("Cargo.toml").exists(),
            "Cargo.toml should exist"
        );
        assert!(
            project_path.join("src").exists(),
            "src directory should exist"
        );
        assert!(
            project_path.join("src/main.rs").exists(),
            "main.rs should exist"
        );
    }

    #[test]
    fn test_create_nodejs_project() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-node-project");

        let config = InitConfig {
            name: "test-node-project".to_string(),
            path: project_path.clone(),
            project_type: InitProjectType::NodeJs,
            description: Some("A test Node.js project".to_string()),
            author: None,
        };

        let initializer = ProjectInitializer::new();
        let result = initializer.create_project(&config);

        assert!(result.is_ok(), "Should create Node.js project successfully");
        assert!(project_path.exists(), "Project directory should exist");
        assert!(
            project_path.join("package.json").exists(),
            "package.json should exist"
        );
        assert!(
            project_path.join("index.js").exists(),
            "index.js should exist"
        );
    }

    #[test]
    fn test_create_project_fails_when_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("existing-project");

        // Create the directory first
        fs::create_dir(&project_path).unwrap();

        let config = InitConfig {
            name: "existing-project".to_string(),
            path: project_path,
            project_type: InitProjectType::Rust,
            description: None,
            author: None,
        };

        let initializer = ProjectInitializer::new();
        let result = initializer.create_project(&config);

        assert!(result.is_err(), "Should fail when directory already exists");
    }
}
