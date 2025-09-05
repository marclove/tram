//! CLI testing helpers

use std::path::PathBuf;
use std::process::{Command, Output};

/// Helper for testing CLI applications
#[derive(Debug)]
pub struct CliTestRunner {
    command: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
    current_dir: Option<PathBuf>,
}

impl CliTestRunner {
    /// Create a new CLI test runner for the given command
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
            env: Vec::new(),
            current_dir: None,
        }
    }

    /// Add an argument to the command
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add multiple arguments to the command
    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set an environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Set the current directory for the command
    pub fn current_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.current_dir = Some(dir.into());
        self
    }

    /// Run the command and return the output
    pub async fn run(self) -> Result<TestOutput, std::io::Error> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);

        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        if let Some(dir) = &self.current_dir {
            cmd.current_dir(dir);
        }

        let output = cmd.output()?;
        Ok(TestOutput::new(output))
    }
}

/// Wrapper around process output with testing utilities
#[derive(Debug)]
pub struct TestOutput {
    inner: Output,
}

impl TestOutput {
    fn new(output: Output) -> Self {
        Self { inner: output }
    }

    /// Check if the command succeeded
    pub fn success(&self) -> bool {
        self.inner.status.success()
    }

    /// Get the exit code
    pub fn exit_code(&self) -> Option<i32> {
        self.inner.status.code()
    }

    /// Get stdout as a string
    pub fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.inner.stdout).to_string()
    }

    /// Get stderr as a string
    pub fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.inner.stderr).to_string()
    }

    /// Get the raw output
    pub fn raw(&self) -> &Output {
        &self.inner
    }
}
