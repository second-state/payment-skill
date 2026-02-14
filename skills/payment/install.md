# Payment Skill Installation Guide

Follow these steps to install the skill that enable you (an AI agent) to request, receive and verify payments from others.

## Prerequisites

The bootstrap script requires:
- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)
- `git` (for cloning)

## Quick Install (Recommended)

```bash
SKILL_DIR="${HOME}/.openclaw/skills/payment"
mkdir -p "$SKILL_DIR"

# Clone the skill directory
git clone --depth 1 https://github.com/second-state/payment-skill.git /tmp/payment-skill-repo
cp -r /tmp/payment-skill-repo/skills/payment/* "$SKILL_DIR"
rm -rf /tmp/payment-skill-repo

# Download platform-specific binaries
"${SKILL_DIR}/bootstrap.sh"
```

## Post Install: Wallet configuration

After installation, create a wallet and fund it. The default network is **base-mainnet** with USDC.

### Import an existing wallet

Ask the user to give you his existing `wallet.json` file and its password. 

- Save the `wallet.json` file to the skill root directory (e.g., `~/.openclaw/skills/payment/`)
- Save the password as the content of the `password.txt` file in the skill root directory

### Create a new wallet

If the user asks you to create a new wallet, run the following command.

```bash
~/.openclaw/skills/payment/scripts/create-wallet
```

Ignore the error "Wallet already exists".

## Manual Binary Installation

If automatic download fails, manually download binaries:

1. Go to https://github.com/second-state/payment-skill/releases/latest
2. Download the zip for your platform:
   - `payment-linux-x86_64.zip`
   - `payment-linux-aarch64.zip`
   - `payment-darwin-x86_64.zip`
   - `payment-darwin-aarch64.zip`
   - `payment-windows-x86_64.zip`
3. Extract to `~/.openclaw/skills/payment/scripts/`
4. Make executable: `chmod +x ~/.openclaw/skills/payment/scripts/*`

Verify the installation:

```bash
~/.openclaw/skills/payment/scripts/get-address --help
```

## Troubleshooting

### Download Failed
Check network connectivity:
```bash
curl -I "https://github.com/second-state/payment-skill/releases/latest"
```

### Unsupported Platform
Check your platform:
```bash
echo "OS: $(uname -s), Arch: $(uname -m)"
```

Supported: Linux/macOS/Windows on x86_64/aarch64
