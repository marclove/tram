//! Mock builders for common objects

use std::path::PathBuf;
use tram_config::{LogLevel, OutputFormat, TramConfig};
use tram_workspace::ProjectType;

/// Builder for creating mock TramConfig instances
#[derive(Debug, Default)]
pub struct MockConfigBuilder {
    log_level: Option<LogLevel>,
    output_format: Option<OutputFormat>,
    color: Option<bool>,
    workspace_root: Option<PathBuf>,
}

impl MockConfigBuilder {
    /// Create a new mock config builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Set the output format
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    /// Set color support
    pub fn color(mut self, enabled: bool) -> Self {
        self.color = Some(enabled);
        self
    }

    /// Set workspace root
    pub fn workspace_root(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace_root = Some(path.into());
        self
    }

    /// Build the mock configuration
    pub fn build(self) -> TramConfig {
        let mut config = TramConfig::default();

        if let Some(log_level) = self.log_level {
            config.log_level = log_level;
        }

        if let Some(output_format) = self.output_format {
            config.output_format = output_format;
        }

        if let Some(color) = self.color {
            config.color = color;
        }

        if let Some(workspace_root) = self.workspace_root {
            config.workspace_root = Some(workspace_root);
        }

        config
    }
}

/// Mock workspace detector for testing
#[derive(Debug)]
pub struct MockWorkspaceDetector {
    root: Option<PathBuf>,
    project_type: Option<ProjectType>,
}

impl MockWorkspaceDetector {
    /// Create a new mock workspace detector
    pub fn new() -> Self {
        Self {
            root: None,
            project_type: None,
        }
    }

    /// Set the workspace root
    pub fn with_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.root = Some(root.into());
        self
    }

    /// Set the project type
    pub fn with_project_type(mut self, project_type: ProjectType) -> Self {
        self.project_type = Some(project_type);
        self
    }

    /// Get the mocked workspace root
    pub fn detect_root(&self) -> Result<PathBuf, tram_core::TramError> {
        match &self.root {
            Some(root) => Ok(root.clone()),
            None => Err(tram_core::TramError::WorkspaceNotFound),
        }
    }

    /// Get the mocked project type
    pub fn get_project_type(&self) -> Option<&ProjectType> {
        self.project_type.as_ref()
    }
}

impl Default for MockWorkspaceDetector {
    fn default() -> Self {
        Self::new()
    }
}
