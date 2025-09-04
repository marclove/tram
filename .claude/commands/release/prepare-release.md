---
allowed-tools: Edit, MultiEdit, Bash(git status:*), Bash(git add:*), Bash(git commit:*), Bash(cargo test:*)
argument-hint: [version]
description: Update version numbers and prepare for publishing a new Tram release
---

## Current Release State
- Git status: !`git status --porcelain`
- Current version in Cargo.toml: !`grep -E '^version = ' Cargo.toml | head -1`
- Recent releases: !`git tag -l | tail -5 || echo "No tags found"`
- Unreleased changes: !`git log --oneline $(git describe --tags --abbrev=0 2>/dev/null || echo "HEAD~10")..HEAD | head -10`

## Pre-Release Checklist

### 1. Version Validation
- Ensure `$1` follows semantic versioning (e.g., 0.1.0, 1.0.0)
- Check that version is higher than current version
- Verify version makes sense for changes (major/minor/patch)

### 2. Quality Assurance
- All tests must pass
- Documentation must be up to date
- Examples must work correctly
- No outstanding critical issues

### 3. Release Notes
- Generate changelog from git commits
- Highlight breaking changes
- Document new features and improvements
- Include upgrade instructions if needed

## Task
Prepare Tram for release version `$1`:

### 1. Update version numbers:
```toml
# Update main Cargo.toml
[package]
version = "$1"

# Update any internal dependency references
[dependencies]
tram-core = { version = "$1", path = "crates/tram-core" }
```

### 2. Run pre-release validation:
```bash
# Full test suite
cargo test --all

# Build all examples
cargo build --release --examples

# Validate examples work
cargo run --release --example basic -- --help

# Check documentation builds
cargo doc --no-deps

# Lint check
cargo clippy -- -D warnings
```

### 3. Update documentation:
- Update README.md with any new features
- Refresh CLAUDE.md if development patterns changed
- Update ROADMAP.md to mark completed features
- Generate fresh API documentation

### 4. Create release artifacts:
```bash
# Clean build
cargo clean
cargo build --release

# Package for distribution (if publishing to crates.io)
cargo package --list

# Verify package contents
cargo package --allow-dirty
```

### 5. Generate release notes:
Based on commits since last release, create notes covering:
- **New Features**: Major additions to Tram
- **Improvements**: Enhancements to existing functionality  
- **Bug Fixes**: Issues resolved
- **Breaking Changes**: API changes requiring user updates
- **Dependencies**: Updated clap, starbase, or other deps

### 6. Commit release changes:
```bash
# Stage all version and documentation updates
git add -A

# Create release commit
git commit -m "chore: prepare release v$1

- Update version to $1
- Update documentation
- Refresh examples

🤖 Generated with Claude Code

Co-Authored-By: Claude <noreply@anthropic.com>"

# Create release tag
git tag -a "v$1" -m "Release v$1"
```

## Release Validation
Before finalizing:
- [ ] All tests pass
- [ ] Examples build and run
- [ ] Documentation is accurate
- [ ] Version numbers are consistent
- [ ] Git history is clean
- [ ] Release notes are comprehensive

## Post-Release Actions
(For manual execution after this command):
- Push to GitHub: `git push origin main --tags`
- Create GitHub release with release notes
- Publish to crates.io if applicable: `cargo publish`
- Update template repository settings
- Announce release in relevant channels