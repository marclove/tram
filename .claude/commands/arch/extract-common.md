---
allowed-tools: Grep, Read, Edit, MultiEdit
argument-hint: [functionality]
description: Extract repeated code into shared utilities
---

## Current Codebase
- Source files: !`find . -name "*.rs" -not -path "./starbase/*" -not -path "./target/*" | wc -l`
- Common patterns: !`grep -r "$1" --include="*.rs" . | wc -l || echo "No matches for $1"`

## Task
Find and extract repeated `$1` functionality into shared utilities:

1. **Identify duplication**: Search for similar code patterns
2. **Create shared module**: Extract common functionality to appropriate crate
3. **Update callers**: Replace duplicated code with shared utility calls
4. **Add tests**: Ensure extracted code is well-tested

## Common Extraction Targets
- **Error handling**: Custom error types and conversion patterns
- **Configuration**: Config loading and validation logic
- **Session helpers**: Common session initialization patterns
- **CLI utilities**: Argument parsing and validation helpers
- **File operations**: Path handling and filesystem utilities

## Success Criteria
- Duplication is eliminated
- Shared code is well-documented and tested
- All callers use the new shared utility
- Code maintainability is improved