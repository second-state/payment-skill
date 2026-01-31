# x402 Skill Installation Guide

This guide is intended for AI agents to programmatically install the x402 payment skill.

## Prerequisites

- `curl` or `wget` for downloading files
- `unzip` for extracting archives
- Write access to the skill installation directory

## Repository Information

- **Repository**: `https://github.com/anthropics/x402-skill` (adjust as needed)
- **Releases URL**: `https://github.com/anthropics/x402-skill/releases/latest`

## Available Platforms

| Platform | Architecture | Zip Filename |
|----------|--------------|--------------|
| Linux | x86_64 | `x402-linux-x86_64.zip` |
| Linux | aarch64 | `x402-linux-aarch64.zip` |
| macOS | x86_64 | `x402-darwin-x86_64.zip` |
| macOS | aarch64 (Apple Silicon) | `x402-darwin-aarch64.zip` |
| Windows | x86_64 | `x402-windows-x86_64.zip` |

## Installation Steps

### Step 1: Detect Platform

```bash
# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
case "$OS" in
  linux*)  PLATFORM="linux" ;;
  darwin*) PLATFORM="darwin" ;;
  mingw*|msys*|cygwin*) PLATFORM="windows" ;;
  *) echo "Unsupported OS: $OS" && exit 1 ;;
esac

# Detect Architecture
ARCH=$(uname -m)
case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

# Construct filename
ZIP_FILE="x402-${PLATFORM}-${ARCH}.zip"
echo "Platform detected: ${PLATFORM}-${ARCH}"
echo "Will download: ${ZIP_FILE}"
```

### Step 2: Set Installation Directory

The default skill scripts directory is:

```bash
SKILL_DIR="${HOME}/.claude/skills/x402"
SCRIPTS_DIR="${SKILL_DIR}/scripts"

# Create directories if they don't exist
mkdir -p "${SCRIPTS_DIR}"
```

### Step 3: Download skill.md

```bash
REPO_URL="https://raw.githubusercontent.com/anthropics/x402-skill/main"

curl -fsSL "${REPO_URL}/skill.md" -o "${SKILL_DIR}/skill.md"
```

### Step 4: Download Platform-Specific Binaries

```bash
RELEASE_URL="https://github.com/anthropics/x402-skill/releases/latest/download"

curl -fsSL "${RELEASE_URL}/${ZIP_FILE}" -o "/tmp/${ZIP_FILE}"
```

### Step 5: Extract Binaries

```bash
unzip -o "/tmp/${ZIP_FILE}" -d "${SCRIPTS_DIR}"
chmod +x "${SCRIPTS_DIR}"/*
rm "/tmp/${ZIP_FILE}"
```

### Step 6: Verify Installation

```bash
# List installed tools
ls -la "${SCRIPTS_DIR}"

# Verify executables work
"${SCRIPTS_DIR}/get-address" --help
```

## Complete Installation Script

Agents can execute this single script to install the skill:

```bash
#!/bin/bash
set -e

# Configuration
REPO="anthropics/x402-skill"
SKILL_DIR="${HOME}/.claude/skills/x402"
SCRIPTS_DIR="${SKILL_DIR}/scripts"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
case "$OS" in
  linux*)  PLATFORM="linux" ;;
  darwin*) PLATFORM="darwin" ;;
  mingw*|msys*|cygwin*) PLATFORM="windows" ;;
  *) echo "Error: Unsupported OS: $OS" >&2 && exit 1 ;;
esac

ARCH=$(uname -m)
case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *) echo "Error: Unsupported architecture: $ARCH" >&2 && exit 1 ;;
esac

ZIP_FILE="x402-${PLATFORM}-${ARCH}.zip"

echo "Installing x402 skill for ${PLATFORM}-${ARCH}..."

# Create directories
mkdir -p "${SCRIPTS_DIR}"

# Download skill.md
echo "Downloading skill.md..."
curl -fsSL "https://raw.githubusercontent.com/${REPO}/main/skill.md" \
  -o "${SKILL_DIR}/skill.md"

# Download and extract binaries
echo "Downloading ${ZIP_FILE}..."
curl -fsSL "https://github.com/${REPO}/releases/latest/download/${ZIP_FILE}" \
  -o "/tmp/${ZIP_FILE}"

echo "Extracting binaries..."
unzip -o "/tmp/${ZIP_FILE}" -d "${SCRIPTS_DIR}"
chmod +x "${SCRIPTS_DIR}"/*
rm "/tmp/${ZIP_FILE}"

echo "Installation complete!"
echo "Skill directory: ${SKILL_DIR}"
echo "Installed tools:"
ls "${SCRIPTS_DIR}"
```

## Installed Files

After installation, the skill directory will contain:

```
~/.claude/skills/x402/
├── skill.md              # Skill definition for Claude
└── scripts/
    ├── create-wallet     # Create new Ethereum wallet
    ├── get-address       # Get wallet public address
    ├── pay               # Transfer tokens
    ├── x402curl          # HTTP client with 402 handling
    └── x402-config       # Configuration management
```

## Runtime Data

The skill stores wallet and configuration data separately (persists across reinstalls):

```
~/.x402/
├── config.toml           # Network, token, and payment settings
├── wallet.json           # Encrypted wallet keystore
└── password.txt          # Wallet password (auto-generated)
```

## Uninstallation

To remove the skill:

```bash
rm -rf "${HOME}/.claude/skills/x402"
```

## Troubleshooting

### Permission Denied
```bash
chmod +x "${SCRIPTS_DIR}"/*
```

### Binary Not Found for Platform
Check that your platform is supported. Run:
```bash
echo "OS: $(uname -s), Arch: $(uname -m)"
```

### Download Failed
Verify network connectivity and that the release exists:
```bash
curl -I "https://github.com/anthropics/x402-skill/releases/latest"
```
