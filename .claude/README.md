# Claude Code Hooks

This directory contains Claude Code hooks for the Tram project.

## PostToolUse Hook: Rust Warning Detection

The `rust-check.py` hook automatically runs after Edit, MultiEdit, or Write operations on Rust files (`.rs`) to detect compiler warnings and errors immediately.

### Features

- Runs `moon run :lint` (clippy with `-D warnings`) to catch issues early
- Extracts warnings/errors relevant to the edited file
- Provides immediate feedback to Claude for automatic issue resolution
- Non-blocking - files are still written, but Claude gets feedback to fix issues
- 30-second timeout to prevent hanging on long operations

### Configuration

The hook is configured in `settings.json`:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|MultiEdit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/rust-check.py",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

### How it Works

1. When Claude edits any `.rs` file, the hook triggers after the edit completes
2. The script runs `moon run :lint` to check for warnings/errors
3. If issues are found, it extracts relevant warnings for the edited file
4. Claude receives immediate feedback with the specific warnings to fix
5. Claude can then automatically fix the issues in the next response

This ensures that no Rust warnings or errors slip through in code that Claude writes.