# Claude Code Hooks

This directory contains Claude Code hooks for the Tram project.

## PreToolUse Hook: Bash Cargo Prevention

The `bash-cargo-check.py` hook runs before Bash commands execute to prevent direct cargo usage and guide Claude to use proper task orchestration.

### Features

- Intercepts direct `cargo` commands before execution
- **Dynamically lists available just and moon tasks** from the project
- Provides specific suggestions for just/moon alternatives
- Shows real-time available development commands
- Explains benefits of using task orchestration
- Quick 10-second timeout for minimal impact

### Blocked Commands & Alternatives

- `cargo build` → `just build`
- `cargo test` → `just test` 
- `cargo check` → `just check`
- `cargo clippy` → `just check`
- `cargo fmt` → `just check`
- `cargo run` → `just run`
- `cargo clean` → `just clean`

### Dynamic Task Discovery

When a cargo command is blocked, the hook automatically runs:

- `just --list` to show all available just recipes with descriptions
- `moon query tasks` to show all available moon tasks across crates

This provides Claude with real-time, accurate information about what tasks are actually available in the project, eliminating the need to guess or look up commands manually.

## PostToolUse Hook: Rust Warning Detection

The `rust-check.py` hook automatically runs after Edit, MultiEdit, or Write operations on Rust files (`.rs`) to detect compiler warnings and errors immediately.

### Features

- Runs `moon run :lint` (clippy with `-D warnings`) to catch issues early
- Extracts warnings/errors relevant to the edited file
- Provides immediate feedback to Claude for automatic issue resolution
- Non-blocking - files are still written, but Claude gets feedback to fix issues
- 30-second timeout to prevent hanging on long operations

## Configuration

Both hooks are configured in `settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/bash-cargo-check.py",
            "timeout": 10
          }
        ]
      }
    ],
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

## Benefits

These hooks ensure:

1. **Consistent workflows** - All development goes through moon task orchestration
2. **No warnings slip through** - Rust code is checked immediately after editing
3. **Immediate feedback** - Issues are caught and reported instantly
4. **Better caching** - Moon's intelligent caching is always used
5. **Proper dependencies** - Task dependencies are respected across crates