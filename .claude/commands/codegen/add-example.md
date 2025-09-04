---
allowed-tools: Bash(cargo new:*), Bash(mkdir:*), Write, MultiEdit, Edit
argument-hint: [example-name] [example-type]
description: Generate a complete example CLI application showcasing Tram features
---

## Current Examples
- Existing examples: !`ls -la examples/`
- README examples section: !`grep -A 20 "## Examples" README.md || echo "No examples section found"`

## Task
Create a complete example CLI application called `$1` of type `$2` that showcases specific Tram features.

### Example Types Available:
- **basic** - Minimal clap + starbase integration
- **config-heavy** - Configuration loading from multiple sources
- **interactive** - Rich terminal UI with prompts and progress bars
- **multi-command** - Complex CLI with subcommands and shared state
- **plugin-system** - Extensible architecture with plugin loading
- **daemon-mode** - Long-running service with signal handling
- **file-processor** - CLI that processes files with glob patterns
- **api-client** - CLI that interacts with REST APIs

### What to create:

1. **Create directory structure**: `examples/$1/`
2. **Cargo.toml** with:
   - Appropriate Tram dependencies for the example type
   - Clear example description
   - Binary configuration
3. **Source code** (`src/main.rs`, `src/cli.rs`, `src/session.rs`) demonstrating:
   - Proper clap command definitions
   - StarBase session lifecycle implementation
   - Error handling with miette
   - Features specific to the example type
4. **README.md** with:
   - What the example demonstrates
   - How to run it
   - Key patterns illustrated
   - Code walkthrough
5. **Test files** showing how to test the CLI patterns
6. **Update main README.md** to include the new example

The example should be:
- **Self-contained** - Runnable with `cargo run --example $1`
- **Well-documented** - Clear comments explaining Tram patterns
- **Realistic** - Demonstrate real-world usage patterns
- **Template-ready** - Code that developers can copy and adapt

Look at @starbase/examples/ for inspiration on structure and patterns.