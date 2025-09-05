//! CLI examples and demonstrations.
//!
//! This module provides descriptions and guidance for the various example programs
//! that demonstrate different CLI patterns and features available in Tram.

use crate::cli::ExampleType;
use crate::session::TramSession;

/// Run an example demonstrating CLI patterns
pub async fn run_example(example: ExampleType, session: &TramSession) -> tram_core::AppResult<()> {
    match example {
        ExampleType::BasicCommand => {
            println!("=== Basic Command Example ===");
            println!("This example demonstrates fundamental clap + starbase integration patterns.");
            println!();
            println!("Key features demonstrated:");
            println!("• Command-line argument parsing with clap");
            println!("• Session-based lifecycle management with starbase");
            println!("• Error handling with miette");
            println!("• Structured logging and tracing");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example basic_command -- greet \"Your Name\"");
        }

        ExampleType::AsyncOperations => {
            println!("=== Async Operations Example ===");
            println!("This example demonstrates async patterns in CLI applications.");
            println!();
            println!("Key features demonstrated:");
            println!("• Long-running async tasks with progress");
            println!("• Concurrent operations with controlled parallelism");
            println!("• Timeout handling and graceful cancellation");
            println!("• Service monitoring and health checks");
            println!();
            println!("🔗 For full interactive example, run:");
            println!(
                "   cargo run --example async_operations -- download https://example.com/file output.txt"
            );
        }

        ExampleType::ConfigUsage => {
            println!("=== Configuration Management Example ===");
            println!("This example demonstrates Tram's configuration system.");
            println!();
            println!("Current configuration:");
            println!("  Log Level: {}", session.config.log_level);
            println!("  Output Format: {}", session.config.output_format);
            println!("  Colors: {}", session.config.color);
            if let Some(workspace_root) = &session.config.workspace_root {
                println!("  Workspace Root: {}", workspace_root.display());
            }
            println!();
            println!("Key features demonstrated:");
            println!("• Loading configuration from multiple sources");
            println!("• Hot reload with file watching");
            println!("• CLI argument overrides");
            println!("• Environment variable integration");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example config_usage -- show --sources");
        }

        ExampleType::ProgressIndicators => {
            println!("=== Progress Indicators Example ===");
            println!("This example demonstrates terminal UI components.");
            println!();
            println!("Key features demonstrated:");
            println!("• Progress bars with ETA calculations");
            println!("• Spinner animations for indeterminate progress");
            println!("• Multi-step progress tracking");
            println!("• Colored terminal output");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example progress_indicators -- progress-bar --steps 20");
        }

        ExampleType::InteractivePrompts => {
            println!("=== Interactive Prompts Example ===");
            println!("This example demonstrates user interaction patterns.");
            println!();
            println!("Key features demonstrated:");
            println!("• Text input with validation");
            println!("• Selection menus and multi-select");
            println!("• Password input (hidden)");
            println!("• Interactive wizards and forms");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example interactive_prompts -- wizard");
        }

        ExampleType::FileOperations => {
            println!("=== File Operations Example ===");
            println!("This example demonstrates file system utilities.");
            println!();
            println!("Key features demonstrated:");
            println!("• File reading, writing, and metadata");
            println!("• Directory traversal and search");
            println!("• Backup and validation operations");
            println!("• File watching and monitoring");
            println!();
            println!("🔗 For full interactive example, run:");
            println!("   cargo run --example file_operations -- basic-operations");
        }
    }

    println!();
    println!(
        "💡 All examples are also available as standalone programs in the examples/ directory."
    );

    Ok(())
}
