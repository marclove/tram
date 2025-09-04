---
allowed-tools: Read, Edit, MultiEdit, Grep
description: Update all documentation files to reflect current functionality
---

## Current Documentation State
- Main README: @README.md
- CLAUDE.md: @CLAUDE.md
- CRATES.md: @CRATES.md
- ROADMAP.md: @ROADMAP.md
- Crate READMEs: !`find crates -name "README.md" 2>/dev/null || echo "No crate READMEs found"`

## Documentation Sync Areas

### 1. Main Repository Docs
- **README.md**: Ensure examples match current code
- **CLAUDE.md**: Update with new patterns and practices
- **CRATES.md**: Reflect actual crate implementations
- **ROADMAP.md**: Update completed items and priorities

### 2. Individual Crate Documentation
- **Crate READMEs**: Consistent format and up-to-date examples
- **lib.rs docs**: Comprehensive module-level documentation
- **API docs**: Ensure all public items have rustdoc comments

### 3. Example Documentation
- **Example READMEs**: Match current functionality
- **Code comments**: Explain Tram-specific patterns
- **Usage instructions**: Accurate commands and outputs

## Task
Synchronize all documentation with the current codebase:

1. **Audit current state**:
   - Read through all existing documentation
   - Compare with actual code implementation
   - Identify outdated or missing information

2. **Update main docs**:
   - Refresh README.md examples and instructions
   - Update CLAUDE.md with new development patterns
   - Sync CRATES.md with implemented crates
   - Mark completed roadmap items

3. **Standardize crate docs**:
   - Ensure all crates have comprehensive READMEs
   - Verify API documentation is complete
   - Check code examples compile and run

4. **Validate examples**:
   - Update example READMEs to match current behavior
   - Ensure code comments explain Tram patterns clearly
   - Verify installation and usage instructions

## Documentation Standards
- **Consistent formatting**: Use the same markdown patterns
- **Accurate examples**: All code examples must be tested
- **Clear explanations**: Focus on developer experience
- **Cross-references**: Link related concepts together
- **Template ready**: Documentation should help users fork effectively

## Success Criteria
- All documentation accurately reflects current code
- Examples are copy-pastable and work correctly
- New developers can understand Tram's architecture quickly
- No broken links or outdated references
- Consistent tone and formatting throughout