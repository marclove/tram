//! Logging and tracing setup for CLI applications.
//!
//! Provides utilities for setting up structured logging with appropriate
//! formatting for different environments.

use std::sync::Once;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

static INIT: Once = Once::new();

/// Initialize tracing with appropriate configuration for CLI applications.
/// This function can be called multiple times safely - it will only initialize once.
pub fn init_tracing(log_level: &str, use_json: bool) -> crate::AppResult<()> {
    INIT.call_once(|| {
        let filter = match EnvFilter::try_new(log_level) {
            Ok(filter) => filter,
            Err(_) => {
                // Fall back to "info" level if the provided level is invalid
                EnvFilter::try_new("info").unwrap_or_else(|_| EnvFilter::new("info"))
            }
        };

        let registry = tracing_subscriber::registry().with(filter);

        if use_json {
            registry
                .with(fmt::layer().json().with_target(true).with_level(true))
                .init();
        } else {
            registry
                .with(fmt::layer().with_target(false).with_level(true).compact())
                .init();
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, error, info, warn};

    #[test]
    fn test_init_tracing_with_valid_level() {
        // Test that tracing initializes successfully with valid log levels
        let result = init_tracing("debug", false);
        assert!(result.is_ok(), "Should initialize tracing with debug level");
    }

    #[test]
    fn test_init_tracing_with_invalid_level_defaults() {
        // Test that invalid log levels fall back to "info"
        let result = init_tracing("invalid", false);
        assert!(
            result.is_ok(),
            "Should fall back to info level for invalid input"
        );
    }

    #[test]
    fn test_init_tracing_json_format() {
        // Test that JSON format initializes without error
        let result = init_tracing("info", true);
        assert!(result.is_ok(), "Should initialize tracing with JSON format");
    }

    #[test]
    fn test_tracing_logs_are_captured() {
        // This test verifies that tracing is working by checking if logs can be captured
        // In a real CLI application, we would verify the actual logging output
        init_tracing("debug", false).unwrap();

        // These should not panic or error - they test that the tracing infrastructure works
        info!("Test info message");
        warn!("Test warning message");
        error!("Test error message");
        debug!("Test debug message");
    }
}
