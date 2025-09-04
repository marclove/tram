---
allowed-tools: Bash(cargo doc:*), Read, Edit, MultiEdit
description: Create comprehensive API documentation for all Tram crates
---

## Current Documentation Status
- Crate structure: !`find crates -name "lib.rs" 2>/dev/null || echo "No crates found"`
- Existing rustdocs: !`find . -name "*.rs" -exec grep -l "///" {} \; 2>/dev/null | head -5`
- Doc generation test: !`cargo doc --no-deps --quiet 2>&1 | head -10`

## API Documentation Requirements

### 1. Module-Level Documentation
- **Crate purpose**: Clear explanation of what each crate does
- **Usage patterns**: How to integrate with other Tram crates
- **Examples**: Working code examples for main use cases
- **Feature flags**: Document optional functionality

### 2. Type Documentation
- **Structs**: Purpose, usage patterns, field meanings
- **Enums**: All variants with clear explanations
- **Traits**: Implementation requirements and examples
- **Functions**: Parameters, return values, error conditions

### 3. Integration Examples
- **Clap integration**: How to use with clap derive macros
- **Starbase integration**: Session lifecycle patterns
- **Error handling**: miette diagnostic usage
- **Configuration**: Multi-source config examples

## Task
Generate comprehensive API documentation:

1. **Audit current rustdoc coverage**:
   ```bash
   # Check current documentation coverage
   cargo doc --document-private-items --no-deps
   
   # Identify undocumented items
   cargo rustdoc -- -D missing_docs
   ```

2. **Create/update module docs**:
   - Add comprehensive lib.rs documentation for each crate
   - Include usage examples and integration patterns
   - Document feature flags and optional dependencies

3. **Document public APIs**:
   - Ensure all public structs, enums, traits have docs
   - Add examples for complex types
   - Document error conditions and edge cases

4. **Add integration examples**:
   - Show real-world usage patterns
   - Demonstrate clap + starbase integration
   - Include error handling examples

5. **Generate final documentation**:
   ```bash
   # Generate docs with examples
   cargo doc --no-deps --examples
   
   # Test that all examples compile
   cargo test --doc
   ```

## Documentation Standards
- **Complete coverage**: All public APIs must have rustdoc comments
- **Working examples**: All doc examples must compile and run
- **Clear explanations**: Focus on developer understanding
- **Integration focus**: Show how pieces work together
- **Error documentation**: Document failure modes and recovery

## Success Criteria
- `cargo doc` completes without warnings
- All public APIs have comprehensive documentation
- Doc examples compile and run correctly
- Generated docs provide clear guidance for Tram users
- Cross-references between crates are working