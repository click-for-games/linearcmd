use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct LinearConfig {
    pub ignore: Option<Vec<String>>,
    pub project: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct LinearCmdConfig {
    pub build_dir: String,
    pub artifacts: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

pub fn load_linear_config() -> Result<LinearConfig> {
    let path = Path::new(".linear");
    if !path.exists() {
        return Ok(LinearConfig {
            ignore: None,
            project: None,
            extra: HashMap::new(),
        });
    }
    let content = fs::read_to_string(path).context("Failed to read .linear config")?;
    parse_linear(&content)
}

pub fn load_linearcmd_config() -> Result<LinearCmdConfig> {
    let path = Path::new(".linearcmd");
    if !path.exists() {
        anyhow::bail!("Missing .linearcmd config file. Create it in project root with build_dir and artifacts.");
    }
    let content = fs::read_to_string(path).context("Failed to read .linearcmd config")?;
    parse_linearcmd(&content)
}

fn parse_linear(content: &str) -> Result<LinearConfig> {
    let mut ignore: Option<Vec<String>> = None;
    let mut project: Option<String> = None;
    let mut current_key: Option<String> = None;
    let mut current_list: Vec<String> = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.ends_with(':') && !trimmed.starts_with('-') {
            // Flush previous key if it was ignore
            if let Some(ref key) = current_key {
                if key == "ignore" {
                    ignore = Some(current_list.clone());
                    current_list.clear();
                }
            }
            current_key = Some(trimmed.trim_end_matches(':').trim().to_string());
            continue;
        }
        if trimmed.starts_with('-') {
            let item = trimmed.trim_start_matches('-').trim().to_string();
            if let Some(ref key) = current_key {
                if key == "ignore" {
                    current_list.push(item.clone());
                    continue;
                }
            }
            // If not in a specific list context, might be ignore items
            if current_key.as_deref() == Some("ignore") {
                current_list.push(item.clone());
                continue;
            }
            current_list.push(item);
            continue;
        }
        if trimmed.contains(':') {
            if let Some(ref key) = current_key {
                if key == "ignore" {
                    ignore = Some(current_list.clone());
                    current_list.clear();
                }
            }
            let mut parts = trimmed.splitn(2, ':');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim().to_string();
            if key == "project" {
                project = Some(value);
                current_key = None;
                continue;
            }
            if key == "ignore" {
                current_key = Some("ignore".to_string());
                continue;
            }
            current_key = None;
            continue;
        }
        // List item without dash
        if let Some(ref key) = current_key {
            if key == "ignore" {
                current_list.push(trimmed.to_string());
                continue;
            }
        }
    }
    if let Some(ref key) = current_key {
        if key == "ignore" {
            ignore = Some(current_list);
        }
    }
    
    Ok(LinearConfig {
        ignore,
        project,
        extra: HashMap::new(),
    })
}

fn parse_linearcmd(content: &str) -> Result<LinearCmdConfig> {
    let mut build_dir: Option<String> = None;
    let mut artifacts: Vec<String> = Vec::new();
    let mut in_artifacts = false;
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed == "artifacts:" {
            in_artifacts = true;
            continue;
        }
        if trimmed.ends_with(':') && trimmed != "artifacts:" {
            in_artifacts = false;
            let mut parts = trimmed.splitn(2, ':');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim();
            if key == "build_dir" {
                build_dir = Some(value.to_string());
            }
            continue;
        }
        if trimmed.contains(':') && !trimmed.starts_with('-') {
            in_artifacts = false;
            let mut parts = trimmed.splitn(2, ':');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim().to_string();
            if key == "build_dir" {
                build_dir = Some(value);
                continue;
            }
            continue;
        }
        if trimmed.starts_with('-') {
            let item = trimmed.trim_start_matches('-').trim().to_string();
            if in_artifacts {
                artifacts.push(item);
            } else {
                artifacts.push(item);
            }
            continue;
        }
    }
    
    if build_dir.is_none() {
        anyhow::bail!("build_dir not found in .linearcmd");
    }
    if artifacts.is_empty() {
        anyhow::bail!("artifacts not found in .linearcmd");
    }
    
    Ok(LinearCmdConfig {
        build_dir: build_dir.unwrap(),
        artifacts,
        extra: HashMap::new(),
    })
}
