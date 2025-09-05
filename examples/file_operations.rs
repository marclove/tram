//! File operations example
//!
//! This example demonstrates common file system operations in CLI applications,
//! including:
//! - Reading and writing files
//! - Directory traversal and manipulation
//! - File watching and monitoring
//! - Backup and restore operations
//! - File validation and checksums
//! - Temporary file handling

use async_trait::async_trait;
use clap::Parser;
use glob::glob;
use miette::Result;
use starbase::{App, AppSession};
use std::fs;
// use std::io::Write; // Not needed for current functionality
use std::path::{Path, PathBuf};
use tokio::time::{Duration, sleep};
use tracing::{info, warn};
use walkdir::WalkDir;

/// File operations CLI example
#[derive(Parser, Debug)]
#[command(name = "file-ops-example")]
#[command(about = "Demonstrates file system operations and utilities")]
struct FileOpsCli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Target directory for operations
    #[arg(short, long, default_value = "./temp_demo")]
    target_dir: PathBuf,

    /// Subcommand to execute
    #[command(subcommand)]
    command: FileOpsCommand,
}

/// Available file operation demonstrations
#[derive(Parser, Debug)]
enum FileOpsCommand {
    /// Basic file read/write operations
    BasicOperations,
    /// Directory traversal and listing
    DirectoryOps {
        /// Directory to traverse
        #[arg(short, long)]
        directory: Option<PathBuf>,
        /// Show hidden files
        #[arg(long)]
        show_hidden: bool,
        /// Recursive traversal
        #[arg(short, long)]
        recursive: bool,
    },
    /// File searching with patterns
    Search {
        /// Pattern to search for
        pattern: String,
        /// Directory to search in
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
        /// Case insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },
    /// File backup and restore
    Backup {
        /// Source file or directory
        source: PathBuf,
        /// Backup destination
        #[arg(short, long)]
        destination: Option<PathBuf>,
    },
    /// File validation and checksums
    Validate {
        /// File to validate
        file: PathBuf,
        /// Expected checksum (optional)
        #[arg(long)]
        expected_checksum: Option<String>,
    },
    /// Temporary file operations
    TempFiles,
    /// File watching demonstration
    Watch {
        /// Directory to watch
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
        /// Watch duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,
    },
    /// File cleanup and maintenance
    Cleanup {
        /// Dry run (don't actually delete)
        #[arg(long)]
        dry_run: bool,
        /// File age threshold in days
        #[arg(long, default_value = "30")]
        days_old: u64,
    },
}

/// Session for file operations
#[derive(Debug, Clone)]
struct FileOpsSession {
    verbose: bool,
    target_dir: PathBuf,
}

impl FileOpsSession {
    fn new(verbose: bool, target_dir: PathBuf) -> Self {
        Self {
            verbose,
            target_dir,
        }
    }
}

#[async_trait]
impl AppSession for FileOpsSession {
    async fn startup(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Starting file operations example");
        }

        // Ensure target directory exists
        if !self.target_dir.exists() {
            fs::create_dir_all(&self.target_dir)
                .map_err(|e| miette::miette!("Failed to create target directory: {}", e))?;
        }

        Ok(None)
    }

    async fn analyze(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!(
                "File operations ready, target dir: {}",
                self.target_dir.display()
            );
        }
        Ok(None)
    }

    async fn shutdown(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("File operations example complete");
        }
        Ok(None)
    }
}

/// Demonstrate basic file read/write operations
fn demo_basic_operations(target_dir: &Path) -> Result<()> {
    println!("=== Basic File Operations ===\n");

    // Create sample files
    let file1 = target_dir.join("sample1.txt");
    let file2 = target_dir.join("sample2.json");
    let file3 = target_dir.join("data.csv");

    // Write different types of content
    fs::write(&file1, "Hello, World!\nThis is a sample text file.\nLine 3")
        .map_err(|e| miette::miette!("Failed to write {}: {}", file1.display(), e))?;

    let json_content = r#"{
  "name": "Tram Example",
  "version": "1.0.0",
  "features": ["cli", "async", "config"],
  "active": true
}"#;
    fs::write(&file2, json_content)
        .map_err(|e| miette::miette!("Failed to write {}: {}", file2.display(), e))?;

    let csv_content = "name,age,city\nAlice,30,New York\nBob,25,San Francisco\nCharlie,35,Chicago";
    fs::write(&file3, csv_content)
        .map_err(|e| miette::miette!("Failed to write {}: {}", file3.display(), e))?;

    println!("✓ Created sample files:");
    println!("  - {}", file1.display());
    println!("  - {}", file2.display());
    println!("  - {}", file3.display());

    // Read and display file contents
    println!("\n📄 File Contents:");

    let content1 = fs::read_to_string(&file1)
        .map_err(|e| miette::miette!("Failed to read {}: {}", file1.display(), e))?;
    println!(
        "\n{}: {} bytes",
        file1.file_name().unwrap().to_string_lossy(),
        content1.len()
    );
    println!("{}", content1);

    // Read file metadata
    let metadata = fs::metadata(&file2)
        .map_err(|e| miette::miette!("Failed to get metadata for {}: {}", file2.display(), e))?;

    println!(
        "\n📊 File Metadata ({})",
        file2.file_name().unwrap().to_string_lossy()
    );
    println!("  Size: {} bytes", metadata.len());
    println!("  Read-only: {}", metadata.permissions().readonly());
    if let Ok(modified) = metadata.modified() {
        println!("  Modified: {:?}", modified);
    }

    // Copy and rename operations
    let file1_copy = target_dir.join("sample1_copy.txt");
    fs::copy(&file1, &file1_copy).map_err(|e| miette::miette!("Failed to copy file: {}", e))?;
    println!("\n✓ Copied {} to {}", file1.display(), file1_copy.display());

    println!();
    Ok(())
}

/// Demonstrate directory operations
fn demo_directory_ops(
    directory: Option<PathBuf>,
    show_hidden: bool,
    recursive: bool,
) -> Result<()> {
    println!("=== Directory Operations ===\n");

    let target_dir = directory.unwrap_or_else(|| PathBuf::from("."));

    if !target_dir.exists() {
        return Err(miette::miette!(
            "Directory does not exist: {}",
            target_dir.display()
        ));
    }

    println!("📂 Listing contents of: {}", target_dir.display());

    if recursive {
        println!("🔍 Recursive traversal:");
        for entry in WalkDir::new(&target_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            // Skip hidden files unless requested
            if !show_hidden && name.starts_with('.') && name != "." && name != ".." {
                continue;
            }

            let depth = "  ".repeat(entry.depth());
            let file_type = if path.is_dir() { "📁" } else { "📄" };

            if let Ok(metadata) = fs::metadata(path) {
                println!(
                    "{}{} {} ({} bytes)",
                    depth,
                    file_type,
                    path.display(),
                    metadata.len()
                );
            } else {
                println!("{}{} {}", depth, file_type, path.display());
            }
        }
    } else {
        println!("📋 Directory listing:");
        let entries = fs::read_dir(&target_dir)
            .map_err(|e| miette::miette!("Failed to read directory: {}", e))?;

        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| miette::miette!("Failed to read entry: {}", e))?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files unless requested
            if !show_hidden && name.starts_with('.') {
                continue;
            }

            if path.is_dir() {
                dirs.push((name, path));
            } else {
                let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                files.push((name, path, size));
            }
        }

        // Sort and display directories first
        dirs.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, _path) in dirs {
            println!("  📁 {}/", name);
        }

        // Then display files
        files.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, _path, size) in files {
            println!("  📄 {} ({} bytes)", name, size);
        }
    }

    // Directory statistics
    let mut total_files = 0;
    let mut total_dirs = 0;
    let mut total_size = 0;

    for entry in WalkDir::new(&target_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            total_dirs += 1;
        } else {
            total_files += 1;
            if let Ok(metadata) = fs::metadata(entry.path()) {
                total_size += metadata.len();
            }
        }
    }

    println!("\n📊 Statistics:");
    println!("  Directories: {}", total_dirs);
    println!("  Files: {}", total_files);
    println!("  Total size: {} bytes", total_size);

    println!();
    Ok(())
}

/// Demonstrate file searching with patterns
fn demo_search(pattern: &str, directory: &Path, ignore_case: bool) -> Result<()> {
    println!("=== File Search ===\n");

    println!(
        "🔍 Searching for pattern: '{}' in {}",
        pattern,
        directory.display()
    );

    if ignore_case {
        println!("   (case insensitive)");
    }

    // Use glob for pattern matching
    let search_pattern = if directory == Path::new(".") {
        pattern.to_string()
    } else {
        format!("{}/{}", directory.display(), pattern)
    };

    println!("\n📄 Matching files:");
    let mut found_count = 0;

    match glob(&search_pattern) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        if let Ok(metadata) = fs::metadata(&path) {
                            let file_type = if path.is_dir() { "📁" } else { "📄" };
                            println!(
                                "  {} {} ({} bytes)",
                                file_type,
                                path.display(),
                                metadata.len()
                            );
                            found_count += 1;
                        }
                    }
                    Err(e) => warn!("Error processing path: {}", e),
                }
            }
        }
        Err(e) => {
            return Err(miette::miette!(
                "Failed to search with pattern '{}': {}",
                pattern,
                e
            ));
        }
    }

    if found_count == 0 {
        println!("  No files found matching pattern '{}'", pattern);
    } else {
        println!("\n✓ Found {} matching file(s)", found_count);
    }

    // Search for content within files (simple text search)
    if !pattern.contains('*') && !pattern.contains('?') {
        println!(
            "\n🔎 Searching for content '{}' within text files:",
            pattern
        );

        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            let path = entry.path();

            // Only search in text-like files
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(
                    ext_str.as_str(),
                    "txt" | "rs" | "js" | "py" | "json" | "yaml" | "toml" | "md"
                ) && let Ok(content) = fs::read_to_string(path)
                {
                    let search_content = if ignore_case {
                        content.to_lowercase()
                    } else {
                        content.clone()
                    };

                    let search_pattern = if ignore_case {
                        pattern.to_lowercase()
                    } else {
                        pattern.to_string()
                    };

                    if search_content.contains(&search_pattern) {
                        let lines: Vec<&str> = content.lines().collect();
                        println!("  📄 {}", path.display());

                        for (line_num, line) in lines.iter().enumerate() {
                            let search_line = if ignore_case {
                                line.to_lowercase()
                            } else {
                                line.to_string()
                            };

                            if search_line.contains(&search_pattern) {
                                println!("    Line {}: {}", line_num + 1, line.trim());
                            }
                        }
                    }
                }
            }
        }
    }

    println!();
    Ok(())
}

/// Demonstrate backup operations
fn demo_backup(source: &Path, destination: Option<PathBuf>) -> Result<()> {
    println!("=== File Backup ===\n");

    if !source.exists() {
        return Err(miette::miette!(
            "Source path does not exist: {}",
            source.display()
        ));
    }

    let backup_name = format!(
        "{}_backup_{}",
        source.file_name().unwrap().to_string_lossy(),
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );

    let backup_path = if let Some(dest) = destination {
        dest.join(&backup_name)
    } else {
        source.parent().unwrap_or(Path::new(".")).join(&backup_name)
    };

    println!("💾 Creating backup:");
    println!("  Source: {}", source.display());
    println!("  Backup: {}", backup_path.display());

    if source.is_file() {
        fs::copy(source, &backup_path)
            .map_err(|e| miette::miette!("Failed to backup file: {}", e))?;

        let original_size = fs::metadata(source)
            .map_err(|e| miette::miette!("Failed to read source metadata: {}", e))?
            .len();
        let backup_size = fs::metadata(&backup_path)
            .map_err(|e| miette::miette!("Failed to read backup metadata: {}", e))?
            .len();

        println!("  ✓ File backed up ({} bytes)", backup_size);

        if original_size != backup_size {
            warn!(
                "Backup size mismatch: original {} bytes, backup {} bytes",
                original_size, backup_size
            );
        }
    } else if source.is_dir() {
        fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
            fs::create_dir_all(dst).map_err(|e| {
                miette::miette!("Failed to create directory {}: {}", dst.display(), e)
            })?;

            for entry in fs::read_dir(src)
                .map_err(|e| miette::miette!("Failed to read directory {}: {}", src.display(), e))?
            {
                let entry =
                    entry.map_err(|e| miette::miette!("Failed to read directory entry: {}", e))?;

                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());

                if src_path.is_dir() {
                    copy_dir(&src_path, &dst_path)?;
                } else {
                    fs::copy(&src_path, &dst_path).map_err(|e| {
                        miette::miette!("Failed to copy {}: {}", src_path.display(), e)
                    })?;
                }
            }

            Ok(())
        }

        copy_dir(source, &backup_path)?;
        println!("  ✓ Directory backed up recursively");
    }

    // Verify backup integrity
    println!("\n🔍 Verifying backup integrity...");

    if backup_path.exists() {
        println!("  ✓ Backup exists");

        // Simple verification by comparing file sizes
        if source.is_file() && backup_path.is_file() {
            let original_size = fs::metadata(source)
                .map_err(|e| miette::miette!("Failed to read source metadata: {}", e))?
                .len();
            let backup_size = fs::metadata(&backup_path)
                .map_err(|e| miette::miette!("Failed to read backup metadata: {}", e))?
                .len();

            if original_size == backup_size {
                println!("  ✓ File sizes match");
            } else {
                println!("  ⚠️  File size mismatch");
            }
        }
    } else {
        return Err(miette::miette!(
            "Backup verification failed: backup not found"
        ));
    }

    println!("\n✓ Backup completed successfully");
    println!();
    Ok(())
}

/// Demonstrate file validation and checksums
fn demo_validate(file: &Path, expected_checksum: Option<String>) -> Result<()> {
    println!("=== File Validation ===\n");

    if !file.exists() {
        return Err(miette::miette!("File does not exist: {}", file.display()));
    }

    println!("🔍 Validating file: {}", file.display());

    // Basic file information
    let metadata =
        fs::metadata(file).map_err(|e| miette::miette!("Failed to get file metadata: {}", e))?;

    println!("\n📊 File Information:");
    println!("  Size: {} bytes", metadata.len());
    println!("  Read-only: {}", metadata.permissions().readonly());

    if let Ok(modified) = metadata.modified() {
        println!("  Modified: {:?}", modified);
    }

    // Simple checksum calculation (using a basic hash for demonstration)
    let content = fs::read(file).map_err(|e| miette::miette!("Failed to read file: {}", e))?;

    let checksum = format!("{:x}", md5::compute(&content));

    println!("\n🔐 Checksum (MD5): {}", checksum);

    if let Some(expected) = expected_checksum {
        if checksum == expected {
            println!("  ✓ Checksum matches expected value");
        } else {
            println!("  ❌ Checksum mismatch!");
            println!("     Expected: {}", expected);
            println!("     Actual:   {}", checksum);
            return Err(miette::miette!("File checksum validation failed"));
        }
    }

    // File type validation
    println!("\n🔍 File Type Analysis:");

    if let Some(extension) = file.extension() {
        println!("  Extension: .{}", extension.to_string_lossy());
    }

    // Simple content type detection
    let first_bytes = &content[..content.len().min(16)];
    println!("  First 16 bytes: {:02x?}", first_bytes);

    // Check for common file signatures
    if content.starts_with(b"#!/") {
        println!("  ✓ Detected: Shell script or executable");
    } else if content.starts_with(b"<?xml") {
        println!("  ✓ Detected: XML document");
    } else if content.starts_with(b"{") || content.starts_with(b"[") {
        println!("  ✓ Detected: Likely JSON document");
    } else if content.iter().all(|&b| b.is_ascii()) {
        println!("  ✓ Detected: ASCII text file");
    } else {
        println!("  ℹ️  Binary or non-ASCII file");
    }

    println!("\n✓ File validation complete");
    println!();
    Ok(())
}

/// Demonstrate temporary file operations
fn demo_temp_files(target_dir: &Path) -> Result<()> {
    println!("=== Temporary File Operations ===\n");

    // Create temporary directory
    let temp_dir = target_dir.join("temp_operations");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| miette::miette!("Failed to create temp directory: {}", e))?;

    println!("📁 Created temporary directory: {}", temp_dir.display());

    // Create temporary files with different patterns
    let temp_files = vec![
        ("temp_data.txt", "Temporary data file\nWith multiple lines"),
        (
            "temp_config.json",
            r#"{"temp": true, "timestamp": "2024-01-01T00:00:00Z"}"#,
        ),
        (
            "temp_script.sh",
            "#!/bin/bash\necho 'This is a temporary script'",
        ),
    ];

    for (filename, content) in &temp_files {
        let file_path = temp_dir.join(filename);
        fs::write(&file_path, content)
            .map_err(|e| miette::miette!("Failed to write temp file {}: {}", filename, e))?;

        println!("  ✓ Created: {}", filename);
    }

    // Simulate working with temporary files
    println!("\n🔄 Working with temporary files...");

    for (filename, _) in &temp_files {
        let file_path = temp_dir.join(filename);

        if let Ok(content) = fs::read_to_string(&file_path) {
            println!("  📄 {} ({} chars)", filename, content.len());
        }
    }

    // Cleanup temporary files
    println!("\n🧹 Cleaning up temporary files...");

    for (filename, _) in &temp_files {
        let file_path = temp_dir.join(filename);
        fs::remove_file(&file_path)
            .map_err(|e| miette::miette!("Failed to remove temp file {}: {}", filename, e))?;

        println!("  ✓ Removed: {}", filename);
    }

    // Remove temporary directory
    fs::remove_dir(&temp_dir)
        .map_err(|e| miette::miette!("Failed to remove temp directory: {}", e))?;

    println!("  ✓ Removed temporary directory");
    println!("\n✓ Temporary file operations complete");
    println!();
    Ok(())
}

/// Demonstrate file watching (simplified version)
async fn demo_watch(directory: &Path, duration: u64) -> Result<()> {
    println!("=== File Watching ===\n");

    if !directory.exists() {
        return Err(miette::miette!(
            "Watch directory does not exist: {}",
            directory.display()
        ));
    }

    println!("👀 Watching directory: {}", directory.display());
    println!("   Duration: {} seconds", duration);
    println!("   Try creating, modifying, or deleting files in another terminal!\n");

    let mut last_scan = std::collections::HashMap::new();

    // Initial scan
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = fs::metadata(entry.path()) {
            last_scan.insert(
                entry.path().to_path_buf(),
                (metadata.len(), metadata.modified().ok()),
            );
        }
    }

    let end_time = tokio::time::Instant::now() + Duration::from_secs(duration);
    let mut check_count = 0;

    while tokio::time::Instant::now() < end_time {
        check_count += 1;

        let mut current_scan = std::collections::HashMap::new();
        let mut changes_detected = false;

        // Scan for changes
        for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
            if let Ok(metadata) = fs::metadata(entry.path()) {
                let current_state = (metadata.len(), metadata.modified().ok());
                current_scan.insert(entry.path().to_path_buf(), current_state);

                // Check for changes
                match last_scan.get(entry.path()) {
                    Some(old_state) if old_state != &current_state => {
                        println!("🔄 Modified: {}", entry.path().display());
                        changes_detected = true;
                    }
                    None => {
                        println!("✨ Created: {}", entry.path().display());
                        changes_detected = true;
                    }
                    _ => {}
                }
            }
        }

        // Check for deleted files
        for old_path in last_scan.keys() {
            if !current_scan.contains_key(old_path) {
                println!("🗑️  Deleted: {}", old_path.display());
                changes_detected = true;
            }
        }

        if changes_detected {
            println!(
                "   [Check #{} at {}]",
                check_count,
                chrono::Utc::now().format("%H:%M:%S")
            );
        }

        last_scan = current_scan;

        // Check every 2 seconds
        sleep(Duration::from_secs(2)).await;
    }

    println!(
        "\n✓ File watching completed ({} checks performed)",
        check_count
    );
    println!();
    Ok(())
}

/// Demonstrate cleanup operations
fn demo_cleanup(target_dir: &Path, dry_run: bool, days_old: u64) -> Result<()> {
    println!("=== File Cleanup ===\n");

    if dry_run {
        println!("🧪 DRY RUN MODE - No files will actually be deleted");
    }

    let cutoff_time =
        std::time::SystemTime::now() - std::time::Duration::from_secs(days_old * 24 * 60 * 60);

    println!("🧹 Cleaning up files older than {} days", days_old);
    println!("   Target directory: {}", target_dir.display());

    let mut files_to_clean = Vec::new();
    let mut total_size = 0;

    // Find old files
    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file()
            && let Ok(metadata) = fs::metadata(entry.path())
            && let Ok(modified) = metadata.modified()
            && modified < cutoff_time
        {
            files_to_clean.push((entry.path().to_path_buf(), metadata.len()));
            total_size += metadata.len();
        }
    }

    if files_to_clean.is_empty() {
        println!("✓ No old files found for cleanup");
        println!();
        return Ok(());
    }

    println!(
        "\n📋 Files to clean ({} files, {} bytes total):",
        files_to_clean.len(),
        total_size
    );

    for (path, size) in &files_to_clean {
        println!("  🗑️  {} ({} bytes)", path.display(), size);
    }

    if !dry_run {
        println!("\n🗑️ Removing old files...");

        let mut removed_count = 0;
        let mut removed_size = 0;

        for (path, size) in files_to_clean {
            match fs::remove_file(&path) {
                Ok(_) => {
                    println!("  ✓ Removed: {}", path.display());
                    removed_count += 1;
                    removed_size += size;
                }
                Err(e) => {
                    warn!("Failed to remove {}: {}", path.display(), e);
                }
            }
        }

        println!(
            "\n✓ Cleanup complete: {} files removed ({} bytes freed)",
            removed_count, removed_size
        );
    } else {
        println!(
            "\n✅ Dry run complete - {} files would be removed ({} bytes)",
            files_to_clean.len(),
            total_size
        );
    }

    println!();
    Ok(())
}

/// Execute the parsed file operations command
async fn execute_command(command: FileOpsCommand, session: &FileOpsSession) -> Result<()> {
    match command {
        FileOpsCommand::BasicOperations => {
            demo_basic_operations(&session.target_dir)?;
        }

        FileOpsCommand::DirectoryOps {
            directory,
            show_hidden,
            recursive,
        } => {
            demo_directory_ops(directory, show_hidden, recursive)?;
        }

        FileOpsCommand::Search {
            pattern,
            directory,
            ignore_case,
        } => {
            demo_search(&pattern, &directory, ignore_case)?;
        }

        FileOpsCommand::Backup {
            source,
            destination,
        } => {
            demo_backup(&source, destination)?;
        }

        FileOpsCommand::Validate {
            file,
            expected_checksum,
        } => {
            demo_validate(&file, expected_checksum)?;
        }

        FileOpsCommand::TempFiles => {
            demo_temp_files(&session.target_dir)?;
        }

        FileOpsCommand::Watch {
            directory,
            duration,
        } => {
            demo_watch(&directory, duration).await?;
        }

        FileOpsCommand::Cleanup { dry_run, days_old } => {
            demo_cleanup(&session.target_dir, dry_run, days_old)?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = FileOpsCli::parse();

    // Create session with options
    let mut session = FileOpsSession::new(cli.verbose, cli.target_dir.clone());

    // Create starbase app
    let app = App::default();

    // Run the application with session lifecycle
    app.run_with_session(&mut session, |session| async move {
        // Execute the file operations command
        execute_command(cli.command, &session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
