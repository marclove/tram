//! Progress indicators example
//!
//! This example demonstrates how to use terminal UI components for progress indication,
//! including:
//! - Progress bars for long-running tasks
//! - Spinners for indeterminate operations
//! - Multi-step progress tracking
//! - Styled terminal output with colors
//! - Progress reporting with ETA calculations

use async_trait::async_trait;
use clap::Parser;
use miette::Result;
use starbase::{App, AppSession};
use std::time::Duration;
use tokio::time::{Instant, sleep};
use tracing::info;

/// Progress indicators CLI example
#[derive(Parser, Debug)]
#[command(name = "progress-example")]
#[command(about = "Demonstrates progress indicators and terminal UI")]
struct ProgressCli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: ProgressCommand,
}

/// Available progress demonstration commands
#[derive(Parser, Debug)]
enum ProgressCommand {
    /// Show a simple progress bar
    ProgressBar {
        /// Number of steps to process
        #[arg(short, long, default_value = "50")]
        steps: usize,
        /// Delay between steps in milliseconds
        #[arg(short, long, default_value = "100")]
        delay: u64,
    },
    /// Show a spinner for indeterminate progress
    Spinner {
        /// Duration to spin in seconds
        #[arg(short, long, default_value = "10")]
        duration: u64,
    },
    /// Multi-step progress with different phases
    MultiStep {
        /// Number of items per phase
        #[arg(short, long, default_value = "10")]
        items_per_phase: usize,
        /// Delay per item in milliseconds
        #[arg(short, long, default_value = "200")]
        delay: u64,
    },
    /// Concurrent progress bars
    Concurrent {
        /// Number of concurrent tasks
        #[arg(short, long, default_value = "3")]
        tasks: usize,
        /// Maximum steps per task
        #[arg(short, long, default_value = "20")]
        max_steps: usize,
    },
    /// File processing simulation
    FileProcessing {
        /// Number of files to process
        #[arg(short, long, default_value = "25")]
        files: usize,
        /// Processing delay per file in milliseconds
        #[arg(short, long, default_value = "150")]
        delay: u64,
    },
}

/// Session for progress examples
#[derive(Debug, Clone)]
struct ProgressSession {
    verbose: bool,
    use_color: bool,
}

impl ProgressSession {
    fn new(verbose: bool, use_color: bool) -> Self {
        Self { verbose, use_color }
    }
}

#[async_trait]
impl AppSession for ProgressSession {
    async fn startup(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Starting progress indicators example");
        }
        Ok(None)
    }

    async fn analyze(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Progress example ready");
        }
        Ok(None)
    }

    async fn shutdown(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Progress example complete");
        }
        Ok(None)
    }
}

/// Simple progress bar implementation
struct ProgressBar {
    current: usize,
    total: usize,
    width: usize,
    start_time: Instant,
    use_color: bool,
}

impl ProgressBar {
    fn new(total: usize, use_color: bool) -> Self {
        Self {
            current: 0,
            total,
            width: 50,
            start_time: Instant::now(),
            use_color,
        }
    }

    fn update(&mut self, current: usize) {
        self.current = current;
        self.render();
    }

    fn finish(&self) {
        println!();
        let elapsed = self.start_time.elapsed();
        if self.use_color {
            println!(
                "\x1b[32m✓ Completed in {:.2}s\x1b[0m",
                elapsed.as_secs_f64()
            );
        } else {
            println!("✓ Completed in {:.2}s", elapsed.as_secs_f64());
        }
    }

    fn render(&self) {
        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64 * 100.0) as usize
        } else {
            0
        };

        let filled = (self.current as f64 / self.total as f64 * self.width as f64) as usize;
        let empty = self.width - filled;

        let elapsed = self.start_time.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 {
            self.current as f64 / elapsed
        } else {
            0.0
        };
        let eta = if rate > 0.0 && self.current < self.total {
            (self.total - self.current) as f64 / rate
        } else {
            0.0
        };

        if self.use_color {
            print!(
                "\r\x1b[K\x1b[36m[\x1b[32m{}\x1b[37m{}\x1b[36m] \x1b[33m{:3}%\x1b[0m {}/{} \x1b[90m({:.1}/s, ETA: {:.0}s)\x1b[0m",
                "=".repeat(filled),
                "-".repeat(empty),
                percentage,
                self.current,
                self.total,
                rate,
                eta
            );
        } else {
            print!(
                "\r\x1b[K[{}{}] {:3}% {}/{} ({:.1}/s, ETA: {:.0}s)",
                "=".repeat(filled),
                "-".repeat(empty),
                percentage,
                self.current,
                self.total,
                rate,
                eta
            );
        }
        use std::io::{self, Write};
        let _ = io::stdout().flush();
    }
}

/// Simple spinner implementation
struct Spinner {
    frames: Vec<&'static str>,
    current_frame: usize,
    use_color: bool,
}

impl Spinner {
    fn new(use_color: bool) -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current_frame: 0,
            use_color,
        }
    }

    fn update(&mut self, message: &str) {
        let frame = self.frames[self.current_frame];
        self.current_frame = (self.current_frame + 1) % self.frames.len();

        if self.use_color {
            print!("\r\x1b[K\x1b[36m{}\x1b[0m {}", frame, message);
        } else {
            print!("\r\x1b[K{} {}", frame, message);
        }
        use std::io::{self, Write};
        let _ = io::stdout().flush();
    }

    fn finish(&self, message: &str) {
        if self.use_color {
            println!("\r\x1b[K\x1b[32m✓\x1b[0m {}", message);
        } else {
            println!("\r\x1b[K✓ {}", message);
        }
    }
}

/// Demonstrate simple progress bar
async fn demo_progress_bar(steps: usize, delay: u64, use_color: bool) -> Result<()> {
    println!("Demonstrating progress bar ({} steps):", steps);

    let mut progress = ProgressBar::new(steps, use_color);

    for i in 0..=steps {
        progress.update(i);
        if i < steps {
            sleep(Duration::from_millis(delay)).await;
        }
    }

    progress.finish();
    Ok(())
}

/// Demonstrate spinner for indeterminate progress
async fn demo_spinner(duration: u64, use_color: bool) -> Result<()> {
    println!("Demonstrating spinner ({}s):", duration);

    let mut spinner = Spinner::new(use_color);
    let start = Instant::now();

    while start.elapsed().as_secs() < duration {
        let elapsed = start.elapsed().as_secs();
        let remaining = duration - elapsed;

        spinner.update(&format!("Processing... ({}s remaining)", remaining));
        sleep(Duration::from_millis(100)).await;
    }

    spinner.finish("Processing complete!");
    Ok(())
}

/// Demonstrate multi-step progress
async fn demo_multi_step(items_per_phase: usize, delay: u64, use_color: bool) -> Result<()> {
    let phases = vec![
        ("Initializing", items_per_phase),
        ("Processing", items_per_phase * 2),
        ("Validating", items_per_phase),
        ("Finalizing", items_per_phase / 2),
    ];

    println!("Demonstrating multi-step progress:");

    for (phase_name, items) in phases {
        if use_color {
            println!("\n\x1b[1m{}\x1b[0m", phase_name);
        } else {
            println!("\n{}", phase_name);
        }

        let mut progress = ProgressBar::new(items, use_color);

        for i in 0..=items {
            progress.update(i);
            if i < items {
                sleep(Duration::from_millis(delay)).await;
            }
        }

        progress.finish();
    }

    if use_color {
        println!("\n\x1b[32m🎉 All phases completed successfully!\x1b[0m");
    } else {
        println!("\n🎉 All phases completed successfully!");
    }

    Ok(())
}

/// Demonstrate concurrent progress bars
async fn demo_concurrent(tasks: usize, max_steps: usize, use_color: bool) -> Result<()> {
    println!("Demonstrating concurrent progress (simulated):");
    println!(
        "Note: This example shows the pattern - real concurrent progress would require more complex terminal handling\n"
    );

    let mut task_handles = Vec::new();

    for task_id in 1..=tasks {
        let task_use_color = use_color;
        let task_steps = max_steps - (task_id * 2); // Vary the number of steps

        let handle = tokio::spawn(async move {
            let mut progress = ProgressBar::new(task_steps, task_use_color);

            for i in 0..=task_steps {
                progress.update(i);

                if i < task_steps {
                    // Vary delay to simulate different task speeds
                    let delay = 100 + (task_id as u64 * 50);
                    sleep(Duration::from_millis(delay)).await;
                }
            }

            progress.finish();
            println!("Task {} completed", task_id);
        });

        task_handles.push(handle);

        // Small delay between starting tasks
        sleep(Duration::from_millis(200)).await;
    }

    // Wait for all tasks to complete
    for handle in task_handles {
        handle
            .await
            .map_err(|e| miette::miette!("Task failed: {}", e))?;
    }

    if use_color {
        println!("\n\x1b[32m✓ All concurrent tasks completed!\x1b[0m");
    } else {
        println!("\n✓ All concurrent tasks completed!");
    }

    Ok(())
}

/// Demonstrate file processing with progress
async fn demo_file_processing(files: usize, delay: u64, use_color: bool) -> Result<()> {
    println!("Demonstrating file processing progress:");

    let file_names = (1..=files)
        .map(|i| format!("file_{:03}.txt", i))
        .collect::<Vec<_>>();

    let mut progress = ProgressBar::new(files, use_color);

    for (i, filename) in file_names.iter().enumerate() {
        progress.update(i);

        // Show current file being processed
        if use_color {
            println!("\n\x1b[90mProcessing: {}\x1b[0m", filename);
        } else {
            println!("\nProcessing: {}", filename);
        }

        // Simulate file processing
        sleep(Duration::from_millis(delay)).await;

        // Show completion
        if use_color {
            println!("\x1b[32m  ✓ Completed: {}\x1b[0m", filename);
        } else {
            println!("  ✓ Completed: {}", filename);
        }
    }

    progress.update(files);
    progress.finish();

    Ok(())
}

/// Execute the parsed progress command
async fn execute_command(command: ProgressCommand, session: &ProgressSession) -> Result<()> {
    match command {
        ProgressCommand::ProgressBar { steps, delay } => {
            demo_progress_bar(steps, delay, session.use_color).await?;
        }

        ProgressCommand::Spinner { duration } => {
            demo_spinner(duration, session.use_color).await?;
        }

        ProgressCommand::MultiStep {
            items_per_phase,
            delay,
        } => {
            demo_multi_step(items_per_phase, delay, session.use_color).await?;
        }

        ProgressCommand::Concurrent { tasks, max_steps } => {
            demo_concurrent(tasks, max_steps, session.use_color).await?;
        }

        ProgressCommand::FileProcessing { files, delay } => {
            demo_file_processing(files, delay, session.use_color).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = ProgressCli::parse();

    // Create session with options
    let mut session = ProgressSession::new(cli.verbose, !cli.no_color);

    // Create starbase app
    let app = App::default();

    // Run the application with session lifecycle
    app.run_with_session(&mut session, |session| async move {
        // Execute the progress command
        execute_command(cli.command, &session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
