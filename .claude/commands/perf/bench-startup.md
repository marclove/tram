---
allowed-tools: Bash(cargo build:*), Bash(cargo run:*), Bash(hyperfine:*), Bash(time:*)
description: Measure startup time and memory footprint of Tram applications
---

## Current Build Status
- Release build status: !`cargo build --release --quiet && echo "Release build OK" || echo "Release build FAILED"`
- Available examples: !`ls examples/*/Cargo.toml 2>/dev/null | sed 's|/Cargo.toml||' | sed 's|examples/||' | head -5`
- hyperfine availability: !`which hyperfine || echo "hyperfine not available - install with 'cargo install hyperfine'"`

## Startup Performance Metrics

### 1. Cold Start Time
- **Minimal CLI**: Time from invocation to help text display
- **Session initialization**: Time to complete startup phase
- **First command**: Time to execute simple command

### 2. Memory Usage
- **Initial allocation**: Memory used at startup
- **Peak usage**: Maximum memory during simple operations
- **Memory efficiency**: Comparison with equivalent tools

### 3. Binary Size
- **Debug build**: Development binary size
- **Release build**: Optimized binary size
- **With features**: Size impact of optional features

## Task
Benchmark Tram application performance:

### Build optimized binaries:
```bash
# Build release versions
cargo build --release --examples

# Check binary sizes
ls -lh target/release/examples/ | head -10
```

### Startup time benchmarks:
```bash
# If hyperfine is available
for example in examples/*/; do
    name=$(basename "$example")
    echo "Benchmarking startup time for $name"
    hyperfine "cargo run --release --example $name -- --help"
done

# Fallback to time command
for example in examples/*/; do
    name=$(basename "$example")
    echo "Timing $name startup"
    time cargo run --release --example $name -- --help
done
```

### Memory usage analysis:
```bash
# Basic memory profiling
for example in examples/*/; do
    name=$(basename "$example")
    echo "Memory usage for $name"
    /usr/bin/time -v cargo run --release --example $name -- --help 2>&1 | grep -E "Maximum resident|User time|System time"
done
```

### Binary size analysis:
```bash
# Compare binary sizes
echo "Binary Size Analysis:"
find target/release/examples -name "*" -type f -executable -exec ls -lh {} \;

# Size with different feature combinations
cargo build --release --no-default-features
ls -lh target/release/examples/
```

## Performance Targets
- **Startup time**: < 100ms for help display
- **Memory usage**: < 10MB for simple operations  
- **Binary size**: < 5MB for basic CLI (release build)
- **Session overhead**: < 10ms additional latency

## Analysis Required
1. **Identify bottlenecks**: What's taking the most time?
2. **Memory hotspots**: Where is memory being allocated?
3. **Size contributors**: Which dependencies add the most size?
4. **Optimization opportunities**: What can be improved?

## Report Format
- **Current performance**: Actual measurements vs targets
- **Trend analysis**: How performance has changed over time
- **Bottleneck identification**: Specific areas for improvement
- **Recommendations**: Concrete steps to optimize performance
- **Comparison**: How Tram performs vs other CLI tools