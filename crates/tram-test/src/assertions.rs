//! Custom assertion macros for testing

/// Assert that a CLI command succeeds
#[macro_export]
macro_rules! assert_success {
    ($output:expr) => {
        assert!(
            $output.success(),
            "Command failed with exit code: {:?}\nStdout: {}\nStderr: {}",
            $output.exit_code(),
            $output.stdout(),
            $output.stderr()
        );
    };
}

/// Assert that a CLI command fails
#[macro_export]
macro_rules! assert_failure {
    ($output:expr) => {
        assert!(
            !$output.success(),
            "Command unexpectedly succeeded\nStdout: {}\nStderr: {}",
            $output.stdout(),
            $output.stderr()
        );
    };
}

/// Assert that stdout contains a specific string
#[macro_export]
macro_rules! assert_stdout_contains {
    ($output:expr, $expected:expr) => {
        let stdout = $output.stdout();
        assert!(
            stdout.contains($expected),
            "Stdout does not contain '{}'\nActual stdout: {}",
            $expected,
            stdout
        );
    };
}

/// Assert that stderr contains a specific string
#[macro_export]
macro_rules! assert_stderr_contains {
    ($output:expr, $expected:expr) => {
        let stderr = $output.stderr();
        assert!(
            stderr.contains($expected),
            "Stderr does not contain '{}'\nActual stderr: {}",
            $expected,
            stderr
        );
    };
}

/// Assert that a file exists
#[macro_export]
macro_rules! assert_file_exists {
    ($path:expr) => {
        assert!($path.exists(), "File does not exist: {}", $path.display());
    };
}

/// Assert that a directory exists
#[macro_export]
macro_rules! assert_dir_exists {
    ($path:expr) => {
        assert!(
            $path.exists() && $path.is_dir(),
            "Directory does not exist: {}",
            $path.display()
        );
    };
}
