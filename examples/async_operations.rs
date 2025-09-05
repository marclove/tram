//! Async operations example
//!
//! This example demonstrates how to handle asynchronous operations in a Tram CLI,
//! including:
//! - Long-running async tasks
//! - Concurrent operations
//! - Timeout handling
//! - Progress reporting during async work
//! - Graceful error handling with async operations

use async_trait::async_trait;
use clap::Parser;
use miette::Result;
use starbase::{App, AppSession};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{info, warn};

/// Async operations CLI example
#[derive(Parser, Debug)]
#[command(name = "async-example")]
#[command(about = "Demonstrates async operations in CLI applications")]
struct AsyncCli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: AsyncCommand,
}

/// Available async commands
#[derive(Parser, Debug)]
enum AsyncCommand {
    /// Download a file (simulated)
    Download {
        /// URL to download from
        url: String,
        /// Output filename
        #[arg(short, long, default_value = "output.txt")]
        output: String,
        /// Timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },
    /// Process multiple items concurrently
    Batch {
        /// Number of items to process
        #[arg(short, long, default_value = "5")]
        count: usize,
        /// Maximum concurrent operations
        #[arg(short, long, default_value = "3")]
        max_concurrent: usize,
    },
    /// Monitor a service (long-running operation)
    Monitor {
        /// Service URL to monitor
        url: String,
        /// Check interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
        /// Maximum number of checks
        #[arg(short, long, default_value = "10")]
        max_checks: u32,
    },
}

/// Session for async operations
#[derive(Debug, Clone)]
struct AsyncSession {
    verbose: bool,
}

impl AsyncSession {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

#[async_trait]
impl AppSession for AsyncSession {
    async fn startup(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Starting async CLI application");
        }

        // Initialize async resources (HTTP clients, connection pools, etc.)
        Ok(None)
    }

    async fn analyze(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Analyzing async environment");
        }

        // Check network connectivity, validate endpoints, etc.
        Ok(None)
    }

    async fn shutdown(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Shutting down async resources");
        }

        // Clean up connections, flush buffers, etc.
        Ok(None)
    }
}

/// Simulate downloading a file with progress
async fn simulate_download(url: &str, output: &str, timeout_secs: u64) -> Result<()> {
    println!("Starting download: {} -> {}", url, output);

    let download_task = async {
        let total_chunks = 10;

        for chunk in 1..=total_chunks {
            // Simulate network delay
            sleep(Duration::from_millis(500)).await;

            let progress = (chunk as f32 / total_chunks as f32) * 100.0;
            println!("  Progress: {:.1}% ({}/{})", progress, chunk, total_chunks);
        }

        println!("✓ Download completed: {}", output);
        Ok::<(), miette::Error>(())
    };

    // Apply timeout to the operation
    match timeout(Duration::from_secs(timeout_secs), download_task).await {
        Ok(result) => result?,
        Err(_) => {
            return Err(miette::miette!(
                "Download timed out after {} seconds",
                timeout_secs
            ));
        }
    }

    Ok(())
}

/// Simulate processing an individual item
async fn process_item(id: usize, verbose: bool) -> Result<String> {
    if verbose {
        info!("Processing item {}", id);
    }

    // Simulate varying processing times
    let delay = Duration::from_millis(1000 + (id as u64 * 200));
    sleep(delay).await;

    // Simulate occasional failures
    if id == 7 {
        return Err(miette::miette!("Processing failed for item {}", id));
    }

    let result = format!("Result for item {}", id);
    println!("  ✓ Completed item {}: {}", id, result);
    Ok(result)
}

/// Process multiple items with controlled concurrency
async fn process_batch(count: usize, max_concurrent: usize, verbose: bool) -> Result<()> {
    println!(
        "Processing {} items with max {} concurrent operations",
        count, max_concurrent
    );

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrent));
    let mut tasks = Vec::new();

    for i in 1..=count {
        let permit = semaphore.clone();
        let task_verbose = verbose;

        let task = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            process_item(i, task_verbose).await
        });

        tasks.push(task);
    }

    // Collect results
    let mut successful = 0;
    let mut failed = 0;

    for (i, task) in tasks.into_iter().enumerate() {
        match task.await {
            Ok(Ok(_result)) => {
                successful += 1;
            }
            Ok(Err(e)) => {
                warn!("Item {} failed: {}", i + 1, e);
                failed += 1;
            }
            Err(e) => {
                warn!("Task {} panicked: {}", i + 1, e);
                failed += 1;
            }
        }
    }

    println!("\nBatch processing complete:");
    println!("  ✓ Successful: {}", successful);
    println!("  ✗ Failed: {}", failed);

    Ok(())
}

/// Simulate monitoring a service
async fn monitor_service(url: &str, interval: u64, max_checks: u32, verbose: bool) -> Result<()> {
    println!("Monitoring service: {}", url);
    println!(
        "Checking every {} seconds, max {} checks",
        interval, max_checks
    );
    println!("Press Ctrl+C to stop monitoring\n");

    let mut check_count = 0;
    let mut interval_timer = tokio::time::interval(Duration::from_secs(interval));

    loop {
        interval_timer.tick().await;
        check_count += 1;

        if verbose {
            info!("Performing health check {} of {}", check_count, max_checks);
        }

        // Simulate health check with occasional failures
        let is_healthy = check_count % 4 != 0; // Fail every 4th check

        let status = if is_healthy {
            "✓ HEALTHY"
        } else {
            "✗ UNHEALTHY"
        };
        let timestamp = chrono::Utc::now().format("%H:%M:%S");

        println!("[{}] Check {}: {}", timestamp, check_count, status);

        if check_count >= max_checks {
            println!("\nReached maximum number of checks ({})", max_checks);
            break;
        }

        // Allow graceful shutdown with Ctrl+C
        if (tokio::time::timeout(Duration::from_millis(100), tokio::signal::ctrl_c()).await).is_ok()
        {
            println!("\nReceived interrupt signal, stopping monitor...");
            break;
        }
    }

    Ok(())
}

/// Execute the parsed async command
async fn execute_command(command: AsyncCommand, session: &AsyncSession) -> Result<()> {
    match command {
        AsyncCommand::Download {
            url,
            output,
            timeout,
        } => {
            if session.verbose {
                info!("Starting download operation");
            }
            simulate_download(&url, &output, timeout).await?;
        }

        AsyncCommand::Batch {
            count,
            max_concurrent,
        } => {
            if session.verbose {
                info!(
                    "Starting batch processing with {} max concurrent",
                    max_concurrent
                );
            }
            process_batch(count, max_concurrent, session.verbose).await?;
        }

        AsyncCommand::Monitor {
            url,
            interval,
            max_checks,
        } => {
            if session.verbose {
                info!("Starting service monitoring");
            }
            monitor_service(&url, interval, max_checks, session.verbose).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = AsyncCli::parse();

    // Create session with parsed options
    let mut session = AsyncSession::new(cli.verbose);

    // Create starbase app
    let app = App::default();

    // Run the application with session lifecycle
    app.run_with_session(&mut session, |session| async move {
        // Execute the async command
        execute_command(cli.command, &session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
