//! Integration tests for manual page generation functionality.
//!
//! These tests verify that the tram CLI can generate valid manual pages
//! and that the generated man pages contain correct content.

use std::fs;

mod common;

use common::{FileAssertions, TempDir, TramCommand, init_tests};

#[test]
fn test_man_page_generation_default() {
    init_tests();

    let temp_dir = TempDir::new("man-default-test").unwrap();
    let man_dir = temp_dir.path().join("man");

    let output = TramCommand::new()
        .args(["man", "--output-dir", man_dir.to_str().unwrap()])
        .assert_success();

    // Check command output
    output.assert_stdout_contains("Generated man page:");
    output.assert_stdout_contains("Manual pages generated in:");
    output.assert_stdout_contains("To install system-wide:");
    output.assert_stdout_contains("To view locally:");

    // Verify directory was created
    FileAssertions::assert_dir_exists(&man_dir);

    // Check that main man page was generated
    let main_man_page = man_dir.join("tram.1");
    FileAssertions::assert_file_exists(&main_man_page);

    // Check subcommand man pages
    let subcommands = [
        "new",
        "generate",
        "init",
        "workspace",
        "config",
        "watch",
        "examples",
        "completions",
        "man",
    ];
    for subcommand in &subcommands {
        let man_file = man_dir.join(format!("tram-{}.1", subcommand));
        FileAssertions::assert_file_exists(&man_file);
    }

    // Count total generated files
    assert_eq!(FileAssertions::count_files(&man_dir, r".*\.1$"), 10); // 1 main + 9 subcommands
}

#[test]
fn test_man_page_content_format() {
    init_tests();

    let temp_dir = TempDir::new("man-content-test").unwrap();
    let man_dir = temp_dir.path().join("man");

    TramCommand::new()
        .args(["man", "--output-dir", man_dir.to_str().unwrap()])
        .assert_success();

    let main_man_page = man_dir.join("tram.1");

    // Check man page format and content
    FileAssertions::assert_file_matches(&main_man_page, r"\.TH tram 1");
    FileAssertions::assert_file_contains(&main_man_page, ".SH NAME");
    FileAssertions::assert_file_contains(&main_man_page, ".SH SYNOPSIS");
    FileAssertions::assert_file_contains(&main_man_page, ".SH DESCRIPTION");
    FileAssertions::assert_file_contains(&main_man_page, ".SH OPTIONS");
    FileAssertions::assert_file_contains(&main_man_page, ".SH SUBCOMMANDS");

    // Check main description
    FileAssertions::assert_file_contains(
        &main_man_page,
        "A batteries\\-included starter kit for building CLI applications in Rust",
    );

    // Check global options are documented
    FileAssertions::assert_file_contains(&main_man_page, "\\-\\-log\\-level");
    FileAssertions::assert_file_contains(&main_man_page, "\\-\\-format");
    FileAssertions::assert_file_contains(&main_man_page, "\\-\\-no\\-color");
    FileAssertions::assert_file_contains(&main_man_page, "\\-\\-config");
}

#[test]
fn test_subcommand_man_pages_content() {
    init_tests();

    let temp_dir = TempDir::new("man-subcommand-test").unwrap();
    let man_dir = temp_dir.path().join("man");

    TramCommand::new()
        .args(["man", "--output-dir", man_dir.to_str().unwrap()])
        .assert_success();

    // Test 'new' subcommand man page
    let new_man_page = man_dir.join("tram-new.1");
    FileAssertions::assert_file_matches(&new_man_page, r"\.TH tram-new 1.*User Commands");
    FileAssertions::assert_file_contains(&new_man_page, "Create a new project interactively");
    FileAssertions::assert_file_contains(&new_man_page, "\\-\\-project\\-type");
    FileAssertions::assert_file_contains(&new_man_page, "\\-\\-description");
    FileAssertions::assert_file_contains(&new_man_page, "\\-\\-skip\\-prompts");

    // Test 'generate' subcommand man page
    let generate_man_page = man_dir.join("tram-generate.1");
    FileAssertions::assert_file_contains(
        &generate_man_page,
        "Generate templates for common CLI patterns",
    );
    FileAssertions::assert_file_contains(&generate_man_page, "\\-\\-template\\-type");
    FileAssertions::assert_file_contains(&generate_man_page, "\\-\\-target\\-dir");
    FileAssertions::assert_file_contains(&generate_man_page, "\\-\\-write");
}

#[test]
fn test_man_page_help() {
    init_tests();

    let output = TramCommand::new().args(["man", "--help"]).assert_success();

    output.assert_stdout_contains("Generate manual pages");
    output.assert_stdout_contains("--output-dir");
    output.assert_stdout_contains("--section");
    output.assert_stdout_contains("Output directory for man pages");
    output.assert_stdout_contains("Generate only specific section");
}

#[test]
fn test_man_page_section_filtering() {
    init_tests();

    let temp_dir = TempDir::new("man-section-test").unwrap();
    let man_dir = temp_dir.path().join("man-section");

    let output = TramCommand::new()
        .args([
            "man",
            "--output-dir",
            man_dir.to_str().unwrap(),
            "--section",
            "1",
        ])
        .assert_success();

    output.assert_stdout_contains("Generated man page:");

    // Verify section 1 files are generated
    FileAssertions::assert_dir_exists(&man_dir);
    assert!(FileAssertions::count_files(&man_dir, r".*\.1$") > 0);

    // Verify no other sections are generated
    assert_eq!(FileAssertions::count_files(&man_dir, r".*\.2$"), 0);
    assert_eq!(FileAssertions::count_files(&man_dir, r".*\.3$"), 0);
}

#[test]
fn test_man_page_custom_output_directory() {
    init_tests();

    let temp_dir = TempDir::new("man-custom-dir-test").unwrap();
    let custom_dir = temp_dir.path().join("custom").join("man-pages");

    let output = TramCommand::new()
        .args(["man", "--output-dir", custom_dir.to_str().unwrap()])
        .assert_success();

    output.assert_stdout_contains(&format!(
        "Manual pages generated in: {}",
        custom_dir.display()
    ));

    // Verify custom directory was created and populated
    FileAssertions::assert_dir_exists(&custom_dir);
    FileAssertions::assert_file_exists(custom_dir.join("tram.1"));
    assert!(FileAssertions::count_files(&custom_dir, r".*\.1$") >= 10);
}

#[test]
fn test_man_page_installation_instructions() {
    init_tests();

    let temp_dir = TempDir::new("man-install-test").unwrap();
    let man_dir = temp_dir.path().join("man");

    let output = TramCommand::new()
        .args(["man", "--output-dir", man_dir.to_str().unwrap()])
        .assert_success();

    let stdout = output.stdout();

    // Check installation instructions are provided
    assert!(stdout.contains("To install system-wide:"));
    assert!(stdout.contains("sudo cp"));
    assert!(stdout.contains("/usr/local/share/man/man1/"));
    assert!(stdout.contains("sudo mandb"));

    // Check local viewing instructions
    assert!(stdout.contains("To view locally:"));
    assert!(stdout.contains("man -M"));
}

#[test]
fn test_man_page_version_info() {
    init_tests();

    let temp_dir = TempDir::new("man-version-test").unwrap();
    let man_dir = temp_dir.path().join("man");

    TramCommand::new()
        .args(["man", "--output-dir", man_dir.to_str().unwrap()])
        .assert_success();

    let generate_man_page = man_dir.join("tram-generate.1");

    // Check that version information is included in subcommand pages
    FileAssertions::assert_file_matches(&generate_man_page, r"tram 0\.1\.0.*User Commands");
}

#[test]
fn test_man_page_overwrite_existing() {
    init_tests();

    let temp_dir = TempDir::new("man-overwrite-test").unwrap();
    let man_dir = temp_dir.path().join("man");

    // Create the directory and a dummy file
    fs::create_dir_all(&man_dir).unwrap();
    fs::write(man_dir.join("tram.1"), "dummy content").unwrap();

    // Generate man pages (should overwrite)
    TramCommand::new()
        .args(["man", "--output-dir", man_dir.to_str().unwrap()])
        .assert_success();

    // Verify file was overwritten with proper content
    FileAssertions::assert_file_exists(man_dir.join("tram.1"));
    FileAssertions::assert_file_contains(man_dir.join("tram.1"), ".TH tram 1");

    // Verify the dummy content was replaced
    let content = fs::read_to_string(man_dir.join("tram.1")).unwrap();
    assert!(!content.contains("dummy content"));
}
