---
allowed-tools: Grep, Read, Bash(find:*), Bash(grep:*)
description: Verify codebase follows consistent patterns and conventions
---

## Current Codebase Structure
- Source files: !`find . -name "*.rs" -not -path "./starbase/*" -not -path "./target/*"`
- Cargo.toml files: !`find . -name "Cargo.toml" -not -path "./starbase/*" -not -path "./target/*"`
- Documentation files: !`find . -name "*.md" -not -path "./starbase/*" -not -path "./target/*"`

## Consistency Checks

### 1. Code Patterns
- **Error handling**: All crates use miette + thiserror consistently
- **Session traits**: AppSession implementations follow the same pattern
- **Import organization**: Consistent use of clap and starbase imports
- **Async patterns**: Proper async/await usage throughout

### 2. Documentation Standards
- **README format**: All crates have similar README structure
- **Code comments**: Consistent documentation style
- **Example format**: Uniform code example presentation
- **API docs**: Complete rustdoc coverage

### 3. Configuration
- **Cargo.toml**: Consistent metadata, dependencies, features
- **Version numbers**: All internal dependencies are aligned
- **Feature flags**: Consistent naming and organization
- **Build settings**: Uniform optimization and target settings

### 4. Testing Standards
- **Test organization**: Similar test structure across crates
- **Test naming**: Consistent naming conventions
- **Test coverage**: Appropriate test coverage levels
- **Integration patterns**: Similar integration test approaches

## Task
Analyze the codebase for consistency issues:

1. **Check imports and dependencies**:
   - Look for inconsistent clap usage patterns
   - Verify starbase integration is uniform
   - Check for duplicate dependencies

2. **Validate error handling patterns**:
   - All error types derive from thiserror
   - Miette diagnostic integration is consistent
   - Error conversion patterns are uniform

3. **Review session implementations**:
   - AppSession trait usage follows patterns
   - Lifecycle methods are implemented consistently
   - State management approaches are similar

4. **Check documentation quality**:
   - All public APIs have rustdoc comments
   - README files follow template structure
   - Code examples are accurate and consistent

5. **Verify naming conventions**:
   - Crate names follow `tram-*` pattern
   - Module organization is consistent
   - Public API naming is uniform

## Report Format
Generate a detailed report with:
- **✅ Consistent areas** - Patterns that are well-maintained
- **⚠️ Minor issues** - Small inconsistencies to fix
- **❌ Major issues** - Significant pattern violations
- **📝 Recommendations** - Suggestions for improving consistency

Include specific file references and code snippets for each issue found.