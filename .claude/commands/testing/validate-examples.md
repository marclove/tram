---
allowed-tools: Bash(cargo build:*), Bash(cargo run:*), Bash(cargo test:*)
description: Build and test all example applications to ensure they work correctly
---

## Current Examples
- Examples directory: !`ls -la examples/`
- Example Cargo.toml files: !`find examples -name "Cargo.toml" -exec echo {} \; -exec head -5 {} \; -exec echo \;`

## Task
Validate all example applications to ensure they:
1. Compile successfully with current Tram dependencies
2. Run without errors (basic smoke test)
3. Show proper help text
4. Handle basic error cases gracefully
5. Follow documented patterns correctly

## Validation Steps

### For each example:
1. **Build check**: `cargo build --example {name}`
2. **Help test**: `cargo run --example {name} -- --help`
3. **Error handling**: Test with invalid arguments
4. **Basic functionality**: Run main command paths
5. **Documentation sync**: Ensure README matches actual behavior

### Pattern Validation:
- **CLI structure**: Verify clap derive usage is consistent
- **Session implementation**: Check AppSession trait usage
- **Error types**: Ensure miette integration works
- **Code style**: Consistent with Tram conventions

## Expected Tests
```bash
# Build all examples
for example in examples/*/; do
    name=$(basename "$example")
    echo "Building example: $name"
    cargo build --example "$name" || echo "FAILED: $name"
done

# Test help text
for example in examples/*/; do
    name=$(basename "$example")
    echo "Testing help for: $name"
    cargo run --example "$name" -- --help
done

# Test error handling
for example in examples/*/; do
    name=$(basename "$example")
    echo "Testing error handling for: $name"
    cargo run --example "$name" -- --invalid-flag 2>/dev/null || echo "OK: Error handled"
done
```

## Success Criteria
- All examples build without errors
- Help text is informative and consistent
- Error messages are user-friendly
- Examples demonstrate their intended patterns clearly
- README documentation matches actual behavior

Generate a report showing:
- Which examples are working correctly
- Any compilation or runtime errors
- Suggestions for improving examples that have issues
- Recommendations for additional examples needed