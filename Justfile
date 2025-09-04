# Development workflow recipes for Tram
# Uses moon for task orchestration and proto for toolchain management

# Show available recipes
default:
    @just --list

# Quick development check - format, lint, build, test
check:
    moon run :format
    moon check --all

# Watch and continuously check on file changes (cargo-watch needed for this)
watch:
    @echo "Watching for changes... (Ctrl+C to stop)"
    @echo "Note: Install cargo-watch if not available: cargo install cargo-watch"
    cargo watch -s "just check"

# Run the CLI with arguments (only direct cargo usage for convenience)
run *ARGS:
    cargo run -- {{ARGS}}

# Clean everything (cargo + moon caches)
clean:
    moon clean --cache
    cargo clean

# Install/update toolchain and dependencies
setup:
    proto install
    moon setup
    moon sync

# Create a new crate in the workspace
new-crate NAME:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Creating new crate: {{NAME}}"
    mkdir -p crates/{{NAME}}/src
    cat > crates/{{NAME}}/Cargo.toml << EOF
    [package]
    name = "{{NAME}}"
    version.workspace = true
    edition.workspace = true
    license.workspace = true
    homepage.workspace = true
    repository.workspace = true
    description = "TODO: Add description"
    
    [dependencies]
    # Core dependencies
    tram-core = { path = "../tram-core" }
    EOF
    cat > crates/{{NAME}}/moon.yml << EOF
    \$$schema: 'https://moonrepo.dev/schemas/project.json'
    
    language: 'rust'
    
    tasks:
      build:
        command: 'cargo build --package {{NAME}}'
        inputs:
          - 'src/**/*'
          - 'Cargo.toml'
          
      test:
        command: 'cargo test --package {{NAME}}'
        inputs:
          - 'src/**/*'
          - 'tests/**/*'
          - 'Cargo.toml'
    EOF
    echo 'pub fn hello() { println!("Hello from {{NAME}}!"); }' > crates/{{NAME}}/src/lib.rs
    echo "Created crate at crates/{{NAME}}"
    echo "Added moon.yml configuration"
    echo "Don't forget to add it to the main Cargo.toml dependencies!"

# Build specific crate
build CRATE="":
    #!/usr/bin/env bash
    if [ "{{CRATE}}" = "" ]; then
        moon run :build
    else
        moon run {{CRATE}}:build
    fi

# Test specific crate  
test CRATE="":
    #!/usr/bin/env bash
    if [ "{{CRATE}}" = "" ]; then
        moon run :test
    else
        moon run {{CRATE}}:test
    fi

# Run specific examples to test functionality  
demo COMMAND="workspace":
    @echo "Running demo: tram {{COMMAND}}"
    cargo run -- {{COMMAND}}

# Performance check - build in release mode
perf:
    moon run :build --profile release
    @echo "Binary size:"
    @ls -lh target/release/tram 2>/dev/null | awk '{print $5 " " $9}' || echo "Release binary not found"

# Full release preparation check
release-check:
    @echo "Running full release checks..."
    moon run :format --check
    moon run :lint
    moon run :test  
    moon run :build --profile release
    @echo "Ready for release!"

# Show moon project graph
graph:
    moon project-graph --dot | dot -Tsvg -o project-graph.svg
    @echo "Project graph saved to project-graph.svg"

# Show what moon would run for a specific target
dry-run TARGET:
    moon run {{TARGET}} --dry-run