use colored::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinearError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("Build error: {0}")]
    Build(String),
    #[error("Release error: {0}")]
    Release(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl LinearError {
    pub fn print(&self) {
        eprintln!("{} {}", "[ERROR]".red().bold(), self);
        match self {
            LinearError::Config(msg) => {
                eprintln!("{} Check your config files (.linear or .linearcmd) and try again.", "[FIX]".yellow());
                eprintln!("{} {}", "[DETAIL]".cyan(), msg);
            }
            LinearError::Build(msg) => {
                eprintln!("{} Ensure your build completed successfully and outputs are in place.", "[FIX]".yellow());
                eprintln!("{} {}", "[DETAIL]".cyan(), msg);
            }
            LinearError::Release(msg) => {
                eprintln!("{} Verify GitHub credentials and artifact paths in .linearcmd.", "[FIX]".yellow());
                eprintln!("{} {}", "[DETAIL]".cyan(), msg);
            }
            LinearError::Io(msg) => {
                eprintln!("{} Check file permissions and paths.", "[FIX]".yellow());
                eprintln!("{} {}", "[DETAIL]".cyan(), msg);
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, LinearError>;
