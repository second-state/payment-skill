# Payment Skill

This skill enables you to request and accept payments through USDC on the blockchain.

**NOTE:** Amounts are in human-readable units. For example, `--amount 1.5` means 1.5 USDC.

## Overview

This skill provides tools to:
- Check wallet address and balance
- Request payment from user or another agent
- Make direct USDC token payments on Ethereum and Base chains
- Make USDC payments through an x402 payment link

## Request payment

In order to request a payment, you will first create a payment link URL. The request below asks for a payment link for 1.0 USDC.

```bash
curl https://link.x402labs.dev/create-payment-link?amount=1.0
```

The HTTP response contains the `payment_url` in the format of `https://link.x402labs.dev/pay/<payment-id>` This `payment_url` is x402 compatible and can be paid using the x402curl tool in this skill.

```json
{"payment_id":"89062690-bd80-4065-8d7b-474b6fe96ba7","payment_url":"https://link.x402labs.dev/pay/89062690-bd80-4065-8d7b-474b6fe96ba7","amount":"1.0"}
```

You should now give the user or agent the `payment_url`.

Once they tell you that they have paid, you will check the URL `https://link.x402labs.dev/status/<payment-id>`

```bash
curl https://link.x402labs.dev/status/89062690-bd80-4065-8d7b-474b6fe96ba7
```

Look for the `paid` and `tx` fields in the response JSON.

If the user has successfully paid, you will see the following response. You can now perform the task they ask for.

```json
{"payment_id":"89062690-bd80-4065-8d7b-474b6fe96ba7","amount":1.0,"paid":true,"tx":"0xTRANSACTION-ID"}
```

If the user has not paid, you will see the following response. You should insist that they pay first.

```json
{"payment_id":"89062690-bd80-4065-8d7b-474b6fe96ba7","amount":1.0,"paid":false,"tx":null}
```

## Tools

### get-address

Returns the agent's public Ethereum address and current token balance as JSON.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/get-address
```

**Output:** JSON with address and balance (if network is configured):
```json
{
  "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f...",
  "balance": "1.5",
  "token": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
  "token_symbol": "USDC",
  "network": "base-mainnet"
}
```

**Fields:**
- `address` - Public Ethereum address (always present)
- `balance` - Token balance in human-readable units, e.g., "1.5" for 1.5 USDC (if network configured)
- `token` - ERC-20 token contract address (if configured)
- `token_symbol` - Token symbol, e.g., "USDC" (if configured)
- `network` - Network name (if configured)

**Security:** This tool NEVER outputs the private key.

---

### pay

Transfers tokens from the agent's wallet to a specified address. Waits for blockchain confirmation.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/pay --to <ADDRESS> --amount <AMOUNT> [OPTIONS]
```

**Required:**
- `--to <ADDRESS>` - Recipient Ethereum address
- `--amount <AMOUNT>` - Amount to send in human-readable units (e.g., 1.5 for 1.5 USDC)

**Options:**
- `--gas-price <GWEI>` - Gas price in Gwei (auto-detected from network if omitted)

**Example:**
```bash
# Pay 1 USDC
~/.claude/skills/payment/skill/scripts/pay --to 0x1234...abcd --amount 1

# Pay 0.5 USDC
~/.claude/skills/payment/skill/scripts/pay --to 0x1234...abcd --amount 0.5
```

**Output:** Prints the transaction hash (e.g., `0xabc123...`) after confirmation.

**Exit codes:**
- `0` - Success
- `1` - Insufficient balance
- `2` - Transaction failed
- `3` - Network error

**Tip:** If the transaction does not go through (stuck pending or times out), retry with a higher gas price:
```bash
~/.claude/skills/payment/skill/scripts/pay --to 0x1234...abcd --amount 1 --gas-price 0.5
```

---

### x402curl

A curl wrapper that automatically handles HTTP 402 Payment Required responses.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/x402curl <URL> [OPTIONS]
```

**Required:**
- `<URL>` - The URL to request

**Options:**
- `-X, --request <METHOD>` - HTTP method (GET, POST, etc.)
- `-H, --header <HEADER>` - Add header (can be repeated)
- `-d, --data <DATA>` - Request body
- `--max-payment <AMOUNT>` - Maximum auto-payment in human units, e.g., 5 for 5 USDC (fails if payment exceeds this)

**Example:**
```bash
# Access a paid API endpoint, auto-pay up to 5 USDC
~/.claude/skills/payment/skill/scripts/x402curl https://link.x402labs.dev/pay/<payment-id> \
    --max-payment 5
```

**Output:** Final HTTP response body to stdout, status info to stderr.

---

### x402-config

Manage configuration settings.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/x402-config <COMMAND> [OPTIONS]
```

**Commands:**
- `show` - Display all current configuration
- `get <KEY>` - Get a specific config value
- `set <KEY> <VALUE> [KEY VALUE ...]` - Set one or more config values

**Examples:**
```bash
# View all config
~/.claude/skills/payment/skill/scripts/x402-config show

# Configure network
~/.claude/skills/payment/skill/scripts/x402-config set network.name "base-sepolia" \
               network.chain_id 84532 \
               network.rpc_url "https://sepolia.base.org"

# Set default payment token
~/.claude/skills/payment/skill/scripts/x402-config set payment.default_token "0x036CbD53842c5426634e7929541eC2318f3dCF7e" \
               payment.default_token_symbol "USDC" \
               payment.default_token_decimals 6

# Set maximum auto-payment limit (5 USDC)
~/.claude/skills/payment/skill/scripts/x402-config set payment.max_auto_payment "5"
```

**Available Configuration Keys:**

| Key | Description |
|-----|-------------|
| `wallet.path` | Path to wallet keystore file |
| `wallet.password_file` | Path to password file |
| `network.name` | Network name (e.g., "base-mainnet") |
| `network.chain_id` | Chain ID for transaction signing |
| `network.rpc_url` | Blockchain RPC endpoint URL |
| `payment.default_token` | Default ERC-20 token contract address |
| `payment.default_token_symbol` | Token symbol (e.g., "USDC") |
| `payment.default_token_decimals` | Token decimals (e.g., 6 for USDC) |
| `payment.max_auto_payment` | Maximum auto-payment amount |

---

## Configuration

Configuration file: `~/.payment/config.toml`

### Missing Config Behavior

When required config is missing, tools output JSON to stderr:
```json
{
  "error": "missing_config",
  "missing_fields": ["network.rpc_url", "network.chain_id"],
  "prompt": "Please provide the following configuration:",
  "questions": [
    {
      "field": "network.name",
      "question": "Which blockchain network should be used for payments?",
      "examples": ["base-sepolia", "base-mainnet"]
    }
  ]
}
```

**Your responsibility**: Parse this, ask the user, then run `x402-config set` with their answers.

---

## Common Workflows

### Making a Direct Payment

```bash
# Pay 1 USDC
~/.claude/skills/payment/skill/scripts/pay --to 0xRecipient... --amount 1

# Pay 2.5 USDC
~/.claude/skills/payment/skill/scripts/pay --to 0xRecipient... --amount 2.5
```

### Making a payment through a payment link

```bash
# Automatically handle 402 and pay (uses config defaults)
~/.claude/skills/payment/skill/scripts/x402curl https://link.x402labs.dev/pay/<payment-id>
```

---

## Supported Networks

| Network | Chain ID | Native Token | Common RPC |
|---------|----------|--------------|------------|
| Base Sepolia | 84532 | ETH | https://sepolia.base.org |
| Base Mainnet | 8453 | ETH | https://mainnet.base.org |
| Ethereum Sepolia | 11155111 | ETH | https://rpc.sepolia.org |
| Ethereum Mainnet | 1 | ETH | https://eth.llamarpc.com |

---

## Security Notes

- **Private keys** are stored encrypted and never exposed via any tool
- Set `--max-payment` when using `x402curl` to prevent unexpected large payments
- The wallet file (`wallet.json`) has restricted permissions (`chmod 600`)

---

## Troubleshooting

### Binary tools not found

If you get "command not found" or cannot find the binary tools (get-address, pay, x402curl, x402-config), run the bootstrap script to download them:

```bash
~/.claude/skills/payment/skill/bootstrap.sh
```

The bootstrap script will:
1. Detect your platform (linux/darwin/windows, x86_64/aarch64)
2. Download the appropriate binary package from GitHub releases
3. Extract binaries to `~/.claude/skills/payment/skill/scripts/`

**Manual download:** If automatic download fails, download the appropriate zip from:
https://github.com/second-state/payment-skill/releases

Extract to `~/.claude/skills/payment/skill/scripts/`
