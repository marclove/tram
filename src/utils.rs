//! Utility functions for parsing and displaying types.
//!
//! This module contains helper functions for converting between string representations
//! and typed enums, as well as display formatting utilities.

use tram_core::{InitProjectType, TemplateType};

/// Parse project type string to InitProjectType.
pub fn parse_project_type(type_str: &str) -> InitProjectType {
    match type_str.to_lowercase().as_str() {
        "rust" => InitProjectType::Rust,
        "nodejs" | "node" | "js" => InitProjectType::NodeJs,
        "python" | "py" => InitProjectType::Python,
        "go" => InitProjectType::Go,
        "java" => InitProjectType::Java,
        _ => InitProjectType::Generic,
    }
}

/// Display name for project type.
pub fn project_type_display(project_type: &InitProjectType) -> &'static str {
    match project_type {
        InitProjectType::Rust => "Rust",
        InitProjectType::NodeJs => "Node.js",
        InitProjectType::Python => "Python",
        InitProjectType::Go => "Go",
        InitProjectType::Java => "Java",
        InitProjectType::Generic => "Generic",
    }
}

/// Parse template type string to TemplateType.
pub fn parse_template_type(type_str: &str) -> TemplateType {
    match type_str.to_lowercase().as_str() {
        "command" | "cmd" => TemplateType::Command,
        "config-section" | "config" => TemplateType::ConfigSection,
        "error-type" | "error" => TemplateType::ErrorType,
        "session-extension" | "session" => TemplateType::SessionExtension,
        _ => TemplateType::Command, // Default
    }
}

/// Display name for template type.
pub fn template_type_display(template_type: &TemplateType) -> &'static str {
    match template_type {
        TemplateType::Command => "Command",
        TemplateType::ConfigSection => "Config Section",
        TemplateType::ErrorType => "Error Type",
        TemplateType::SessionExtension => "Session Extension",
    }
}
