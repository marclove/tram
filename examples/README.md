# Tram Examples

This directory contains practical examples demonstrating how to use Tram as a foundation for building CLI applications with clap and starbase.

## Available Examples

### Basic Patterns
- [`basic_command.rs`](basic_command.rs) - Simple CLI with subcommands
- [`async_operations.rs`](async_operations.rs) - Async command execution patterns
- [`config_usage.rs`](config_usage.rs) - Configuration loading and validation

### Advanced Patterns
- [`progress_indicators.rs`](progress_indicators.rs) - Progress bars and spinners
- [`interactive_prompts.rs`](interactive_prompts.rs) - User interaction patterns
- [`file_operations.rs`](file_operations.rs) - File system operations

## Running Examples

Each example is a standalone Rust file that can be run with:

```bash
cargo run --example basic_command
cargo run --example async_operations
# ... etc
```

## Learning Path

1. **Start with `basic_command.rs`** - Learn the fundamental clap + starbase integration
2. **Move to `config_usage.rs`** - Understand configuration management
3. **Try `async_operations.rs`** - See async patterns in action
4. **Explore the advanced examples** - Learn UI and interaction patterns

## Integration Patterns

These examples demonstrate the core Tram philosophy:
- **Parse with clap** - Define your CLI interface declaratively
- **Execute with starbase** - Leverage session-based lifecycle management
- **Handle errors gracefully** - Use miette for user-friendly diagnostics

Each example shows a different aspect of building production-ready CLI applications.