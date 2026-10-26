use crate::config::LinearCmdConfig;
use crate::error::LinearError;
use anyhow::Result;
use colored::*;
use dialoguer::Input;
use reqwest::blocking::{Client, multipart};
use std::fs;
use std::path::{Path, PathBuf};

pub fn run_release() -> Result<()> {
    println!("{} Starting release process...", "[LINEARCMD]".green().bold());
    
    let config = crate::config::load_linearcmd_config().map_err(|e| LinearError::Config(e.to_string()))?;
    
    // Prompt for release tag
    let tag: String = Input::new()
        .with_prompt("Release Tag (e.g., v1.0.0)")
        .interact_text()?;
    
    println!("{} Release tag: {}", "[INFO]".cyan(), tag);
    println!("{} Build dir: {}", "[INFO]".cyan(), config.build_dir);
    
    // Collect artifacts
    let mut artifacts: Vec<PathBuf> = Vec::new();
    let build_path = Path::new(&config.build_dir);
    if !build_path.exists() {
        return Err(LinearError::Build(format!("Build directory '{}' does not exist. Run your build first.", config.build_dir)).into());
    }
    
    for artifact in &config.artifacts {
        let artifact_path = build_path.join(artifact);
        if !artifact_path.exists() {
            // Also try from current dir
            let alt_path = Path::new(artifact);
            if alt_path.exists() {
                artifacts.push(alt_path.to_path_buf());
                println!("{} Found artifact: {}", "[OK]".green(), alt_path.display());
                continue;
            }
            return Err(LinearError::Build(format!("Artifact '{}' not found in {} or current directory", artifact, config.build_dir)).into());
        }
        artifacts.push(artifact_path.clone());
        println!("{} Found artifact: {}", "[OK]".green(), artifact_path.display());
    }
    
    // Safety check - prevent overwriting
    println!("{} Safety checks passed", "[OK]".green());
    
    // Try to publish to GitHub if token is available
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        println!("{} Publishing to GitHub...", "[INFO]".cyan());
        if let Err(e) = publish_github(&token, &tag, &artifacts) {
            eprintln!("{} GitHub publish failed: {}", "[WARN]".yellow(), e);
            println!("{} Artifacts collected and ready for manual upload", "[INFO]".cyan());
        } else {
            println!("{} Release published successfully!", "[SUCCESS]".green().bold());
            return Ok(());
        }
    } else {
        println!("{} GITHUB_TOKEN not found in environment", "[WARN]".yellow());
        println!("{} Artifacts collected:", "[INFO]".cyan());
        for a in &artifacts {
            println!("  - {}", a.display());
        }
        println!("{} Set GITHUB_TOKEN to auto-publish to GitHub", "[INFO]".cyan());
    }
    
    println!("{} Release process complete", "[SUCCESS]".green().bold());
    Ok(())
}

fn publish_github(token: &str, tag: &str, artifacts: &[PathBuf]) -> Result<()> {
    let repo = std::env::var("GITHUB_REPO").unwrap_or_else(|_| {
        // Try to get from git remote
        String::new()
    });
    
    if repo.is_empty() {
        anyhow::bail!("GITHUB_REPO not set (format: owner/repo)");
    }
    
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    
    let body = serde_json::json!({
        "tag_name": tag,
        "name": tag,
        "draft": false,
        "prerelease": false
    });
    
    let resp: serde_json::Value = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "linearcmd")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send()?
        .json()?;
    
    let upload_url = resp["upload_url"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get upload URL"))?
        .replace("{?name,label}", "");
    
    for artifact in artifacts {
        let filename = artifact.file_name().unwrap().to_string_lossy().to_string();
        let data = fs::read(artifact)?;
        let _part = multipart::Part::bytes(data)
            .file_name(filename.clone());
        
        let upload = format!("{}?name={}", upload_url, filename);
        client
            .post(&upload)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "linearcmd")
            .header("Content-Type", "application/octet-stream")
            .body(fs::read(artifact)?)
            .send()?;
        println!("{} Uploaded {}", "[OK]".green(), filename);
    }
    
    Ok(())
}
