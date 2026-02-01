# Project Guidance

## IMPORTANT: Git Workflow

**NEVER push directly to main branch.** Always:
1. Create a new branch for changes
2. Commit changes to the branch
3. Create a Pull Request (PR)
4. Wait for user approval before merging

Example workflow:
```bash
git checkout -b feature-branch-name
# make changes
git add <files>
git commit -m "message"
git push -u origin feature-branch-name
# Then create PR using: gh pr create --title "..." --body "..."
```

## Project Overview

Payment skill for AI agents (Claude Code, OpenClaw, OpenCode) to make and receive USDC payments on the blockchain.

## Development

- **Rust version**: 1.93.0
- **Build**: `cargo build --release`
- **Test**: `cargo test --all`
- **Format**: `cargo fmt --all`
- **Lint**: `cargo clippy --all`

## Directory Structure

- `x402-common/` - Shared library (config, wallet, errors)
- `create-wallet/` - Wallet creation tool
- `get-address/` - Get wallet address and balance
- `pay/` - Make payments
- `payment-config/` - Configuration management
- `x402curl/` - HTTP client with 402 payment handling
- `skill/` - Skill files for Claude Code
