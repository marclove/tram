---
allowed-tools: Write, MultiEdit, Edit, Grep
argument-hint: [pattern-name]
description: Implement a common CLI pattern with full code generation
---

## Available Patterns

**Core Patterns:**
- `session-lifecycle` - Full AppSession implementation with all phases
- `error-handling` - Comprehensive error types with miette diagnostics
- `config-loading` - Multi-source configuration with validation
- `command-routing` - Clean command dispatch from clap to handlers

**UI Patterns:**
- `progress-tracking` - Progress bars and status updates
- `interactive-prompts` - User input collection and validation
- `table-output` - Structured data display
- `logging-tracing` - Proper logging setup with different levels

**Advanced Patterns:**
- `plugin-architecture` - Extensible plugin loading system
- `daemon-mode` - Background service with signal handling
- `file-watching` - Monitor filesystem changes
- `concurrent-tasks` - Parallel execution with proper error handling

**Integration Patterns:**
- `shell-integration` - Completion generation and profile hooks
- `update-checking` - Version checking and update notifications
- `crash-reporting` - Error collection and reporting (opt-in)

## Current Codebase Context
- Project structure: !`find src -name "*.rs" -type f | head -10`
- Existing patterns: !`grep -r "impl.*Session" src/ || echo "No sessions found"`

## Task
Implement the `$1` pattern in the Tram starter kit:

1. **Analyze the pattern** - Understand what code structure is needed
2. **Create/update source files** - Add the pattern implementation
3. **Add documentation** - Include inline docs and README sections
4. **Create tests** - Unit and integration tests for the pattern
5. **Update examples** - Show how the pattern is used in practice
6. **Update CRATES.md** - Document which crate should contain this pattern

The implementation should:
- **Follow Tram conventions** - Use established error handling, session management
- **Be production-ready** - Include proper error cases and edge case handling
- **Be well-documented** - Clear explanations for developers who fork Tram
- **Be testable** - Include example tests that developers can reference
- **Integrate cleanly** - Work well with other Tram patterns

Look at @starbase/ submodule and @CRATES.md for architectural guidance.