mod commands;
mod config;
mod error;
mod release;

use anyhow::Result;
use colored::*;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("{} {}", "[FATAL]".red().bold(), e);
        std::process::exit(1);
    }
}
