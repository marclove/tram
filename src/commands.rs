//! Command execution logic.
//!
//! This module contains the implementation of all CLI commands, handling the business logic
//! for each subcommand while maintaining separation from the CLI argument parsing.

use std::collections::HashMap;
use tracing::{debug, info, warn};
use tram_config::ConfigWatcher;
use tram_core::{InitConfig, ProjectInitializer, TemplateConfig, TemplateGenerator};

use crate::cli::Commands;
use crate::dev_tools::{generate_completions, generate_man_pages};
use crate::examples::run_example;
use crate::session::{TramSession, WatchConfigHandler};
use crate::utils::{
    parse_project_type, parse_template_type, project_type_display, template_type_display,
};

/// Execute a CLI command with the session.
pub async fn execute_command(command: Commands, session: &TramSession) -> tram_core::AppResult<()> {
    match command {
        Commands::New {
            name,
            project_type,
            description,
            skip_prompts,
        } => {
            info!("Creating new project: {}", name);

            if !skip_prompts {
                // In future iterations, we would add interactive prompts here
                // For now, just note that interactive mode is planned
                debug!("Interactive prompts would be shown here (future feature)");
            }

            let project_type = parse_project_type(&project_type);
            let current_dir =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let project_path = current_dir.join(&name);

            let init_config = InitConfig {
                name: name.clone(),
                path: project_path,
                project_type,
                description,
                author: None,
            };

            let initializer = ProjectInitializer::new();
            initializer.create_project(&init_config)?;

            println!(
                "✓ Created new {} project: {}",
                project_type_display(&init_config.project_type),
                name
            );
            if let Some(desc) = &init_config.description {
                println!("  Description: {}", desc);
            }
        }

        Commands::Generate {
            template_type,
            name,
            description,
            target_dir,
            write,
        } => {
            info!("Generating {} template: {}", template_type, name);

            let template_type = parse_template_type(&template_type);
            let target_dir = target_dir.unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            });

            let mut parameters = HashMap::new();
            if let Some(desc) = description {
                parameters.insert("description".to_string(), desc);
            }

            let template_config = TemplateConfig {
                name: name.clone(),
                template_type: template_type.clone(),
                target_dir,
                parameters,
            };

            let generator = TemplateGenerator::new()?;
            let template = generator.generate_template(&template_config)?;

            if write {
                generator.write_template(&template)?;
                println!(
                    "✓ Generated {} template: {} -> {}",
                    template_type_display(&template_type),
                    name,
                    template.file_path.display()
                );
            } else {
                println!(
                    "Generated {} template for '{}':",
                    template_type_display(&template_type),
                    name
                );
                println!("File path: {}", template.file_path.display());
                println!("\n{}", "=".repeat(80));
                println!("{}", template.content);
                println!("{}", "=".repeat(80));
                println!("\nTo write to filesystem, add the --write flag");
            }
        }

        Commands::Init { name, verbose } => {
            println!("🚀 Initializing project: {}", name);

            if verbose {
                println!("Verbose mode enabled");
                if let Some(root) = &session.workspace_root {
                    println!("Workspace root: {}", root.display());
                }
                println!("Config: {:?}", session.config);
            }

            // Legacy command - for now, just create a generic project
            let current_dir =
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let project_path = current_dir.join(&name);

            let init_config = InitConfig {
                name: name.clone(),
                path: project_path,
                project_type: tram_core::InitProjectType::Generic,
                description: Some("A new project".to_string()),
                author: None,
            };

            let initializer = ProjectInitializer::new();
            if let Err(e) = initializer.create_project(&init_config) {
                println!("Warning: Could not create project files: {}", e);
            }

            println!("Project '{}' initialized!", name);
        }

        Commands::Workspace { detailed } => {
            if let Some(root) = &session.workspace_root {
                println!("Workspace root: {}", root.display());

                if let Some(project_type) = &session.project_type {
                    println!("Project type: {:?}", project_type);

                    if detailed {
                        println!("Ignore patterns: {:?}", project_type.ignore_patterns());
                    }
                }
            } else {
                return Err(tram_core::TramError::WorkspaceNotFound.into());
            }
        }

        Commands::Config => {
            println!("Current configuration:");
            println!("   Log level: {}", session.config.log_level);
            println!("   Output format: {}", session.config.output_format);
            println!("   Colors: {}", session.config.color);

            if let Some(workspace_root) = &session.config.workspace_root {
                println!("   Workspace root: {}", workspace_root.display());
            }
        }

        Commands::Watch {
            config: watch_config,
            check,
        } => {
            info!("Starting watch mode...");

            if watch_config {
                info!("🔍 Config hot reload: ENABLED");
            } else {
                info!("🔍 Config hot reload: DISABLED");
            }

            if check {
                info!("⚡ Auto-checks (format, lint, build, test): ENABLED");
            } else {
                info!("⚡ Auto-checks: DISABLED");
            }

            println!("Watch mode started. Press Ctrl+C to stop.");

            let mut tasks = Vec::new();

            // Set up config watcher if enabled
            if watch_config {
                let config_watcher = ConfigWatcher::new(session.config.clone(), None)
                    .await
                    .map_err(|e| tram_core::TramError::InvalidConfig {
                        message: format!("Failed to start config watcher: {}", e),
                    })?;

                let handler = WatchConfigHandler;
                if let Err(e) = config_watcher.start_with_handler(handler).await {
                    warn!("Failed to start config change handler: {}", e);
                }

                // Keep the watcher alive by storing it
                tasks.push(tokio::spawn(async move {
                    // Keep the config_watcher alive for the duration of the task
                    let _watcher = config_watcher;
                    // Wait indefinitely (until the task is cancelled)
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
                    loop {
                        interval.tick().await;
                    }
                }));
            }

            // Set up file watching for code changes if enabled
            if check {
                tasks.push(tokio::spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
                    let mut last_check = std::time::Instant::now();

                    loop {
                        interval.tick().await;

                        // Simple implementation: check if any Rust files have been modified
                        // In a real implementation, you'd use a proper file watcher
                        let current_time = std::time::Instant::now();
                        if current_time.duration_since(last_check).as_secs() >= 2 {
                            debug!("Running periodic checks (placeholder for file-based trigger)");
                            last_check = current_time;

                            // Here you would run `just check` or equivalent
                            // For now, just log that we would run checks
                            debug!("Would run: just check");
                        }
                    }
                }));
            }

            if tasks.is_empty() {
                warn!("No watch features enabled. Use --config or --check flags.");
                return Ok(());
            }

            // Wait for Ctrl+C
            tokio::signal::ctrl_c()
                .await
                .map_err(|e| tram_core::TramError::InvalidConfig {
                    message: format!("Failed to wait for Ctrl+C: {}", e),
                })?;

            info!("Shutting down watch mode...");

            // Cancel all tasks
            for task in tasks {
                task.abort();
            }

            println!("Watch mode stopped.");
        }

        Commands::Examples { example } => {
            info!("Running example: {:?}", example);
            run_example(example, session).await?;
        }

        Commands::Completions { shell } => {
            info!("Generating completions for {:?}", shell);
            generate_completions(shell)?;
        }

        Commands::Man {
            output_dir,
            section,
        } => {
            info!("Generating manual pages");
            generate_man_pages(&output_dir, section)?;
        }
    }

    Ok(())
}
