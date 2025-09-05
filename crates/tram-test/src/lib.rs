//! Testing utilities and fixtures for Tram CLI applications
//!
//! This crate provides utilities to make testing CLI applications built with Tram
//! easier and more comprehensive. It includes:
//!
//! - Test fixtures for common scenarios
//! - CLI command testing helpers
//! - Custom assertion macros
//! - Mock builders for configuration and workspace objects
//! - Integration test utilities
//!
//! # Examples
//!
//! ```rust
//! use tram_test::{TempDir, CliTestRunner};
//!
//! #[tokio::test]
//! async fn test_my_command() {
//!     let temp_dir = TempDir::new().unwrap();
//!     let runner = CliTestRunner::new("my-cli");
//!     
//!     let result = runner
//!         .arg("--config")
//!         .arg(temp_dir.path().join("config.toml"))
//!         .arg("test-command")
//!         .run()
//!         .await
//!         .unwrap();
//!         
//!     assert!(result.success());
//! }
//! ```

pub mod assertions;
pub mod cli;
pub mod fixtures;
pub mod mocks;

// Re-export commonly used items
// pub use assertions::*; // Uncomment when macros are used
pub use cli::*;
pub use fixtures::*;
pub use mocks::*;

// Re-export useful testing dependencies
pub use tempfile;
// pub use tokio_test; // Add tokio-test dependency if needed

/// Common result type for test utilities
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Test utilities version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
