# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Vision

Tram is a foundational starter kit for building production-ready CLI applications in Rust. It combines the power of two excellent libraries:

- **clap** - Industry-standard command-line argument parsing with derive macros
- **starbase** - Developer experience framework providing async sessions, diagnostics, terminal UI, and utilities

**Meta-Development Focus**: We are currently doing meta-development of Tram itself. Our job is to iteratively develop this foundation as a great starter kit that developers will fork using GitHub's template repository functionality. We are **not** building a specific CLI application, but rather creating a robust, well-documented codebase that serves as an excellent starting point for others to build their own CLI applications.

The goal is to provide developers with a batteries-included starting point that demonstrates best practices for CLI development, eliminating the need to wire together common patterns from scratch.

## Development Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the CLI application
cargo run -- [arguments]

# Check code without building
cargo check

# Run clippy for linting
cargo clippy

# Format code
cargo fmt

# Run a specific test
cargo test test_name

# Build in release mode
cargo build --release
```

## Architecture Overview

Tram integrates clap and starbase through a structured approach:

### Command Structure (clap)
- Use clap's derive API to define CLI commands and arguments
- Organize subcommands logically with clear help documentation
- Leverage clap's validation and type conversion features

### Application Lifecycle (starbase)
Starbase provides a session-based architecture with four phases:

1. **Startup** - Initialize configuration, detect workspace, load plugins
2. **Analyze** - Process environment, build dependency graphs, validate inputs  
3. **Execute** - Run the core business logic based on parsed CLI arguments
4. **Shutdown** - Clean up resources, save state, handle graceful termination

### Key Starbase Components
- `starbase::App` - Main application container with diagnostics and tracing
- `starbase::AppSession` - Session trait for lifecycle management
- `starbase_console` - Terminal UI components (progress bars, prompts, tables)
- `starbase_shell` - Shell detection and profile management
- `starbase_utils` - File system, networking, and serialization utilities
- `starbase_events` - Event-driven architecture for extensibility

## Development Guidelines

### CLI Command Design
- Define commands using clap's derive macros for maintainability
- Group related functionality into subcommands
- Provide comprehensive help text and examples
- Use clap's validation features for input sanitization

### Session Implementation
```rust
#[derive(Clone, Debug)]
struct AppSession {
    config: Config,
    workspace_root: PathBuf,
}

#[async_trait]
impl starbase::AppSession for AppSession {
    async fn startup(&mut self) -> AppResult {
        // Load configuration, detect workspace
        Ok(None)
    }
    
    async fn analyze(&mut self) -> AppResult {
        // Process environment, validate state
        Ok(None)
    }
    
    // Execute phase runs your CLI logic
    async fn shutdown(&mut self) -> AppResult {
        // Cleanup resources
        Ok(None)
    }
}
```

### Error Handling
- Use `thiserror` for defining error types
- Implement `miette::Diagnostic` for rich error reporting
- Return `AppResult` (miette::Result) from all operations
- Leverage starbase's automatic error formatting

### Terminal UI
- Use `starbase_console` components for consistent UX
- Implement progress indicators for long-running operations
- Use structured output formats (JSON, YAML) when appropriate
- Follow terminal color and styling conventions

### Project Structure
- Keep the starter kit minimal but demonstrate key patterns
- Show integration between clap command parsing and starbase sessions
- Include examples of common CLI operations (file processing, configuration, etc.)
- Maintain clear separation between CLI interface and business logic
- Focus on creating reusable patterns rather than application-specific functionality
- Prioritize code quality, documentation, and developer experience over feature completeness

## Starbase Reference

The `starbase/` directory contains the starbase framework as a git submodule for reference. This codebase should be treated as **read-only** - it's included to help understand starbase's capabilities and implementation patterns, not for modification.

Key starbase crates to understand:
- `starbase` - Core application framework
- `starbase_console` - Terminal UI components  
- `starbase_utils` - File system and utility functions
- `starbase_shell` - Shell integration
- `starbase_events` - Event system
- `starbase_archive` - Archive handling

## Integration Patterns

When developers fork Tram to build their CLI applications, they should follow this pattern:

1. **Parse arguments with clap** - Define your CLI interface
2. **Initialize starbase App** - Set up diagnostics and tracing  
3. **Create your session** - Implement AppSession with your application state
4. **Run with session** - Let starbase manage the lifecycle while executing your CLI logic
5. **Handle errors gracefully** - Use miette diagnostics for user-friendly error messages

This approach provides the parsing power of clap with the developer experience and utilities of starbase, creating a robust foundation for CLI applications.

## Meta-Development Guidelines

As we develop Tram itself:

- **Demonstrate patterns, not solutions** - Show how to integrate clap and starbase effectively
- **Keep examples generic** - Use placeholder functionality that illustrates concepts without being application-specific  
- **Document thoroughly** - Each pattern should be well-explained for developers who will fork this
- **Test the developer experience** - Ensure the starter kit is easy to understand and extend
- **Maintain template quality** - Code should be production-ready and follow Rust best practices