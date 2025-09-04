---
allowed-tools: Bash(cargo build:*), Bash(find:*), Bash(ls:*), Bash(cargo bloat:*)
description: Generate detailed binary size analysis for different feature combinations
---

## Current Build State
- Available examples: !`find examples -name "Cargo.toml" | wc -l || echo "0"`
- Release build status: !`cargo build --release --quiet && echo "OK" || echo "FAILED"`
- cargo-bloat availability: !`which cargo-bloat || echo "Install with: cargo install cargo-bloat"`

## Size Analysis Areas

### 1. Binary Size by Configuration
- **Minimal build**: Only essential features
- **Default build**: Standard feature set
- **Full build**: All optional features enabled
- **Debug vs Release**: Optimization impact

### 2. Dependency Contribution
- **Core dependencies**: clap, starbase, tokio size impact
- **Optional dependencies**: Feature-specific additions
- **Tram overhead**: Cost of integration layer

### 3. Feature Impact Analysis
- **Individual features**: Size cost of each optional feature
- **Feature combinations**: Non-linear size interactions
- **Dead code**: Unused code in various configurations

## Task
Generate comprehensive binary size report:

### Build different configurations:
```bash
# Minimal build
cargo build --release --no-default-features
cp target/release/examples/* /tmp/tram-minimal/ 2>/dev/null || mkdir -p /tmp/tram-minimal

# Default build
cargo build --release
cp target/release/examples/* /tmp/tram-default/ 2>/dev/null || mkdir -p /tmp/tram-default

# Full feature build (if features exist)
cargo build --release --all-features
cp target/release/examples/* /tmp/tram-full/ 2>/dev/null || mkdir -p /tmp/tram-full
```

### Size analysis:
```bash
# Binary sizes by configuration
echo "=== Binary Size Comparison ==="
echo "Minimal build:"
ls -lh /tmp/tram-minimal/ 2>/dev/null || echo "No minimal binaries"

echo "Default build:"
ls -lh /tmp/tram-default/ 2>/dev/null || echo "No default binaries"

echo "Full build:"
ls -lh /tmp/tram-full/ 2>/dev/null || echo "No full binaries"

# Size breakdown with cargo-bloat if available
if which cargo-bloat > /dev/null; then
    echo "=== Dependency Size Analysis ==="
    cargo bloat --release --crates
    
    echo "=== Function Size Analysis ==="
    cargo bloat --release -n 20
fi
```

### Feature impact analysis:
```bash
# Individual feature impact (if features exist)
echo "=== Feature Impact Analysis ==="
for feature in $(grep -E "^\[features\]" Cargo.toml -A 20 | grep -E "^[a-z]" | cut -d' ' -f1 | cut -d'=' -f1); do
    echo "Testing feature: $feature"
    cargo build --release --no-default-features --features $feature --quiet
    size=$(ls -l target/release/examples/* 2>/dev/null | awk '{sum += $5} END {print sum}')
    echo "Size with $feature: $size bytes"
done
```

### Debug vs Release comparison:
```bash
echo "=== Debug vs Release Size Comparison ==="
cargo build --examples
echo "Debug build sizes:"
ls -lh target/debug/examples/ | head -5

cargo build --release --examples  
echo "Release build sizes:"
ls -lh target/release/examples/ | head -5
```

## Size Targets
- **Minimal CLI**: < 2MB (no optional features)
- **Standard CLI**: < 5MB (default features)
- **Full-featured CLI**: < 10MB (all features)
- **Debug overhead**: < 3x release size

## Analysis Output
1. **Size breakdown**: Exact sizes for different configurations
2. **Dependency impact**: Which dependencies contribute most to size
3. **Feature cost**: Size penalty for each optional feature
4. **Optimization opportunities**: Areas for size reduction
5. **Trends**: How size has changed over development
6. **Recommendations**: Specific actions to reduce binary size

## Report Sections
- **Executive summary**: Key size metrics and trends
- **Detailed breakdown**: Per-configuration analysis
- **Feature analysis**: Cost/benefit of optional features
- **Optimization recommendations**: Actionable size reduction steps
- **Historical tracking**: Size evolution over time