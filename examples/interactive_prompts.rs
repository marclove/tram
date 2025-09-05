//! Interactive prompts example
//!
//! This example demonstrates how to handle user interaction in CLI applications,
//! including:
//! - Text input prompts
//! - Confirmation dialogs
//! - Selection menus
//! - Multi-select options
//! - Password input
//! - Validation and error handling

use async_trait::async_trait;
use clap::Parser;
use dialoguer::{
    Confirm, Input, MultiSelect, Password, Select,
    console::Term,
    theme::{ColorfulTheme, SimpleTheme},
};
use miette::Result;
use starbase::{App, AppSession};
use std::collections::HashMap;
use tracing::info;

/// Interactive prompts CLI example
#[derive(Parser, Debug)]
#[command(name = "interactive-example")]
#[command(about = "Demonstrates interactive user prompts and dialogs")]
struct InteractiveCli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Skip interactive prompts (use defaults)
    #[arg(short, long)]
    yes: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: InteractiveCommand,
}

/// Available interactive command demonstrations
#[derive(Parser, Debug)]
enum InteractiveCommand {
    /// Basic input prompts
    BasicInput,
    /// Confirmation dialogs
    Confirmations,
    /// Selection menus
    Selections,
    /// Multi-select options
    MultiSelect,
    /// Password input
    Password,
    /// Project setup wizard
    Wizard,
    /// Form-style input collection
    Form,
    /// Validation examples
    Validation,
}

/// Session for interactive examples
#[derive(Debug, Clone)]
struct InteractiveSession {
    verbose: bool,
    use_color: bool,
    auto_confirm: bool,
}

impl InteractiveSession {
    fn new(verbose: bool, use_color: bool, auto_confirm: bool) -> Self {
        Self {
            verbose,
            use_color,
            auto_confirm,
        }
    }
}

#[async_trait]
impl AppSession for InteractiveSession {
    async fn startup(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Starting interactive prompts example");
        }
        Ok(None)
    }

    async fn analyze(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Interactive prompts ready");
        }
        Ok(None)
    }

    async fn shutdown(&mut self) -> Result<Option<u8>, miette::Error> {
        if self.verbose {
            info!("Interactive prompts example complete");
        }
        Ok(None)
    }
}

/// Demonstrate basic text input
fn demo_basic_input(use_color: bool) -> Result<()> {
    println!("=== Basic Input Prompts ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // Simple text input
    let name: String = Input::with_theme(theme)
        .with_prompt("What's your name?")
        .default("Anonymous".to_string())
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    println!("Hello, {}!\n", name);

    // Number input with validation
    let age_str: String = Input::with_theme(theme)
        .with_prompt("How old are you?")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<u32>() {
                Ok(age) if age > 0 && age < 150 => Ok(()),
                Ok(_) => Err("Please enter a realistic age (1-149)"),
                Err(_) => Err("Please enter a valid number"),
            }
        })
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    let age = age_str.parse::<u32>().unwrap(); // Safe because validation passed

    println!("You are {} years old.\n", age);

    // Input with default value
    let city: String = Input::with_theme(theme)
        .with_prompt("What city are you from?")
        .default("Unknown".to_string())
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    println!("You're from {}.\n", city);

    Ok(())
}

/// Demonstrate confirmation dialogs
fn demo_confirmations(use_color: bool) -> Result<()> {
    println!("=== Confirmation Dialogs ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // Simple yes/no confirmation
    let proceed = Confirm::with_theme(theme)
        .with_prompt("Do you want to proceed?")
        .default(true)
        .interact()
        .map_err(|e| miette::miette!("Confirmation error: {}", e))?;

    if proceed {
        println!("Proceeding...\n");
    } else {
        println!("Operation cancelled.\n");
        return Ok(());
    }

    // Dangerous operation confirmation
    let delete = Confirm::with_theme(theme)
        .with_prompt("This will delete all files. Are you sure?")
        .default(false)
        .interact()
        .map_err(|e| miette::miette!("Confirmation error: {}", e))?;

    if delete {
        println!("⚠️ Files would be deleted (simulated).\n");
    } else {
        println!("Delete operation cancelled.\n");
    }

    Ok(())
}

/// Demonstrate selection menus
fn demo_selections(use_color: bool) -> Result<()> {
    println!("=== Selection Menus ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // Simple selection
    let languages = vec!["Rust", "TypeScript", "Python", "Go", "Java"];

    let selection = Select::with_theme(theme)
        .with_prompt("What's your favorite programming language?")
        .default(0)
        .items(&languages)
        .interact()
        .map_err(|e| miette::miette!("Selection error: {}", e))?;

    println!("You selected: {}\n", languages[selection]);

    // Selection with descriptions
    let tools = [
        ("Git", "Version control system"),
        ("Docker", "Containerization platform"),
        ("Kubernetes", "Container orchestration"),
        ("Terraform", "Infrastructure as code"),
    ];

    let formatted_options: Vec<String> = tools
        .iter()
        .map(|(name, desc)| format!("{} - {}", name, desc))
        .collect();

    let tool_selection = Select::with_theme(theme)
        .with_prompt("Which tool do you use most?")
        .items(&formatted_options)
        .interact()
        .map_err(|e| miette::miette!("Selection error: {}", e))?;

    println!("You selected: {}\n", tools[tool_selection].0);

    Ok(())
}

/// Demonstrate multi-select options
fn demo_multi_select(use_color: bool) -> Result<()> {
    println!("=== Multi-Select Options ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // Multiple selections
    let features = vec![
        "Authentication",
        "Database integration",
        "REST API",
        "WebSocket support",
        "File upload",
        "Email notifications",
        "Logging",
        "Metrics",
    ];

    let selections = MultiSelect::with_theme(theme)
        .with_prompt("Which features do you want to enable? (use space to select)")
        .items(&features)
        .interact()
        .map_err(|e| miette::miette!("Multi-select error: {}", e))?;

    if selections.is_empty() {
        println!("No features selected.\n");
    } else {
        println!("Selected features:");
        for &selection in &selections {
            println!("  ✓ {}", features[selection]);
        }
        println!();
    }

    Ok(())
}

/// Demonstrate password input
fn demo_password(use_color: bool) -> Result<()> {
    println!("=== Password Input ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // Simple password input
    let password = Password::with_theme(theme)
        .with_prompt("Enter password")
        .interact()
        .map_err(|e| miette::miette!("Password input error: {}", e))?;

    println!("Password entered (length: {})\n", password.len());

    // Password with confirmation
    let new_password = Password::with_theme(theme)
        .with_prompt("Enter new password")
        .with_confirmation("Confirm password", "Passwords don't match")
        .interact()
        .map_err(|e| miette::miette!("Password confirmation error: {}", e))?;

    println!("New password set (length: {})\n", new_password.len());

    Ok(())
}

/// Demonstrate a project setup wizard
fn demo_wizard(use_color: bool) -> Result<()> {
    println!("=== Project Setup Wizard ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // Collect project information
    let project_name: String = Input::with_theme(theme)
        .with_prompt("Project name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Project name cannot be empty")
            } else if input.contains(' ') {
                Err("Project name cannot contain spaces")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    let description: String = Input::with_theme(theme)
        .with_prompt("Project description")
        .default("A new project".to_string())
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    let project_types = vec!["Web Application", "CLI Tool", "Library", "API Service"];
    let project_type = Select::with_theme(theme)
        .with_prompt("Project type")
        .items(&project_types)
        .interact()
        .map_err(|e| miette::miette!("Selection error: {}", e))?;

    let features = vec![
        "Docker support",
        "GitHub Actions CI/CD",
        "Testing framework",
        "Documentation",
        "Linting configuration",
    ];

    let selected_features = MultiSelect::with_theme(theme)
        .with_prompt("Additional features (space to select, enter to continue)")
        .items(&features)
        .interact()
        .map_err(|e| miette::miette!("Multi-select error: {}", e))?;

    let use_git = Confirm::with_theme(theme)
        .with_prompt("Initialize Git repository?")
        .default(true)
        .interact()
        .map_err(|e| miette::miette!("Confirmation error: {}", e))?;

    // Display summary
    println!("\n=== Project Summary ===");
    println!("Name: {}", project_name);
    println!("Description: {}", description);
    println!("Type: {}", project_types[project_type]);

    if !selected_features.is_empty() {
        println!("Features:");
        for &feature_idx in &selected_features {
            println!("  ✓ {}", features[feature_idx]);
        }
    }

    println!("Git: {}", if use_git { "Yes" } else { "No" });

    let create = Confirm::with_theme(theme)
        .with_prompt("\nCreate project with these settings?")
        .default(true)
        .interact()
        .map_err(|e| miette::miette!("Confirmation error: {}", e))?;

    if create {
        println!(
            "\n✓ Project '{}' would be created (simulated)",
            project_name
        );
    } else {
        println!("\nProject creation cancelled.");
    }

    println!();
    Ok(())
}

/// Demonstrate form-style input
fn demo_form(use_color: bool) -> Result<()> {
    println!("=== Form-Style Input ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    let mut user_data = HashMap::new();

    // Personal information
    println!("📝 Personal Information:");

    let first_name: String = Input::with_theme(theme)
        .with_prompt("  First name")
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;
    user_data.insert("first_name", first_name);

    let last_name: String = Input::with_theme(theme)
        .with_prompt("  Last name")
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;
    user_data.insert("last_name", last_name);

    let email: String = Input::with_theme(theme)
        .with_prompt("  Email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') && input.contains('.') {
                Ok(())
            } else {
                Err("Please enter a valid email address")
            }
        })
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;
    user_data.insert("email", email);

    // Preferences
    println!("\n⚙️ Preferences:");

    let notification_types = vec!["Email", "SMS", "Push", "None"];
    let notification = Select::with_theme(theme)
        .with_prompt("  Preferred notifications")
        .items(&notification_types)
        .default(0)
        .interact()
        .map_err(|e| miette::miette!("Selection error: {}", e))?;
    user_data.insert(
        "notifications",
        notification_types[notification].to_string(),
    );

    let newsletter = Confirm::with_theme(theme)
        .with_prompt("  Subscribe to newsletter?")
        .default(false)
        .interact()
        .map_err(|e| miette::miette!("Confirmation error: {}", e))?;
    user_data.insert(
        "newsletter",
        if newsletter { "Yes" } else { "No" }.to_string(),
    );

    // Display collected data
    println!("\n=== Collected Information ===");
    for (key, value) in &user_data {
        println!("{}: {}", key.replace('_', " "), value);
    }

    println!();
    Ok(())
}

/// Demonstrate input validation
fn demo_validation(use_color: bool) -> Result<()> {
    println!("=== Input Validation ===\n");

    let theme = if use_color {
        &ColorfulTheme::default() as &dyn dialoguer::theme::Theme
    } else {
        &SimpleTheme
    };

    // URL validation
    let url: String = Input::with_theme(theme)
        .with_prompt("Enter a valid URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.starts_with("http://") || input.starts_with("https://") {
                Ok(())
            } else {
                Err("URL must start with http:// or https://")
            }
        })
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    println!("Valid URL: {}\n", url);

    // Port number validation
    let port_str: String = Input::with_theme(theme)
        .with_prompt("Enter a port number (1024-65535)")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<u16>() {
                Ok(port) if port >= 1024 => Ok(()),
                Ok(_) => Err("Port must be 1024 or higher"),
                Err(_) => Err("Please enter a valid port number"),
            }
        })
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    let port = port_str.parse::<u16>().unwrap(); // Safe because validation passed

    println!("Valid port: {}\n", port);

    // File path validation
    let path: String = Input::with_theme(theme)
        .with_prompt("Enter a file path")
        .validate_with(|input: &String| -> Result<(), &str> {
            let path = std::path::Path::new(input);
            if path.exists() {
                Ok(())
            } else {
                Err("File or directory does not exist")
            }
        })
        .interact_text()
        .map_err(|e| miette::miette!("Input error: {}", e))?;

    println!("Valid path: {}\n", path);

    Ok(())
}

/// Execute the parsed interactive command
async fn execute_command(command: InteractiveCommand, session: &InteractiveSession) -> Result<()> {
    if session.auto_confirm {
        println!("Note: Running in auto-confirm mode (--yes flag)\n");
    }

    // Check if we're running in a proper terminal
    if !Term::stdout().is_term() {
        return Err(miette::miette!(
            "Interactive prompts require a terminal. Please run this command in a terminal."
        ));
    }

    match command {
        InteractiveCommand::BasicInput => {
            demo_basic_input(session.use_color)?;
        }

        InteractiveCommand::Confirmations => {
            demo_confirmations(session.use_color)?;
        }

        InteractiveCommand::Selections => {
            demo_selections(session.use_color)?;
        }

        InteractiveCommand::MultiSelect => {
            demo_multi_select(session.use_color)?;
        }

        InteractiveCommand::Password => {
            demo_password(session.use_color)?;
        }

        InteractiveCommand::Wizard => {
            demo_wizard(session.use_color)?;
        }

        InteractiveCommand::Form => {
            demo_form(session.use_color)?;
        }

        InteractiveCommand::Validation => {
            demo_validation(session.use_color)?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = InteractiveCli::parse();

    // Create session with options
    let mut session = InteractiveSession::new(cli.verbose, !cli.no_color, cli.yes);

    // Create starbase app
    let app = App::default();

    // Run the application with session lifecycle
    app.run_with_session(&mut session, |session| async move {
        // Execute the interactive command
        execute_command(cli.command, &session).await?;
        Ok(Some(0))
    })
    .await
    .map_err(|e| miette::miette!("Application error: {}", e))?;

    Ok(())
}
