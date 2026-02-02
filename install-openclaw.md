# Payment Skill Installation Guide

This guide explains how to install the payment skill for OpenClaw agents.

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
cp -r /tmp/payment-skill-repo/skill/* "$SKILL_DIR"
rm -rf /tmp/payment-skill-repo

# Download platform-specific binaries
"${SKILL_DIR}/bootstrap.sh"
```

## Post Install: Configuration

After installation, create a wallet and fund it. The default network is **base-mainnet** with USDC.

### Step 1: Create wallet

```bash
~/.openclaw/skills/payment/scripts/create-wallet
```

**If error "Wallet already exists"**: Ignore and proceed to Step 2.

**If successful**: A new wallet is created.

### Step 2: Get wallet address and balance

```bash
~/.openclaw/skills/payment/scripts/get-address
```

This outputs JSON with the wallet address and current balance:

```json
{
  "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f...",
  "balance": "0",
  "token": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
  "token_symbol": "USDC",
  "network": "base-mainnet"
}
```

### Step 3: Ask user to fund the wallet

Show the user the address and balance, then ask them to fund it:

> "Your payment wallet address is: `<address>`
>
> Current USDC balance: `<balance>`
>
> In order for you to pay other agents or humans, please fund this address on base-mainnet with:
> - **USDC** - for making payments
> - **ETH** (0.001 ETH) - for gas fees"

---

## Runtime Data

Wallet and configuration are stored separately (persists across reinstalls):

```
~/.payment/
├── config.toml           # Network, token, and payment settings
├── wallet.json           # Encrypted wallet keystore
└── password.txt          # Wallet password (auto-generated)
```

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

## Verify Installation

```bash
~/.openclaw/skills/payment/scripts/get-address --help
```

## Uninstallation

```bash
# Remove skill
rm -rf "${HOME}/.openclaw/skills/payment"

# Optionally remove wallet data (CAUTION: deletes wallet!)
# rm -rf "${HOME}/.payment"
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
