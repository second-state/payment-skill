# Payment skill for AI agents

A skill for OpenClaw / Claude Code / OpenCode to make and receive payments. Currently supports USDC payments on the Base chain.

## Installation

Send the following message to your agent. It will guide you to setup a wallet if you have not done so.

```
Read https://raw.githubusercontent.com/second-state/payment-skill/refs/heads/main/install.md and follow the instructions to set up USDC payments.
```

## Overview

This project provides four CLI tools for Ethereum wallet management and token payments:

### create-wallet

Creates a new Ethereum-compatible wallet with a secure random private key. The wallet is encrypted using the Web3 Secret Storage standard (keystore format) and saved to disk.

```bash
create-wallet [OPTIONS]
```

Key features:
- Generates cryptographically secure private keys
- Encrypts wallet using scrypt + AES-128-CTR
- Auto-generates secure password if not provided
- Outputs the wallet address to stdout

### get-address

Retrieves the Ethereum address and token balance from an existing wallet without requiring the password.

```bash
get-address [OPTIONS]
```

Key features:
- Reads address directly from keystore (no decryption needed)
- Queries current token balance from blockchain (if network configured)
- Outputs JSON with address, balance, token info, and network
- Creates a new wallet automatically if none exists

### pay

Transfers ERC-20 tokens or native ETH from your wallet to a recipient address.

```bash
pay --to <ADDRESS> --amount <AMOUNT> [OPTIONS]
```

Key features:
- Supports ERC-20 token transfers (USDC, etc.)
- Supports native ETH/gas token transfers
- Checks balance before sending
- Waits for transaction confirmation by default
- Uses configuration for network and token defaults

### x402-config

Manages configuration for all x402 tools. Stores settings in `~/.payment/config.toml`.

```bash
x402-config <COMMAND>
```

Commands:
- `show` - Display all current configuration
- `get <KEY>` - Get a specific config value
- `set <KEY> <VALUE>` - Set config values
- `use-network <PROFILE>` - Apply a predefined network profile (base-sepolia, base-mainnet, etc.)
- `list-networks` - List available network profiles
- `list-keys` - List all valid config keys

## Personal Data Storage

All personal data is stored in the `~/.payment/` folder:

```
~/.payment/
├── config.toml      # Network, token, and payment settings
├── wallet.json      # Encrypted wallet keystore (Web3 Secret Storage format)
└── password.txt     # Wallet password (auto-generated, 600 permissions)
```

This folder persists across skill reinstalls, so your wallet and configuration are preserved.

## Skill Directory Structure

The `skill/` directory contains everything needed for Claude agents to use x402 payments:

```
skill/
├── bootstrap.sh     # Downloads platform-specific binaries on first run
├── skill.md         # Instructions for Claude on how to use the tools
└── scripts/         # CLI binaries installed here by bootstrap.sh
    ├── create-wallet
    ├── get-address
    ├── pay
    └── x402-config
```

### How Installation Works

1. **Clone skill files**: The install script copies the `skill/` directory to `~/.claude/skills/payment/`

2. **Bootstrap binaries**: Running `bootstrap.sh` detects your platform (OS + architecture) and downloads the appropriate pre-compiled binaries from GitHub Releases

3. **Binary installation**: Binaries are extracted to `~/.claude/skills/payment/skill/scripts/` and made executable

Supported platforms:
- Linux x86_64 / aarch64
- macOS x86_64 / aarch64 (Apple Silicon)
- Windows x86_64

### Final installed structure

```
~/.claude/skills/payment/
└── skill/
    ├── bootstrap.sh
    ├── skill.md
    └── scripts/
        ├── create-wallet
        ├── get-address
        ├── pay
        └── x402-config
```

## Development

This is a Rust workspace with the following crates:

| Crate | Description |
|-------|-------------|
| `x402-common` | Shared library for configuration, errors, and utilities |
| `create-wallet` | Wallet creation CLI |
| `get-address` | Address retrieval CLI |
| `pay` | Token payment CLI |
| `x402-config` | Configuration management CLI |
| `x402curl` | HTTP client with x402 payment support (coming soon) |

### Building

```bash
cargo build --release
```

### Running tests

```bash
cargo test
```

### Cross-compilation

The project uses GitHub Actions to build binaries for all supported platforms. See `.github/workflows/release.yml` for the CI configuration.

## License

See [LICENSE](LICENSE) for details.
