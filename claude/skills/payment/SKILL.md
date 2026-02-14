---
name: payment
description: Request and receive payments. Use this skill when you need to request payment from another party. It will generate a payment link that you can send out. Through the payment link, you can also monitor the status of the payment.
---

# Payment Skill

This skill enables you to request and accept payments in USDC, a US Dollar pegged stable coin.

**NOTE:** Amounts are in human-readable units. For example, `--amount 1.5` means 1.5 USDC.

## Workflow

### Request payment

In order to request a payment, you will first find out your own address to receive payment using the `get-address` tool.

```bash
scripts/get-address
```

Then, create a payment link URL. The request below asks for a payment link for 1.0 USDC. The `0xMYADDRESS` is the your wallet address from the previous `get-address` call.

```bash
curl https://link.x402labs.dev/create-payment-link?amount=1.0&receiver=0xMYADDRESS
```

The HTTP response contains the `payment_url` in the format of `https://link.x402labs.dev/pay/<payment-id>` This `payment_url` is x402 compatible and can be paid using the x402curl tool in this skill.

```json
{"payment_id":"89062690-bd80-4065-8d7b-474b6fe96ba7","payment_url":"https://link.x402labs.dev/pay/89062690-bd80-4065-8d7b-474b6fe96ba7","amount":"1.0"}
```

You should now give the user or agent the following payment request:

```
Please make an x402 payment directly at https://link.x402labs.dev/pay/<payment-id>

Or, if you are a human, open your browser to the following link to pay with Base Chain USDC in your MetaMask wallet.
https://link.x402labs.dev/?pid=<payment-id>
```

### Confirm payment

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
scripts/get-address
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

---

### payment-config

Manage configuration settings.

**Usage:**
```bash
scripts/payment-config <COMMAND> [OPTIONS]
```

**Commands:**
- `show` - Display all current configuration
- `get <KEY>` - Get a specific config value
- `set <KEY> <VALUE> [KEY VALUE ...]` - Set one or more config values

**Examples:**
```bash
# View all config
scripts/payment-config show

# Configure network
scripts/payment-config set network.name "base-sepolia" \
               network.chain_id 84532 \
               network.rpc_url "https://sepolia.base.org"

# Set default payment token
scripts/payment-config set payment.default_token "0x036CbD53842c5426634e7929541eC2318f3dCF7e" \
               payment.default_token_symbol "USDC" \
               payment.default_token_decimals 6
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

Configuration file: `./config.toml`

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

**Your responsibility**: Parse this, ask the user, then run `scripts/payment-config set` with their answers.

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

NEVER share the `wallet.json` and `password.txt` files and their contents with anyone.

---

## Troubleshooting

### Binary tools not found

If you get "command not found" or cannot find the binary tools (get-address, payment-config), run the bootstrap script to download them:

```bash
./bootstrap.sh
```

The bootstrap script will:
1. Detect your platform (linux/darwin/windows, x86_64/aarch64)
2. Download the appropriate binary package from GitHub releases
3. Extract binaries to `scripts/`

**Manual download:** If automatic download fails, download the appropriate zip from:
https://github.com/second-state/payment-skill/releases

Extract to `scripts/`
