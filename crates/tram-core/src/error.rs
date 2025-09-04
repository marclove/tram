//! Common error types for CLI applications.
//!
//! Provides error types commonly needed in CLI applications with good
//! diagnostic messages.

use miette::Diagnostic;
use thiserror::Error;

/// Common CLI application errors with good user-facing diagnostics.
#[derive(Debug, Diagnostic, Error)]
pub enum TramError {
    #[error("Configuration file not found: {path}")]
    #[diagnostic(
        code(tram::config_not_found),
        help("Run with --help to see configuration options")
    )]
    ConfigNotFound { path: String },

    #[error("Invalid configuration: {message}")]
    #[diagnostic(code(tram::invalid_config))]
    InvalidConfig { message: String },

    #[error("Workspace not found")]
    #[diagnostic(
        code(tram::workspace_not_found),
        help("Make sure you're running this command from within a project")
    )]
    WorkspaceNotFound,
}
