//! Integration tests for shell completion functionality.
//!
//! These tests verify that the tram CLI can generate valid shell completions
//! for all supported shells and that the generated completions work correctly.

use std::fs;

mod common;

use common::{FileAssertions, TempDir, TramCommand, init_tests};

#[test]
fn test_bash_completions_generation() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "bash"])
        .assert_success();

    // Check that bash completion script is generated
    output.assert_stdout_contains("complete -F _tram");
    output.assert_stdout_contains("# To install bash completions");
}

#[test]
fn test_zsh_completions_generation() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "zsh"])
        .assert_success();

    // Check that zsh completion script is generated
    output.assert_stdout_contains("#compdef tram");
    output.assert_stdout_contains("# To install zsh completions");
}

#[test]
fn test_fish_completions_generation() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "fish"])
        .assert_success();

    // Check that fish completion script is generated
    output.assert_stdout_contains("complete -c tram");
    output.assert_stdout_contains("# To install fish completions");
}

#[test]
fn test_powershell_completions_generation() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "powershell"])
        .assert_success();

    // Check that PowerShell completion script is generated
    output.assert_stdout_contains("Register-ArgumentCompleter");
    output.assert_stdout_contains("# To install PowerShell completions");
}

#[test]
fn test_completions_help() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "--help"])
        .assert_success();

    output.assert_stdout_contains("Generate shell completions");
    output.assert_stdout_contains("Shell to generate completions for");
}

#[test]
fn test_invalid_shell_completion() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "invalid-shell"])
        .assert_failure();

    // Should show error about invalid shell
    output.assert_stderr_contains("invalid value 'invalid-shell'");
}

#[test]
fn test_completions_include_all_subcommands() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "bash"])
        .assert_success();

    // Verify all main subcommands are included in completions
    let stdout = output.stdout();
    assert!(stdout.contains("new"));
    assert!(stdout.contains("generate"));
    assert!(stdout.contains("init"));
    assert!(stdout.contains("workspace"));
    assert!(stdout.contains("config"));
    assert!(stdout.contains("watch"));
    assert!(stdout.contains("examples"));
    assert!(stdout.contains("completions"));
    assert!(stdout.contains("man"));
}

#[test]
fn test_bash_completion_includes_options() {
    init_tests();

    let output = TramCommand::new()
        .args(["completions", "bash"])
        .assert_success();

    let stdout = output.stdout();

    // Check for global options
    assert!(stdout.contains("--log-level"));
    assert!(stdout.contains("--format"));
    assert!(stdout.contains("--no-color"));
    assert!(stdout.contains("--config"));
    assert!(stdout.contains("--help"));
}

#[test]
fn test_completions_save_to_file() {
    init_tests();

    let temp_dir = TempDir::new("completions-save-test").unwrap();
    let completion_file = temp_dir.path().join("tram_completion.bash");

    // Generate bash completions using TramCommand
    let output = TramCommand::new()
        .args(["completions", "bash"])
        .output()
        .expect("Failed to execute command");

    // Write output to file
    fs::write(&completion_file, output.stdout).unwrap();

    // Verify file was created and contains expected content
    FileAssertions::assert_file_exists(&completion_file);
    FileAssertions::assert_file_contains(&completion_file, "complete -F _tram");
    FileAssertions::assert_file_contains(&completion_file, "_tram() {");
}

#[test]
fn test_all_shells_generate_unique_completions() {
    init_tests();

    let temp_dir = TempDir::new("all-shells-test").unwrap();

    let shells = ["bash", "zsh", "fish", "powershell"];

    for shell in &shells {
        let output = TramCommand::new()
            .args(["completions", shell])
            .assert_success();

        let completion_file = temp_dir.path().join(format!("tram.{}", shell));
        fs::write(&completion_file, output.stdout()).unwrap();

        // Verify each shell has unique syntax
        match *shell {
            "bash" => {
                FileAssertions::assert_file_contains(&completion_file, "complete -F _tram");
            }
            "zsh" => {
                FileAssertions::assert_file_contains(&completion_file, "#compdef tram");
            }
            "fish" => {
                FileAssertions::assert_file_contains(&completion_file, "complete -c tram");
            }
            "powershell" => {
                FileAssertions::assert_file_contains(
                    &completion_file,
                    "Register-ArgumentCompleter",
                );
            }
            _ => unreachable!(),
        }
    }

    // Verify we created all expected files
    assert_eq!(FileAssertions::count_files(temp_dir.path(), r"tram\.*"), 4);
}
