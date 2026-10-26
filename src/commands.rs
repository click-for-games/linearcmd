use crate::config;
use crate::error::LinearError;
use crate::release;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;

#[derive(Parser)]
#[command(name = "linearcmd")]
#[command(author = "linearcmd")]
#[command(version = "0.1.0")]
#[command(about = "Ultimate developer CLI for clean project management, error handling, and automated releases", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check and display configuration
    Config,
    /// Run build (placeholder for smart build detection)
    Build,
    /// Create and publish a release automatically
    Release,
    /// Get contextual code help and diagnostics
    Help,
    /// Initialize config files
    Init,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Config => {
            println!("{} Loading configurations...", "[LINEARCMD]".green().bold());
            match config::load_linear_config() {
                Ok(cfg) => {
                    println!("{} .linear found", "[OK]".green());
                    if let Some(p) = cfg.project {
                        println!("  Project: {}", p);
                    }
                    if let Some(ref ignore) = cfg.ignore {
                        if !ignore.is_empty() {
                            println!("  Ignored patterns: {}", ignore.join(", "));
                        }
                    }
                }
                Err(_) => {
                    println!("{} .linear not found or invalid", "[WARN]".yellow());
                }
            }
            match config::load_linearcmd_config() {
                Ok(cfg) => {
                    println!("{} .linearcmd found", "[OK]".green());
                    println!("  Build dir: {}", cfg.build_dir);
                    println!("  Artifacts: {}", cfg.artifacts.join(", "));
                }
                Err(e) => {
                    println!("{} .linearcmd: {}", "[ERR]".red(), e);
                }
            }
        }
        Commands::Build => {
            println!("{} Smart build detection...", "[LINEARCMD]".green().bold());
            println!("{} Scanning for build scripts...", "[INFO]".cyan());
            let build_files = vec!["package.json", "Makefile", "CMakeLists.txt", "Cargo.toml", "build.gradle", "pom.xml"];
            for f in build_files {
                if std::path::Path::new(f).exists() {
                    println!("{} Found {}", "[OK]".green(), f);
                }
            }
            println!("{} Use your native build tool, then run 'linearcmd release'", "[INFO]".cyan());
        }
        Commands::Release => {
            if let Err(e) = release::run_release() {
                let err = LinearError::Release(e.to_string());
                err.print();
                std::process::exit(1);
            }
        }
        Commands::Help => {
            println!("{} Contextual Code Help", "[LINEARCMD]".green().bold());
            println!();
            println!("{} Quick Diagnostics:", "Tips:".cyan().bold());
            println!("  - Config issues: Run 'linearcmd config' to check");
            println!("  - Missing artifacts: Ensure build outputs match .linearcmd");
            println!("  - Release failed: Check GITHUB_TOKEN and GITHUB_REPO env vars");
            println!();
            println!("{} Tips:", "Usage:".cyan().bold());
            println!("  - Create .linear for ignore patterns and project settings");
            println!("  - Create .linearcmd with build_dir and artifacts list");
            println!("  - Run 'linearcmd release' for one-command publishing");
        }
        Commands::Init => {
            println!("{} Initializing config files...", "[LINEARCMD]".green().bold());
            if !std::path::Path::new(".linear").exists() {
                let content = "# Files and folders to skip\nignore:\n  node_modules/\n  build/\n  dist/\n  *.log\n  .env\n\n# Optional project settings\n# project: MyProject\n";
                fs::write(".linear", content)?;
                println!("{} Created .linear", "[OK]".green());
            } else {
                println!("{} .linear already exists", "[WARN]".yellow());
            }
            if !std::path::Path::new(".linearcmd").exists() {
                let content = "# Tell linearcmd where your compiled assets live\nbuild_dir: ./dist\n\n# Specify output binaries/archives to attach to the release\nartifacts:\n  - app.exe\n";
                fs::write(".linearcmd", content)?;
                println!("{} Created .linearcmd", "[OK]".green());
            } else {
                println!("{} .linearcmd already exists", "[WARN]".yellow());
            }
            println!("{} Edit the files to match your project", "[INFO]".cyan());
        }
    }
    
    Ok(())
}
