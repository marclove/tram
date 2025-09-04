---
allowed-tools: Read, Edit, MultiEdit, Grep
argument-hint: [file-path] [pattern-name]
description: Refactor existing code to follow established Tram patterns
---

## Current File Analysis
- Target file: @$1
- File type: !`file $1 2>/dev/null || echo "File not found: $1"`
- Current patterns: !`grep -E "(impl|struct|enum|fn)" $1 | head -10`

## Available Tram Patterns

### Session-Based (`session-lifecycle`)
- **AppSession trait**: Proper startup/analyze/execute/shutdown flow
- **State management**: Clone-safe session state
- **Error propagation**: Session methods return AppResult

### Error Handling (`error-handling`)
- **thiserror**: Derive Error for custom error types
- **miette**: Diagnostic integration for user-friendly errors
- **Error conversion**: From trait implementations for error chaining

### Configuration (`config-loading`)
- **Multi-source**: CLI args, env vars, config files
- **Validation**: Config struct with validation methods
- **Defaults**: Sensible default values with override capability

### Command Routing (`command-routing`)
- **clap integration**: Derive Parser and Subcommand
- **Handler dispatch**: Clean routing from CLI to business logic
- **Context passing**: Session and config propagation

### Async Patterns (`async-patterns`)
- **Tokio integration**: Proper runtime usage
- **Error handling**: Result propagation in async contexts
- **Concurrent execution**: Parallel task management

## Task
Refactor `$1` to follow the `$2` pattern:

### Analysis Phase:
1. **Read current implementation**
2. **Identify pattern violations**:
   - Missing trait implementations
   - Inconsistent error handling
   - Non-standard async usage
   - Configuration anti-patterns

3. **Plan refactoring steps**:
   - What needs to change
   - Dependencies to add/remove
   - API compatibility considerations

### Refactoring Phase:
1. **Update imports**: Add necessary use statements
2. **Modify types**: Update structs/enums to match patterns
3. **Implement traits**: Add required trait implementations
4. **Update functions**: Refactor to use standard patterns
5. **Fix error handling**: Ensure consistent error propagation

### Pattern-Specific Refactoring:

#### For `session-lifecycle`:
- Implement `AppSession` trait with all four phases
- Ensure session is `Clone + Send + Sync`
- Use `AppResult` return types
- Proper async/await usage

#### For `error-handling`:
- Create custom error enum with `thiserror::Error`
- Add `miette::Diagnostic` derive
- Implement `From` traits for error conversion
- Use `Result<T, YourError>` or `AppResult`

#### For `config-loading`:
- Define config struct with `serde` derives
- Implement config loading from multiple sources
- Add validation methods
- Use `clap` integration for CLI overrides

#### For `command-routing`:
- Use `clap::Parser` and `clap::Subcommand` derives
- Create clean handler functions
- Pass session/config through routing
- Proper error propagation from handlers

### Validation Phase:
1. **Compile check**: Ensure code still builds
2. **Test compatibility**: Run existing tests
3. **Pattern compliance**: Verify pattern is correctly implemented
4. **Integration check**: Ensure it works with other Tram components

## Success Criteria
- Code follows the specified Tram pattern correctly
- All existing functionality is preserved
- Tests pass (or are updated appropriately)
- Code is more maintainable and consistent
- Integration with other Tram patterns is clean

## Pattern Validation Checklist

### Session-based:
- [ ] Implements `AppSession` trait
- [ ] All phases have meaningful implementations
- [ ] Session state is properly managed
- [ ] Error handling is consistent

### Error handling:
- [ ] Custom error types use `thiserror`
- [ ] `miette::Diagnostic` is implemented
- [ ] Error messages are user-friendly
- [ ] Error conversion is comprehensive

### Configuration:
- [ ] Config struct has proper derives
- [ ] Multiple config sources are supported
- [ ] Validation is thorough
- [ ] CLI integration works correctly

### Command routing:
- [ ] CLI structure is clean and intuitive
- [ ] Handler functions are well-organized
- [ ] Context passing is efficient
- [ ] Error handling through routing works