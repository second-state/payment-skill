# x402 Skill Installation Guide

This guide explains how to install the x402 payment skill for Claude agents.

## Prerequisites

The bootstrap script requires:
- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)
- `git` (for cloning)

## Quick Install (Recommended)

```bash
SKILL_DIR="${HOME}/.claude/skills/x402"
mkdir -p "$(dirname "$SKILL_DIR")"

# Clone the skill directory
git clone --depth 1 https://github.com/second-state/x402-agent-tools.git /tmp/x402-repo
cp -r /tmp/x402-repo/skill "$SKILL_DIR"
rm -rf /tmp/x402-repo

# Download platform-specific binaries
"${SKILL_DIR}/bootstrap.sh"
```

## Runtime Data

Wallet and configuration are stored separately (persists across reinstalls):

```
~/.x402/
├── config.toml           # Network, token, and payment settings
├── wallet.json           # Encrypted wallet keystore
└── password.txt          # Wallet password (auto-generated)
```

## Manual Binary Installation

If automatic download fails, manually download binaries:

1. Go to https://github.com/second-state/x402-agent-tools/releases/latest
2. Download the zip for your platform:
   - `x402-linux-x86_64.zip`
   - `x402-linux-aarch64.zip`
   - `x402-darwin-x86_64.zip`
   - `x402-darwin-aarch64.zip`
   - `x402-windows-x86_64.zip`
3. Extract to `~/.claude/skills/x402/skill/scripts/`
4. Make executable: `chmod +x ~/.claude/skills/x402/skill/scripts/*`

## Verify Installation

```bash
~/.claude/skills/x402/skill/scripts/get-address --help
```

## Uninstallation

```bash
# Remove skill
rm -rf "${HOME}/.claude/skills/x402"

# Optionally remove wallet data (CAUTION: deletes wallet!)
# rm -rf "${HOME}/.x402"
```

## Troubleshooting

### Download Failed
Check network connectivity:
```bash
curl -I "https://github.com/second-state/x402-agent-tools/releases/latest"
```

### Unsupported Platform
Check your platform:
```bash
echo "OS: $(uname -s), Arch: $(uname -m)"
```

Supported: Linux/macOS/Windows on x86_64/aarch64
