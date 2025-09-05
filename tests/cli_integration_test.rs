//! Integration tests for CLI subcommands and end-to-end functionality.
//!
//! These tests verify that all CLI subcommands work correctly and
//! provide the expected behavior for end users.

mod common;

use common::{FileAssertions, TempDir, TramCommand, init_tests};

#[test]
fn test_cli_help() {
    init_tests();

    let output = TramCommand::new().args(["--help"]).assert_success();

    output.assert_stdout_contains(
        "A batteries-included starter kit for building CLI applications in Rust",
    );
    output.assert_stdout_contains("Usage:");
    output.assert_stdout_contains("Commands:");

    // Verify all main subcommands are listed
    output.assert_stdout_contains("new");
    output.assert_stdout_contains("generate");
    output.assert_stdout_contains("init");
    output.assert_stdout_contains("workspace");
    output.assert_stdout_contains("config");
    output.assert_stdout_contains("watch");
    output.assert_stdout_contains("examples");
    output.assert_stdout_contains("completions");
    output.assert_stdout_contains("man");

    // Verify global options are listed
    output.assert_stdout_contains("--log-level");
    output.assert_stdout_contains("--format");
    output.assert_stdout_contains("--no-color");
    output.assert_stdout_contains("--config");
}

#[test]
fn test_subcommand_help() {
    init_tests();

    // Test help for each subcommand
    let subcommands = [
        ("new", "Create a new project interactively"),
        ("generate", "Generate templates for common CLI patterns"),
        ("init", "Initialize a new project"),
        ("workspace", "Show workspace information"),
        ("config", "Show configuration information"),
        ("watch", "Watch mode"),
        ("examples", "Run interactive examples"),
        ("completions", "Generate shell completions"),
        ("man", "Generate manual pages"),
    ];

    for (subcommand, description) in &subcommands {
        let output = TramCommand::new()
            .args([subcommand, "--help"])
            .assert_success();

        output.assert_stdout_contains(description);
        output.assert_stdout_contains("Usage:");
    }
}

#[test]
fn test_config_command() {
    init_tests();

    let output = TramCommand::new().args(["config"]).assert_success();

    output.assert_stdout_contains("Current configuration:");
    output.assert_stdout_contains("Log level:");
    output.assert_stdout_contains("Output format:");
    output.assert_stdout_contains("Colors:");
}

#[test]
fn test_workspace_command_no_workspace() {
    init_tests();

    // Use /tmp directory which should never be a tram workspace
    let output = TramCommand::new()
        .current_dir("/tmp")
        .args(["workspace"])
        .assert_failure();

    output.assert_stderr_contains("Workspace not found");
}

#[test]
fn test_workspace_command_with_workspace() {
    init_tests();

    let output = TramCommand::new().args(["workspace"]).assert_success();

    output.assert_stdout_contains("Workspace root:");
    output.assert_stdout_contains("Project type:");
}

#[test]
fn test_workspace_command_detailed() {
    init_tests();

    let output = TramCommand::new()
        .args(["workspace", "--detailed"])
        .assert_success();

    output.assert_stdout_contains("Workspace root:");
    output.assert_stdout_contains("Project type:");
    output.assert_stdout_contains("Ignore patterns:");
}

#[test]
fn test_examples_command() {
    init_tests();

    let examples = [
        "basic-command",
        "async-operations",
        "config-usage",
        "progress-indicators",
        "interactive-prompts",
        "file-operations",
    ];

    for example in &examples {
        let output = TramCommand::new()
            .args(["examples", example])
            .assert_success();

        output.assert_stdout_contains("Example");
        output.assert_stdout_contains("Key features demonstrated:");
        output.assert_stdout_contains("For full interactive example, run:");
    }
}

#[test]
fn test_new_command_dry_run() {
    init_tests();

    let temp_dir = TempDir::new("new-command-test").unwrap();

    let output = TramCommand::new()
        .current_dir(temp_dir.path())
        .args(["new", "test-project", "--skip-prompts"])
        .assert_success();

    output.assert_stdout_contains("Created new");
    output.assert_stdout_contains("test-project");

    // Verify project directory was created
    FileAssertions::assert_dir_exists(temp_dir.path().join("test-project"));
}

#[test]
fn test_new_command_with_options() {
    init_tests();

    let temp_dir = TempDir::new("new-options-test").unwrap();

    let output = TramCommand::new()
        .current_dir(temp_dir.path())
        .args([
            "new",
            "test-nodejs-project",
            "--project-type",
            "nodejs",
            "--description",
            "A test Node.js project",
            "--skip-prompts",
        ])
        .assert_success();

    output.assert_stdout_contains("Created new Node.js project: test-nodejs-project");
    output.assert_stdout_contains("Description: A test Node.js project");
}

#[test]
fn test_generate_command_to_stdout() {
    init_tests();

    let output = TramCommand::new()
        .args([
            "generate",
            "--template-type",
            "command",
            "backup",
            "--description",
            "Backup command template",
        ])
        .assert_success();

    output.assert_stdout_contains("Generated Command template for 'backup':");
    output.assert_stdout_contains("File path:");
    output.assert_stdout_contains("To write to filesystem, add the --write flag");
}

#[test]
fn test_generate_command_with_write() {
    init_tests();

    let temp_dir = TempDir::new("generate-write-test").unwrap();

    let output = TramCommand::new()
        .current_dir(temp_dir.path())
        .args([
            "generate",
            "--template-type",
            "command",
            "backup",
            "--write",
        ])
        .assert_success();

    output.assert_stdout_contains("Generated Command template: backup");

    // Template should be written to filesystem
    // (The exact file location depends on the template implementation)
}

#[test]
fn test_init_legacy_command() {
    init_tests();

    let temp_dir = TempDir::new("init-legacy-test").unwrap();

    let output = TramCommand::new()
        .current_dir(temp_dir.path())
        .args(["init", "legacy-project"])
        .assert_success();

    output.assert_stdout_contains("Initializing project: legacy-project");
    output.assert_stdout_contains("Project 'legacy-project' initialized!");
}

#[test]
fn test_init_verbose() {
    init_tests();

    let temp_dir = TempDir::new("init-verbose-test").unwrap();

    let output = TramCommand::new()
        .current_dir(temp_dir.path())
        .args(["init", "verbose-project", "--verbose"])
        .assert_success();

    output.assert_stdout_contains("Verbose mode enabled");
    output.assert_stdout_contains("Workspace root:");
    output.assert_stdout_contains("Config:");
}

#[test]
fn test_global_options_log_level() {
    init_tests();

    let output = TramCommand::new()
        .args(["--log-level", "debug", "config"])
        .assert_success();

    // With debug level, should see debug output in logs
    // The exact format depends on the logging configuration
    output.assert_stdout_contains("Current configuration:");
}

#[test]
fn test_global_options_format() {
    init_tests();

    // Test JSON format
    let output = TramCommand::new()
        .args(["--format", "json", "config"])
        .assert_success();

    output.assert_stdout_contains("Current configuration:");

    // Test YAML format
    let output = TramCommand::new()
        .args(["--format", "yaml", "config"])
        .assert_success();

    output.assert_stdout_contains("Current configuration:");

    // Test Table format (default)
    let output = TramCommand::new()
        .args(["--format", "table", "config"])
        .assert_success();

    output.assert_stdout_contains("Current configuration:");
}

#[test]
fn test_global_options_no_color() {
    init_tests();

    let output = TramCommand::new()
        .args(["--no-color", "config"])
        .assert_success();

    output.assert_stdout_contains("Current configuration:");
    // With no-color flag, output should not contain ANSI color codes
    // This would require more sophisticated testing to verify properly
}

#[test]
fn test_invalid_subcommand() {
    init_tests();

    let output = TramCommand::new()
        .args(["invalid-command"])
        .assert_failure();

    output.assert_stderr_contains("unrecognized subcommand 'invalid-command'");
}

#[test]
fn test_invalid_global_option() {
    init_tests();

    let output = TramCommand::new()
        .args(["--invalid-option"])
        .assert_failure();

    output.assert_stderr_contains("unexpected argument '--invalid-option'");
}

#[test]
fn test_missing_required_argument() {
    init_tests();

    let output = TramCommand::new().args(["new"]).assert_failure();

    output.assert_stderr_contains("required arguments were not provided");
}

#[test]
fn test_cli_version_info() {
    init_tests();

    let output = TramCommand::new().args(["--version"]).assert_success();

    output.assert_stdout_contains("tram");
    output.assert_stdout_contains("0.1.0");
}
