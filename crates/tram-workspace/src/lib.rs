//! Workspace detection utilities for CLI applications.
//!
//! Provides simple, practical utilities for detecting project roots
//! and working with workspace structures.

use std::path::{Path, PathBuf};
use tram_core::{AppResult, TramError};

/// Simple workspace detector that finds project roots by looking for common indicators.
#[derive(Debug, Clone)]
pub struct WorkspaceDetector {
    current_dir: PathBuf,
}

impl WorkspaceDetector {
    /// Create a new workspace detector starting from the current directory.
    pub fn new() -> AppResult<Self> {
        let current_dir = std::env::current_dir().map_err(|_| TramError::WorkspaceNotFound)?;

        Ok(Self { current_dir })
    }

    /// Create a workspace detector starting from a specific directory.
    pub fn from_dir(dir: PathBuf) -> Self {
        Self { current_dir: dir }
    }

    /// Detect the workspace root by walking up the directory tree.
    pub fn detect_root(&self) -> AppResult<PathBuf> {
        let mut current = self.current_dir.as_path();

        loop {
            if self.is_workspace_root(current) {
                return Ok(current.to_path_buf());
            }

            if let Some(parent) = current.parent() {
                current = parent;
            } else {
                return Err(TramError::WorkspaceNotFound.into());
            }
        }
    }

    /// Check if a directory appears to be a workspace root.
    fn is_workspace_root(&self, path: &Path) -> bool {
        // Version control directories
        if path.join(".git").exists() || path.join(".hg").exists() || path.join(".svn").exists() {
            return true;
        }

        // Common project files
        let project_files = [
            "Cargo.toml",     // Rust
            "package.json",   // Node.js
            "pyproject.toml", // Python
            "setup.py",       // Python
            "go.mod",         // Go
            "build.gradle",   // Gradle
            "pom.xml",        // Maven
            "Makefile",       // Make
            "justfile",       // Just
            ".project",       // Eclipse
        ];

        project_files.iter().any(|&file| path.join(file).exists())
    }
}

impl Default for WorkspaceDetector {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self::from_dir(PathBuf::from(".")))
    }
}

/// Project type detection based on files present.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Rust,
    NodeJs,
    Python,
    Go,
    Java,
    Generic,
}

impl ProjectType {
    /// Detect project type from a directory.
    pub fn detect(path: &Path) -> Option<Self> {
        if path.join("Cargo.toml").exists() {
            Some(ProjectType::Rust)
        } else if path.join("package.json").exists() {
            Some(ProjectType::NodeJs)
        } else if path.join("pyproject.toml").exists() || path.join("setup.py").exists() {
            Some(ProjectType::Python)
        } else if path.join("go.mod").exists() {
            Some(ProjectType::Go)
        } else if path.join("pom.xml").exists() || path.join("build.gradle").exists() {
            Some(ProjectType::Java)
        } else {
            Some(ProjectType::Generic)
        }
    }

    /// Get common ignore patterns for this project type.
    pub fn ignore_patterns(&self) -> &[&str] {
        match self {
            ProjectType::Rust => &["target/", "Cargo.lock"],
            ProjectType::NodeJs => &["node_modules/", "dist/", "build/"],
            ProjectType::Python => &[
                "__pycache__/",
                "*.pyc",
                ".venv/",
                "venv/",
                "dist/",
                "build/",
            ],
            ProjectType::Go => &["vendor/"],
            ProjectType::Java => &["target/", "build/", "*.class"],
            ProjectType::Generic => &["build/", "dist/", "out/"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        assert_eq!(
            ProjectType::detect(temp_dir.path()),
            Some(ProjectType::Rust)
        );
    }

    #[test]
    fn test_workspace_detector() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        let detector = WorkspaceDetector::from_dir(temp_dir.path().to_path_buf());
        let root = detector.detect_root().unwrap();

        assert_eq!(root, temp_dir.path());
    }
}
