# Tram Crates Architecture

Logical sub-packages for the Tram CLI starter kit, organized by developer experience themes.

## Current Implementation Status

**Phase 2 (Developer Experience)** is now **complete** with a pragmatic approach:

- **Core functionality** is implemented in focused crates (`tram-core`, `tram-config`, `tram-workspace`, `tram-test`)
- **Developer tools** are integrated into the main binary for simplicity (shell completions, man pages)
- **Examples** demonstrate patterns without requiring separate crates
- **Advanced features** are marked for future extraction into dedicated crates as needed

This approach prioritizes **immediate usability** over architectural purity, allowing developers to fork and use Tram right away while maintaining the option to extract functionality into separate crates later.

## Core Foundation

### `tram-core` ✅ **Implemented**
**Integration layer between clap and starbase**
- CLI-to-session bridge utilities
- Common application lifecycle patterns  
- Standard error types and result handling (TramError with miette integration)
- Project initialization system (Rust, Node.js, Python, Go, Java, Generic)
- Template generation system with Handlebars integration
- Structured logging and tracing setup
- Base traits for CLI applications

### `tram-config` ✅ **Implemented**
**Configuration management and validation**
- Multi-source config loading (JSON, YAML, TOML files + env vars + CLI args)
- Config merging with proper precedence rules (CLI > env > files > defaults)
- Schema validation with schematic framework
- Hot-reload configuration changes with file watching (notify crate)
- Thread-safe configuration updates with custom ConfigChangeHandler trait
- Common config patterns (log levels, output formats, colors)
- camelCase field names for JavaScript ecosystem compatibility

### `tram-workspace` ✅ **Implemented**
**Project and workspace detection**
- Workspace root detection algorithms (Git, package.json, Cargo.toml, etc.)
- Project type identification (Rust, Node.js, Python, Go, Java)
- Path utilities and workspace traversal
- Ignore pattern handling for different project types
- ProjectType enum with detection methods and ignore patterns

## Developer Experience

### `tram-templates` ✅ **Implemented** (integrated into tram-core)
**Code generation and scaffolding system**
- Template engine integration (Handlebars) ✅ **Implemented**
- Project initialization workflows ✅ **Implemented**
- File and directory generation ✅ **Implemented**
- Variable substitution and templating ✅ **Implemented**
- CLI pattern template generation (`tram generate`) ✅ **Implemented**
- Template repository management (planned)

### `tram-dev` 🔄 **Partially Implemented** (integrated into main binary + tram-config)
**Development workflow enhancements**
- File watching and hot reload ✅ **Implemented** (`tram watch` command)
- Live configuration updates ✅ **Implemented** (integrated with tram-config hot reload)
- Development-specific logging ✅ **Implemented**
- Debug mode utilities ✅ **Implemented**
- Development server mode (planned)
- Auto-restart on changes (planned)

### `tram-test` ✅ **Implemented**
**CLI testing utilities and fixtures**
- CLI command testing framework (TramCommand helper)
- Output assertion helpers with stdout/stderr pattern matching
- Temporary filesystem utilities (TempDir with automatic cleanup)
- File system assertion utilities (FileAssertions)
- Mock builders for common objects  
- Integration test patterns with workspace-level test support
- Clean environment setup (NO_COLOR, TRAM_LOG_LEVEL controls)

## User Interface

### `tram-output` 🔄 **Partially Implemented** (via tram-config)
**Structured output and formatting**
- Multiple output format support (JSON, YAML, table) ✅ **Implemented**
- Consistent formatting across commands ✅ **Implemented**
- Color and styling management ✅ **Implemented** (NO_COLOR support)
- Pagination and streaming output (planned)
- Export utilities (planned) 
- Machine-readable vs human-readable modes ✅ **Implemented**

### `tram-interactive` 🔄 **Examples Implemented** (via examples/ directory)
**Interactive CLI elements**
- Enhanced prompts and confirmations ✅ **Example implemented** 
- Progress indicators and spinners ✅ **Example implemented**
- Multi-step wizards (planned)
- Form-like input collection ✅ **Example implemented**
- Interactive selection menus ✅ **Example implemented** 
- Keyboard shortcut handling (planned)

### `tram-shell` 🔄 **Partially Implemented** (integrated into main binary)
**Shell integration and environment**
- Shell completion generation (bash, zsh, fish, PowerShell) ✅ **Implemented**
- Manual page generation with build automation ✅ **Implemented**  
- Environment variable management (via tram-config) ✅ **Implemented**
- Shell hook integration (planned)
- Profile and dotfile integration (planned)
- Cross-platform shell compatibility (planned)
- Shell detection and adaptation (planned)

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
