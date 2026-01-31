# x402 Skill Installation Guide

This guide explains how to install the x402 payment skill for Claude agents.

## Quick Install (Recommended)

Clone the skill directory to your Claude skills location:

```bash
SKILL_DIR="${HOME}/.claude/skills/x402"
mkdir -p "$(dirname "$SKILL_DIR")"

# Clone just the skill directory
git clone --depth 1 https://github.com/second-state/x402-agent-tools.git /tmp/x402-repo
cp -r /tmp/x402-repo/skill "$SKILL_DIR"
rm -rf /tmp/x402-repo

# Make scripts executable
chmod +x "${SKILL_DIR}"/*.sh "${SKILL_DIR}/create-wallet" "${SKILL_DIR}/get-address" \
         "${SKILL_DIR}/pay" "${SKILL_DIR}/x402curl" "${SKILL_DIR}/x402-config"
```

**That's it!** The bootstrap scripts will automatically download platform-specific binaries on first use.

## How It Works

The skill uses a lazy-loading approach:

1. Wrapper scripts in `skill/` detect your platform
2. On first run, binaries are downloaded from GitHub releases
3. Binaries are cached in `skill/scripts/`
4. Subsequent runs use the cached binaries

## Directory Structure

After installation:

```
~/.claude/skills/x402/
├── skill.md              # Skill definition for Claude
├── bootstrap.sh          # Shared bootstrap logic
├── create-wallet         # Wrapper script
├── get-address           # Wrapper script
├── pay                   # Wrapper script
├── x402curl              # Wrapper script
├── x402-config           # Wrapper script
└── scripts/              # Downloaded binaries (auto-populated)
    ├── .gitignore
    └── .gitkeep
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

## Prerequisites

The bootstrap scripts require:
- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)

## Verify Installation

```bash
# Test that wrapper scripts work
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

### Permission Denied
```bash
chmod +x ~/.claude/skills/x402/*.sh ~/.claude/skills/x402/create-wallet \
         ~/.claude/skills/x402/get-address ~/.claude/skills/x402/pay \
         ~/.claude/skills/x402/x402curl ~/.claude/skills/x402/x402-config
```

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
