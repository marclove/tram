---
allowed-tools: Bash(cargo new:*), Bash(mkdir:*), Write, MultiEdit, Edit
argument-hint: [crate-name] [description]
description: Create a new Tram crate with standard structure and boilerplate
---

## Current Workspace Structure
- Workspace root: !`pwd`
- Existing crates: !`ls -la crates/`
- Current Cargo.toml workspace members: !`grep -A 10 members Cargo.toml || echo "No workspace Cargo.toml found"`

## Task
Create a new Tram crate called `tram-$1` with the following:

1. Create the crate structure in `crates/tram-$1/`
2. Set up Cargo.toml with:
   - Workspace dependencies for clap, starbase, async-trait, miette, thiserror
   - Standard metadata following Tram patterns
   - Description: "$2"
3. Create a comprehensive README.md explaining:
   - The crate's purpose and position in the Tram ecosystem
   - Usage examples
   - Integration with other Tram crates
4. Create lib.rs with:
   - Standard module exports
   - Re-exports of commonly used types
   - Basic documentation
5. Create basic test structure with integration tests
6. Update root Cargo.toml to include the new crate in workspace members
7. Update CRATES.md documentation to reflect the new crate

Follow the patterns established in @CRATES.md and look at the starbase submodule structure at @starbase/crates/ for inspiration.

Ensure the crate integrates cleanly with the clap + starbase foundation and follows Tram's architectural principles.