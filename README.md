# linearcmd

The ultimate open-source developer CLI designed for clean project management, supercharged error handling, smart code help, and effortless automated releases.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Commands](#commands)
- [Release Workflow](#release-workflow)
- [Environment Variables](#environment-variables)
- [Error Handling](#error-handling)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)

## Overview

linearcmd replaces complex, bloated toolchains with simple configuration files and automated build/release workflows. Built for high speed, clarity, and rock-solid safety.

## Features

- **Instant Releases** - Zero manual uploading or manual version tagging. One command publishes everything.
- **Smart Error Handling** - Friendly, human-readable error diagnostics with actionable fix suggestions.
- **Contextual Code Help** - Built-in code diagnostics and terminal assistance to fix issues fast.
- **Safety & Ergonomics** - Automatic safety checks to prevent overwriting assets or breaking builds.
- **Ultra-Simple Config** - Minimal, key-value based configuration files that are instantly readable.
- **Auto-Detection** - Smart build script detection for common project types.

## Installation

### From Source

`ash
git clone https://github.com/click-for-games/linearcmd.git
cd linearcmd
cargo build --release
`

Add 	arget/release to your PATH or copy the binary to a location in your PATH.

## Quick Start

`ash
# Initialize configuration files in your project
linearcmd init

# Edit .linear and .linearcmd to match your project
# Check your configuration
linearcmd config

# Build your project with your native tool
# Then release
linearcmd release
`

## Configuration

### .linear (Ignore & Project Instructions)

Replaces messy ignore lists with clean, simple rules for files to skip and basic project directives.

`yaml
# Files and folders to skip
ignore:
  node_modules/
  build/
  dist/
  *.log
  .env

# Optional project settings
project: MyProject
`

### .linearcmd (Automated Build & Release Targets)

Tells linearcmd exactly where your compiled output files are located for automatic release creation.

`yaml
# Tell linearcmd where your compiled assets live
build_dir: ./dist

# Specify output binaries/archives to attach to the release
artifacts:
  - app.exe
  - app.zip
  - app.deb
`

## Commands

| Command | Description |
|---------|-------------|
| linearcmd init | Initialize .linear and .linearcmd config files with templates |
| linearcmd config | Check and display current configuration |
| linearcmd build | Smart build detection - scans for common build scripts |
| linearcmd release | Create and publish a release automatically |
| linearcmd help | Get contextual code help and diagnostics |

## Release Workflow

When you run linearcmd release:

1. **Load Config** - Reads .linearcmd for build directory and artifacts
2. **Prompt for Tag** - Asks for release tag (e.g., 1.0.0, 1.2.3)
3. **Collect Artifacts** - Finds all specified artifacts in uild_dir or current directory
4. **Safety Checks** - Validates all files exist before proceeding
5. **Publish** - If GITHUB_TOKEN and GITHUB_REPO are set, creates GitHub release and uploads assets. Otherwise collects artifacts for manual upload.

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| GITHUB_TOKEN | Optional | GitHub personal access token with epo scope for automatic publishing |
| GITHUB_REPO | Optional | Target repository in format owner/repo |

## Error Handling

linearcmd provides clear, actionable error messages. Every error states:
- **What failed** - Clear description of the issue
- **How to fix** - Specific steps to resolve
- **Details** - Additional context for debugging

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or higher

### Build

`ash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
`

The compiled binary will be located at:
- Debug: 	arget/debug/linearcmd
- Release: 	arget/release/linearcmd

## Contributing

Contributions are welcome! When modifying linearcmd:

- Keep configurations ultra-simple - syntax in .linear and .linearcmd should remain minimal, key-value based, and instantly readable
- Clear diagnostics - Every error output must clearly state what failed and how to fix it
- Automate every step - Maintain a single-command release workflow

## License

Open source - see repository for details.
