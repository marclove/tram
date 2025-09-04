//! Core utilities and patterns for Tram CLI applications.
//!
//! This crate provides common utilities for building CLI applications with
//! clap and starbase, without unnecessary abstractions.

pub mod error;
pub mod logging;
pub mod project_init;

pub use error::*;
pub use logging::*;
pub use project_init::*;

// Re-export commonly used types for convenience
pub use miette::{IntoDiagnostic, Result as AppResult, miette};
pub use starbase::AppSession;
