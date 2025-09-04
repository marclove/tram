# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Vision

Tram is a foundational starter kit for building production-ready CLI applications in Rust. It combines the power of two excellent libraries:

- **clap** - Industry-standard command-line argument parsing with derive macros
- **starbase** - Developer experience framework providing async sessions, diagnostics, terminal UI, and utilities

**Meta-Development Focus**: We are currently doing meta-development of Tram itself. Our job is to iteratively develop this foundation as a great starter kit that developers will fork using GitHub's template repository functionality. We are **not** building a specific CLI application, but rather creating a robust, well-documented codebase that serves as an excellent starting point for others to build their own CLI applications.

The goal is to provide developers with a batteries-included starting point that demonstrates best practices for CLI development, eliminating the need to wire together common patterns from scratch.

## Development Commands

Tram uses **just** for development commands and **moon** for task orchestration. Always prefer just commands over direct cargo usage:

```bash
# Primary development workflow
just check              # Quick check: format, lint, build, test
just build [CRATE]      # Build workspace or specific crate via moon
just test [CRATE]       # Run tests via moon
just run [ARGS]         # Run the CLI application

# Project management
just setup              # Install toolchain and dependencies
just new-crate NAME     # Create new crate with moon configuration
just clean              # Clean cargo + moon caches
just watch              # Watch for changes and run checks

# Advanced commands
just perf               # Performance check with release build
just release-check      # Full release preparation pipeline
just graph              # Generate project dependency graph
just demo [COMMAND]     # Run CLI demos

# Show all available commands
just --list
```

**Important**: Use `just` commands instead of direct `cargo` commands to leverage moon's task orchestration, caching, and dependency management.

## Architecture Overview

Tram integrates clap and starbase through a structured multi-crate architecture:

### Multi-Crate Workspace Structure
- **`src/main.rs`** - Main application binary with clap + starbase integration
- **`tram-core`** - Core types, error handling, and shared utilities
- **`tram-config`** - Configuration management with multiple source support
- **`tram-workspace`** - Workspace detection and project type identification

### Command Structure (clap)
- Use clap's derive API to define CLI commands and arguments directly in `src/main.rs`
- Organize subcommands logically with clear help documentation
- Leverage clap's validation and type conversion features
- Integrate global options using `#[command(flatten)]`

### Application Lifecycle (starbase)
Starbase provides a session-based architecture with four phases:

1. **Startup** - Initialize configuration, detect workspace, load plugins
2. **Analyze** - Process environment, build dependency graphs, validate inputs
3. **Execute** - Run the core business logic based on parsed CLI arguments
4. **Shutdown** - Clean up resources, save state, handle graceful termination

### Key Starbase Components
- `starbase::App` - Main application container with diagnostics and tracing
- `starbase::AppSession` - Session trait for lifecycle management (implement directly, no wrappers)
- `starbase_console` - Terminal UI components (progress bars, prompts, tables)
- `starbase_shell` - Shell detection and profile management
- `starbase_utils` - File system, networking, and serialization utilities
- `starbase_events` - Event-driven architecture for extensibility

### Task Orchestration (Moon)
- Moon manages tasks across the multi-crate workspace
- Intelligent dependency resolution between crates
- Efficient caching and incremental builds
- Parallel task execution where possible

## Development Guidelines

### CLI Command Design
- Define commands using clap's derive macros for maintainability
- Group related functionality into subcommands
- Provide comprehensive help text and examples
- Use clap's validation features for input sanitization

### Session Implementation
```rust
#[derive(Clone, Debug)]
pub struct TramSession {
    pub config: tram_config::Config,
    pub workspace: tram_workspace::WorkspaceDetector,
    pub workspace_root: Option<PathBuf>,
    pub project_type: Option<tram_workspace::ProjectType>,
}

#[async_trait]
impl AppSession for TramSession {
    async fn startup(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Load configuration from multiple sources
        self.config = Config::load_from_args(&cli_args)?;
        self.config.validate()?;

        // Detect workspace and project type
        if let Ok(root) = self.workspace.detect_root() {
            self.workspace_root = Some(root.clone());
            self.project_type = ProjectType::detect(&root);
        }

        Ok(None)
    }

    async fn analyze(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Process environment, validate state, show workspace info
        if let Some(root) = &self.workspace_root {
            println!("Working in {} workspace", root.display());
        }
        Ok(None)
    }

    async fn shutdown(&mut self) -> tram_core::AppResult<Option<u8>> {
        // Cleanup resources, save state
        println!("Done!");
        Ok(None)
    }
}

impl TramSession {
    pub fn new() -> tram_core::AppResult<Self> {
        Ok(Self {
            config: Config::default(),
            workspace: WorkspaceDetector::new()?,
            workspace_root: None,
            project_type: None,
        })
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
- Use multi-crate workspace structure for organizing complex applications
- Maintain clear separation between CLI interface and business logic
- Focus on creating reusable patterns rather than application-specific functionality
- Prioritize code quality, documentation, and developer experience over feature completeness
- Use moon task orchestration - Never use direct cargo commands in development workflows
- Implement quality checks - Claude Code hooks prevent warnings from being committed

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
- **Use Edition 2024** - Always use Rust Edition 2024 for this project
- **Leverage moon orchestration** - All development tasks should go through moon for consistency
- **Implement automated quality checks** - Use Claude Code hooks to prevent issues from being introduced

## Development Tooling

### Moon Task Runner
- **Always use moon commands through just recipes** - Never bypass moon task orchestration
- **Leverage intelligent caching** - Moon caches build artifacts and task results
- **Respect task dependencies** - Moon ensures proper build order across crates
- **Use parallel execution** - Moon runs independent tasks in parallel

### Claude Code Integration
- **PostToolUse hooks** - Automatically check for Rust warnings after file edits
- **Quality gates** - Prevent compiler warnings from slipping through
- **Immediate feedback** - Get compilation issues reported instantly to fix them

### Workspace Management
- **Multi-crate structure** - Organize functionality into focused crates
- **Consistent patterns** - Each crate follows the same moon.yml structure
- **Shared dependencies** - Use workspace-level dependency management
- **Easy extension** - Use `just new-crate NAME` to add functionality
