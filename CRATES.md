# Tram Crates Architecture

Logical sub-packages for the Tram CLI starter kit, organized by developer experience themes.

## Core Foundation

### `tram-core`
**Integration layer between clap and starbase**
- CLI-to-session bridge utilities
- Common application lifecycle patterns
- Standard error types and result handling
- Base traits for CLI applications
- Session state management helpers

### `tram-config`
**Configuration management and validation**
- Multi-source config loading (files, env vars, CLI args)
- Config merging and precedence rules
- Schema validation with helpful error messages
- Environment-specific configuration
- Hot-reloading configuration changes
- Common config patterns (logging levels, output formats)

### `tram-workspace`
**Project and workspace detection**
- Workspace root detection algorithms
- Project type identification (Rust, Node.js, Python, etc.)
- Multi-project workspace handling
- Path utilities and workspace traversal
- .gitignore and exclude pattern handling

## Developer Experience

### `tram-templates`
**Code generation and scaffolding system**
- Template engine integration
- Project initialization workflows
- File and directory generation
- Variable substitution and templating
- Custom template creation utilities
- Template repository management

### `tram-dev`
**Development workflow enhancements**
- File watching and hot reload
- Development server mode
- Auto-restart on changes
- Development-specific logging
- Debug mode utilities
- Live configuration updates

### `tram-test`
**CLI testing utilities and fixtures**
- CLI command testing framework
- Output assertion helpers
- Temporary filesystem utilities
- Mock external dependencies
- Integration test patterns
- Performance regression testing

## User Interface

### `tram-output`
**Structured output and formatting**
- Multiple output format support (JSON, YAML, CSV, table)
- Consistent formatting across commands
- Color and styling management
- Pagination and streaming output
- Export utilities
- Machine-readable vs human-readable modes

### `tram-interactive`
**Interactive CLI elements**
- Enhanced prompts and confirmations
- Progress indicators and spinners  
- Multi-step wizards
- Form-like input collection
- Interactive selection menus
- Keyboard shortcut handling

### `tram-shell`
**Shell integration and environment**
- Shell completion generation (bash, zsh, fish, PowerShell)
- Shell hook integration
- Environment variable management
- Profile and dotfile integration
- Cross-platform shell compatibility
- Shell detection and adaptation

## Advanced Features

### `tram-plugins`
**Plugin architecture and extensibility**
- Plugin loading and lifecycle management
- Hook system for extensibility points
- Plugin communication and data sharing
- Plugin discovery and registration
- Security and sandboxing for plugins
- Plugin development utilities

### `tram-distribute`
**Release automation and distribution**
- Automated release workflows
- Package manager integration (Homebrew, Scoop, etc.)
- Cross-compilation and platform targeting
- Update notification system
- Version management utilities
- Binary size optimization

### `tram-diagnostics`
**Performance monitoring and diagnostics**
- Built-in profiling and benchmarking
- Memory usage tracking
- Performance regression detection
- Crash reporting and error telemetry (opt-in)
- Debug information collection
- System resource monitoring

## Integration Patterns

Each crate follows these principles:

### **Modular Design**
- Single responsibility per crate
- Clean interfaces between crates
- Optional dependencies where possible
- Feature flags for advanced functionality

### **Clap + Starbase Integration**
- Seamless integration with existing patterns
- Extends rather than replaces core functionality
- Preserves starbase's session-based architecture
- Enhances clap's derive macros where beneficial

### **Developer Experience Focus**
- Zero-config defaults for common use cases
- Extensive documentation and examples
- Clear error messages and diagnostics
- Consistent APIs across all crates

### **Production Ready**
- Comprehensive testing and validation
- Performance optimization
- Security considerations
- Cross-platform compatibility

## Usage Patterns

### **Minimal Setup**
```rust
// Just use tram-core for basic CLI + session integration
use tram_core::{App, Session};
```

### **Full-Featured**
```rust
// Use multiple crates for comprehensive CLI application
use tram_core::{App, Session};
use tram_config::Config;
use tram_interactive::Progress;
use tram_output::Table;
```

### **À la Carte**
```rust
// Pick specific crates for targeted functionality
use tram_templates::Generator;
use tram_test::CliTester;
```

This architecture ensures developers can:
- Start minimal and grow incrementally
- Use only the features they need
- Maintain consistency across CLI applications
- Benefit from battle-tested patterns and utilities