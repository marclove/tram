---
allowed-tools: Bash(find:*), Bash(grep:*), Read
description: Show comprehensive development status of Tram starter kit
---

## Current State
- Crates: !`find crates -name "Cargo.toml" 2>/dev/null | wc -l || echo "0"`
- Examples: !`find examples -name "Cargo.toml" 2>/dev/null | wc -l || echo "0"`
- Tests: !`find . -name "*test*.rs" | wc -l`
- Documentation: !`find . -name "*.md" -not -path "./starbase/*" | wc -l`

## Status Report

### Crate Implementation
- **Implemented**: @CRATES.md vs actual crates directory
- **Test coverage**: Tests per crate
- **Documentation**: README and API docs status

### Roadmap Progress
- **Phase 1 status**: @ROADMAP.md completion percentage
- **Blocked items**: Dependencies preventing progress
- **Next priorities**: Ready-to-implement items

### Code Quality
- **Build status**: `cargo check --all` results
- **Test status**: `cargo test --all` results
- **Lint status**: `cargo clippy` results

### Meta-Development Health
- **Documentation sync**: Are docs current with code?
- **Example validity**: Do examples work with current codebase?
- **Pattern consistency**: Are Tram patterns followed throughout?

## Output Summary
Provide concise status with:
- **Overall health**: Green/Yellow/Red status
- **Completion percentage**: Progress toward Phase 1 goals
- **Immediate actions**: Top 3 items needing attention
- **Blockers**: Issues preventing progress