# linearcmd

The ultimate open-source developer CLI designed for clean project management, supercharged error handling, smart code help, and effortless automated releases.

## Features

- **Instant Releases**: Zero manual uploading or manual version tagging
- **Smart Error Handling**: Friendly, human-readable error diagnostics with fix suggestions
- **Contextual Code Help**: Built-in code diagnostics and terminal assistance
- **Safety & Ergonomics**: Automatic safety checks to prevent overwriting assets or breaking builds
- **Ultra-Simple Config**: Clean, key-value configuration files

## Configuration

### .linear (Ignore & Project Instructions)

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

`yaml
# Tell linearcmd where your compiled assets live
build_dir: ./dist

# Specify output binaries/archives to attach to the release
artifacts:
  - app.exe
  - app.zip
  - app.deb
`

## Usage

`ash
# Initialize config files
linearcmd init

# Check configurations
linearcmd config

# Smart build detection
linearcmd build

# Get contextual help
linearcmd help

# Create and publish a release
linearcmd release
`

When running linearcmd release:
1. Prompts for release tag (e.g., v1.0.0)
2. Collects artifacts from build_dir
3. Publishes to GitHub if GITHUB_TOKEN and GITHUB_REPO are set
4. Otherwise prepares artifacts for manual upload

## Environment Variables

- GITHUB_TOKEN - GitHub personal access token for automatic publishing
- GITHUB_REPO - Repository in format owner/repo

## Building from Source

`ash
cargo build --release
`

The binary will be at 	arget/release/linearcmd.
