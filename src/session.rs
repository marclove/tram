//! Application session management and lifecycle.
//!
//! This module defines the TramSession struct and implements the starbase AppSession trait,
//! providing the core application lifecycle management including startup, analysis, and shutdown phases.

use async_trait::async_trait;
use starbase::AppSession;
use tracing::{debug, info, warn};
use tram_config::{ConfigChangeHandler, OutputFormat, TramConfig};
use tram_core::init_tracing;
use tram_workspace::{ProjectType, WorkspaceDetector};

/// Application session - directly implements starbase's AppSession.
#[derive(Clone, Debug)]
pub struct TramSession {
    pub config: TramConfig,
    pub workspace: WorkspaceDetector,
    pub workspace_root: Option<std::path::PathBuf>,
    pub project_type: Option<ProjectType>,
}

impl TramSession {
    pub fn with_config(config: TramConfig) -> tram_core::AppResult<Self> {
        Ok(Self {
            config,
            workspace: WorkspaceDetector::new()?,
            workspace_root: None,
            project_type: None,
        })
    }
}

#[async_trait]
impl AppSession for TramSession {
    async fn startup(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Initialize tracing before anything else
        let use_json = matches!(self.config.output_format, OutputFormat::Json);
        init_tracing(&self.config.log_level.to_string(), use_json)?;

        info!("Starting Tram CLI application");
        debug!("Configuration: {:?}", self.config);

        // Configuration validation is handled by schematic automatically

        // Detect workspace
        if let Ok(root) = self.workspace.detect_root() {
            self.workspace_root = Some(root.clone());
            self.project_type = ProjectType::detect(&root);
            info!("Detected workspace at: {}", root.display());
        } else {
            debug!("No workspace detected");
        }

        Ok(None)
    }

    async fn analyze(&mut self) -> tram_core::AppResult<Option<u8>> {
        // This phase would typically validate the environment,
        // check dependencies, build task graphs, etc.

        debug!("Analyzing workspace environment");

        // Skip workspace info for utility commands that need clean stdout
        let args: Vec<String> = std::env::args().collect();
        let is_utility_command = args.len() >= 2 && (args[1] == "completions" || args[1] == "man");

        if !is_utility_command {
            if let Some(root) = &self.workspace_root {
                eprintln!("Working in {} workspace", root.display());

                if let Some(project_type) = &self.project_type {
                    eprintln!("Detected {:?} project", project_type);
                    info!("Project type: {:?}", project_type);
                }
            }
        }

        Ok(None)
    }

    async fn shutdown(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Cleanup - save caches, write state, etc.
        debug!("Shutting down application");
        
        // Skip "Done!" message for utility commands that need clean stdout
        let args: Vec<String> = std::env::args().collect();
        let is_utility_command = args.len() >= 2 && (args[1] == "completions" || args[1] == "man");
        
        if !is_utility_command {
            eprintln!("Done!");
        }
        
        Ok(None)
    }
}

/// Handler for configuration changes during watch mode.
pub struct WatchConfigHandler;

#[async_trait::async_trait]
impl ConfigChangeHandler for WatchConfigHandler {
    async fn handle_config_change(&self, new_config: &TramConfig) {
        info!("🔄 Configuration reloaded successfully");
        info!("   Log level: {}", new_config.log_level);
        info!("   Output format: {}", new_config.output_format);
        info!("   Colors: {}", new_config.color);

        if let Some(workspace_root) = &new_config.workspace_root {
            info!("   Workspace root: {}", workspace_root.display());
        }
    }

    async fn handle_config_error(&self, error: Box<dyn std::error::Error + Send + Sync>) {
        warn!("❌ Configuration reload failed: {}", error);
        warn!("   Continuing with previous configuration");
    }
}
