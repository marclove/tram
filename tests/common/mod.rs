//! Common utilities for integration tests.
//!
//! This module provides shared infrastructure for testing Tram's functionality,
//! including temporary directory management, CLI execution helpers, and
//! assertion utilities.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Once;

/// Global test setup that runs once across all tests.
static INIT: Once = Once::new();

/// Workspace root directory.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Initialize global test environment.
pub fn init_tests() {
    INIT.call_once(|| {
        // Set up global test environment
        let _ = env_logger::try_init();
    });
}

/// Test temporary directory manager.
pub struct TempDir {
    path: PathBuf,
    cleanup_on_drop: bool,
}

impl TempDir {
    /// Create a new temporary directory for testing.
    pub fn new(test_name: &str) -> std::io::Result<Self> {
        let workspace_root = workspace_root();
        let temp_root = workspace_root.join("test-tmp");

        // Ensure temp root exists
        fs::create_dir_all(&temp_root)?;

        let path = temp_root.join(test_name);

        // Remove any existing directory
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }

        // Create fresh directory
        fs::create_dir_all(&path)?;

        Ok(Self {
            path,
            cleanup_on_drop: true,
        })
    }

    /// Get the path to the temporary directory.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Prevent cleanup on drop (useful for debugging).
    pub fn keep_on_drop(&mut self) {
        self.cleanup_on_drop = false;
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.cleanup_on_drop && self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

/// CLI command builder for integration tests.
pub struct TramCommand {
    command: Command,
}

impl TramCommand {
    /// Create a new command to run the tram CLI.
    pub fn new() -> Self {
        let workspace_root = workspace_root();
        let binary_path = workspace_root.join("target").join("debug").join("tram");
        let mut command = Command::new(binary_path);
        command.current_dir(workspace_root);
        // Disable colored output to make tests more reliable
        command.env("NO_COLOR", "1");
        // Set log level to error to minimize output
        command.env("TRAM_LOG_LEVEL", "error");

        Self { command }
    }

    /// Add an argument to the command.
    pub fn arg<S: AsRef<std::ffi::OsStr>>(mut self, arg: S) -> Self {
        self.command.arg(arg);
        self
    }

    /// Add multiple arguments to the command.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.command.args(args);
        self
    }

    /// Set the current directory for the command.
    pub fn current_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.command.current_dir(dir);
        self
    }

    /// Set an environment variable for the command.
    pub fn env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        self.command.env(key, val);
        self
    }

    /// Execute the command and return the output.
    pub fn output(mut self) -> std::io::Result<Output> {
        self.command.output()
    }

    /// Execute the command and assert it succeeds.
    pub fn assert_success(self) -> TramOutput {
        let output = self.output().expect("Failed to execute command");

        if !output.status.success() {
            panic!(
                "Command failed with status: {}\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        TramOutput::new(output)
    }

    /// Execute the command and assert it fails.
    pub fn assert_failure(self) -> TramOutput {
        let output = self.output().expect("Failed to execute command");

        if output.status.success() {
            panic!(
                "Expected command to fail, but it succeeded\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        TramOutput::new(output)
    }
}

/// Wrapper around command output with helpful assertion methods.
pub struct TramOutput {
    output: Output,
    stdout: String,
    stderr: String,
}

impl TramOutput {
    fn new(output: Output) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Self {
            output,
            stdout,
            stderr,
        }
    }

    /// Get the raw output.
    pub fn output(&self) -> &Output {
        &self.output
    }

    /// Get stdout as a string.
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Get stderr as a string.
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Assert that stdout contains the given text.
    pub fn assert_stdout_contains(&self, text: &str) -> &Self {
        assert!(
            self.stdout.contains(text),
            "stdout does not contain '{}'\nstdout: {}",
            text,
            self.stdout
        );
        self
    }

    /// Assert that stderr contains the given text.
    pub fn assert_stderr_contains(&self, text: &str) -> &Self {
        assert!(
            self.stderr.contains(text),
            "stderr does not contain '{}'\nstderr: {}",
            text,
            self.stderr
        );
        self
    }

    /// Assert that stdout matches a regex pattern.
    pub fn assert_stdout_matches(&self, pattern: &str) -> &Self {
        let re = regex::Regex::new(pattern).expect("Invalid regex pattern");
        assert!(
            re.is_match(&self.stdout),
            "stdout does not match pattern '{}'\nstdout: {}",
            pattern,
            self.stdout
        );
        self
    }
}

/// File system test utilities.
pub struct FileAssertions;

impl FileAssertions {
    /// Assert that a file exists.
    pub fn assert_file_exists<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();
        assert!(path.exists(), "Expected file to exist: {}", path.display());
    }

    /// Assert that a directory exists.
    pub fn assert_dir_exists<P: AsRef<Path>>(path: P) {
        let path = path.as_ref();
        assert!(
            path.exists() && path.is_dir(),
            "Expected directory to exist: {}",
            path.display()
        );
    }

    /// Assert that a file contains specific text.
    pub fn assert_file_contains<P: AsRef<Path>>(path: P, text: &str) {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read file {}: {}", path.display(), e));

        assert!(
            content.contains(text),
            "File {} does not contain '{}'\nContent: {}",
            path.display(),
            text,
            content
        );
    }

    /// Assert that a file matches a regex pattern.
    pub fn assert_file_matches<P: AsRef<Path>>(path: P, pattern: &str) {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read file {}: {}", path.display(), e));

        let re = regex::Regex::new(pattern).expect("Invalid regex pattern");
        assert!(
            re.is_match(&content),
            "File {} does not match pattern '{}'\nContent: {}",
            path.display(),
            pattern,
            content
        );
    }

    /// Count files in a directory matching a pattern.
    pub fn count_files<P: AsRef<Path>>(dir: P, pattern: &str) -> usize {
        let dir = dir.as_ref();
        let re = regex::Regex::new(pattern).expect("Invalid regex pattern");

        fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("Failed to read directory {}: {}", dir.display(), e))
            .filter_map(Result::ok)
            .filter(|entry| !entry.file_name().to_string_lossy().starts_with("."))
            .filter(|entry| re.is_match(&entry.file_name().to_string_lossy()))
            .count()
    }
}
