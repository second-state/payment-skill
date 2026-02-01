# Payment Skill Installation Guide

This guide explains how to install the payment skill for Claude agents.

## Prerequisites

The bootstrap script requires:
- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)
- `git` (for cloning)

## Quick Install (Recommended)

```bash
SKILL_DIR="${HOME}/.claude/skills/payment"
mkdir -p "$(dirname "$SKILL_DIR")"

# Clone the skill directory
git clone --depth 1 https://github.com/second-state/payment-skill.git /tmp/payment-skill-repo
cp -r /tmp/payment-skill-repo/skill "$SKILL_DIR"
rm -rf /tmp/payment-skill-repo

# Download platform-specific binaries
"${SKILL_DIR}/bootstrap.sh"
```

## Post Install: Configuration

After installation, configure the network and wallet.

### Step 1: Configure network

Ask the user which network to use:

> "Which blockchain network should I use for payments?"
>
> Options:
> - **base-sepolia** (recommended for testing) - Base testnet
> - **base-mainnet** - Base mainnet (real funds)

Based on user's answer, run:

```bash
~/.claude/skills/payment/skill/scripts/x402-config use-network <profile-name>
```

For example:
```bash
~/.claude/skills/payment/skill/scripts/x402-config use-network base-sepolia
```

### Step 2: Create wallet

```bash
~/.claude/skills/payment/skill/scripts/create-wallet
```

**If error "Wallet already exists"**: Ignore and proceed to Step 3.

**If successful**: A new wallet is created.

### Step 3: Get wallet address and balance

```bash
~/.claude/skills/payment/skill/scripts/get-address
```

This outputs JSON with the wallet address and current balance:

```json
{
  "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f...",
  "balance": "0",
  "token": "0x036CbD53842c5426634e7929541eC2318f3dCF7e",
  "token_symbol": "USDC",
  "network": "base-sepolia"
}
```

### Step 4: Ask user to fund the wallet

Show the user the address and balance, then ask them to fund it:

> "Your payment wallet address is: `<address>`
>
> Current USDC balance: `<balance>`
>
> To enable payments, please fund this address on `<network>` with:
> - **USDC** - for making payments
> - **ETH** (0.001 ETH) - for gas fees
>
> You can get testnet tokens from faucets if using base-sepolia."

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
3. Extract to `~/.claude/skills/payment/skill/scripts/`
4. Make executable: `chmod +x ~/.claude/skills/payment/skill/scripts/*`

## Verify Installation

```bash
~/.claude/skills/payment/skill/scripts/get-address --help
```

## Uninstallation

```bash
# Remove skill
rm -rf "${HOME}/.claude/skills/payment"

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
