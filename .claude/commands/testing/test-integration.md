---
allowed-tools: Bash(cargo test:*), Bash(cargo build:*), Bash(cargo check:*)
description: Run comprehensive integration tests for clap + starbase integration
---

## System Info
- Current directory: !`pwd`
- Cargo version: !`cargo --version`
- Build status: !`cargo check --all 2>&1`
- Test status: !`cargo test --no-run --all 2>&1`

## Integration Test Areas

### 1. Core Integration
- Clap command parsing → starbase session flow
- Argument validation and error propagation
- Session lifecycle (startup → analyze → execute → shutdown)
- Error handling through the entire stack

### 2. Configuration Integration
- CLI args override config files
- Environment variables integration
- Multiple config source merging
- Config validation with clap constraints

### 3. Multi-Command Applications
- Subcommand routing to session methods
- Shared state between commands
- Command-specific error handling
- Help text generation

## Task
Run comprehensive integration testing:

1. **Build all crates**: Ensure clean compilation
2. **Unit tests**: Run tests for each individual crate
3. **Integration tests**: Focus on clap + starbase interaction points
4. **Example tests**: Build and basic smoke test all examples
5. **Error path testing**: Verify error propagation works correctly
6. **Performance checks**: Ensure integration doesn't add significant overhead

## Test Commands to Run
```bash
# Clean build
cargo clean && cargo build --all

# Run all tests with verbose output
cargo test --all --verbose

# Test examples build
cargo build --examples

# Integration-specific tests
cargo test integration --all

# Check that error handling works
cargo test error --all
```

## Success Criteria
- All tests pass
- Examples build successfully
- Error messages are helpful and user-friendly
- No performance regressions from integration
- Documentation examples compile and run

Provide a summary report of any failures or issues, with specific guidance on what needs to be fixed to ensure smooth clap + starbase integration.