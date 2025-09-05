//! Test fixtures for common testing scenarios

use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempDir as TempFileDir};

/// A temporary directory for testing
pub struct TempDir {
    inner: TempFileDir,
}

impl TempDir {
    /// Create a new temporary directory
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            inner: TempFileDir::new()?,
        })
    }

    /// Get the path to the temporary directory
    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Create a file in the temporary directory
    pub fn create_file(&self, name: &str, contents: &str) -> Result<PathBuf, std::io::Error> {
        let path = self.path().join(name);
        std::fs::write(&path, contents)?;
        Ok(path)
    }

    /// Create a subdirectory in the temporary directory
    pub fn create_dir(&self, name: &str) -> Result<PathBuf, std::io::Error> {
        let path = self.path().join(name);
        std::fs::create_dir_all(&path)?;
        Ok(path)
    }
}

/// A temporary file for testing
pub struct TempFile {
    inner: NamedTempFile,
}

impl TempFile {
    /// Create a new temporary file
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            inner: NamedTempFile::new()?,
        })
    }

    /// Get the path to the temporary file
    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Write contents to the temporary file
    pub fn write(&mut self, contents: &str) -> Result<(), std::io::Error> {
        use std::io::Write;
        self.inner.write_all(contents.as_bytes())?;
        Ok(())
    }
}
