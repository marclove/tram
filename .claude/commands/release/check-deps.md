---
allowed-tools: Bash(cargo update:*), Bash(cargo audit:*), Bash(cargo outdated:*), Read
description: Verify dependencies are up-to-date and check for security advisories
---

## Current Dependency Status
- Main dependencies: !`grep -A 10 "\[dependencies\]" Cargo.toml`
- Workspace dependencies: !`grep -A 20 "\[workspace.dependencies\]" Cargo.toml || echo "No workspace dependencies"`
- Lock file status: !`ls -la Cargo.lock`
- cargo-audit availability: !`which cargo-audit || echo "Install with: cargo install cargo-audit"`
- cargo-outdated availability: !`which cargo-outdated || echo "Install with: cargo install cargo-outdated"`

## Dependency Health Areas

### 1. Security Auditing
- **Known vulnerabilities**: Check for security advisories
- **Yanked crates**: Identify pulled crate versions
- **License compliance**: Verify compatible licenses

### 2. Version Management
- **Outdated dependencies**: Find newer versions available
- **Compatibility**: Ensure clap + starbase versions work together
- **Breaking changes**: Identify major version updates needed

### 3. Dependency Tree Analysis
- **Duplicate dependencies**: Multiple versions of same crate
- **Unnecessary dependencies**: Dependencies not actually used
- **Dependency bloat**: Heavy dependencies for simple functionality

## Task
Comprehensive dependency health check:

### 1. Security audit:
```bash
# Install cargo-audit if not available
if ! which cargo-audit > /dev/null; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit
fi

# Run security audit
echo "=== Security Audit ==="
cargo audit

# Check for yanked crates
echo "=== Yanked Crates Check ==="
cargo audit --stale
```

### 2. Update analysis:
```bash
# Install cargo-outdated if needed
if ! which cargo-outdated > /dev/null; then
    echo "Installing cargo-outdated..."
    cargo install cargo-outdated
fi

# Check for outdated dependencies
echo "=== Outdated Dependencies ==="
cargo outdated --root-deps-only

# Show all outdated (including transitive)
echo "=== All Outdated Dependencies ==="
cargo outdated
```

### 3. Compatibility testing:
```bash
# Test with latest compatible versions
echo "=== Testing Dependency Updates ==="

# Update to latest compatible versions
cargo update

# Verify everything still builds
cargo build --all

# Run tests to ensure compatibility
cargo test --all

# Test examples still work
cargo build --examples
```

### 4. Dependency tree analysis:
```bash
# Show dependency tree
echo "=== Dependency Tree Analysis ==="
cargo tree | head -20

# Look for duplicate dependencies
echo "=== Duplicate Dependencies ==="
cargo tree --duplicates

# Check for unused dependencies (requires nightly)
if rustc --version | grep -q nightly; then
    echo "=== Unused Dependencies ==="
    cargo +nightly udeps
fi
```

### 5. Critical dependency status:
Special attention to core dependencies:

```bash
echo "=== Core Dependency Status ==="
echo "clap version:"
grep -E "clap.*=" Cargo.toml

echo "starbase versions:"
grep -E "starbase.*=" Cargo.toml

echo "async-trait version:"
grep -E "async-trait.*=" Cargo.toml

echo "tokio version:"
grep -E "tokio.*=" Cargo.toml
```

## Update Recommendations

### Safe Updates:
- **Patch versions**: Security fixes and bug patches
- **Compatible minors**: New features without breaking changes
- **Dev dependencies**: Testing and tooling updates

### Careful Updates:
- **clap major versions**: May require CLI interface updates
- **starbase updates**: Could affect session patterns
- **tokio major versions**: Runtime compatibility changes

### Update Strategy:
1. **Security fixes first**: Address any vulnerabilities immediately
2. **Patch updates**: Safe to apply automatically
3. **Minor updates**: Test thoroughly, especially for core deps
4. **Major updates**: Plan carefully, may require code changes

## Report Format
- **Security status**: Any vulnerabilities or yanked crates
- **Update opportunities**: Safe updates available
- **Breaking changes**: Major updates requiring attention
- **Compatibility matrix**: Core dependency version compatibility
- **Action items**: Specific updates to apply and testing needed
- **Timeline**: When updates should be applied (immediate/next release)