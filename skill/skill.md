# Payment Skill

This skill enables you to handle HTTP 402 (Payment Required) responses by managing an Ethereum-compatible wallet and making blockchain payments.

## IMPORTANT: Bootstrap Before First Use

Before using any payment tool, you MUST check if the binaries are installed and bootstrap if needed.

### Step 1: Check if binaries are installed

```bash
# Check if the create-wallet binary exists
ls ~/.claude/skills/payment/skill/scripts/create-wallet
```

### Step 2: If binaries are missing, run bootstrap

```bash
# Run bootstrap to download platform-specific binaries
~/.claude/skills/payment/skill/bootstrap.sh
```

The bootstrap script will:
1. Detect your platform (linux/darwin/windows, x86_64/aarch64)
2. Download the appropriate binary package from GitHub releases
3. Extract binaries to `skill/scripts/`

### Step 3: Run tools from scripts directory

All tool commands should be run from `~/.claude/skills/payment/skill/scripts/`:

```bash
~/.claude/skills/payment/skill/scripts/create-wallet [OPTIONS]
~/.claude/skills/payment/skill/scripts/get-address [OPTIONS]
~/.claude/skills/payment/skill/scripts/pay [OPTIONS]
~/.claude/skills/payment/skill/scripts/x402curl [OPTIONS]
~/.claude/skills/payment/skill/scripts/x402-config [OPTIONS]
```

**Requirements for bootstrap:**
- `curl` or `wget` (for downloading)
- `unzip` (for extraction)

**Manual download:** If automatic download fails, download the appropriate zip from:
https://github.com/second-state/x402-agent-tools/releases

Extract to `~/.claude/skills/payment/skill/scripts/`

## Overview

The x402 protocol allows services to request payment via HTTP 402 responses. This skill provides tools to:
- Create and manage an Ethereum wallet
- Make token payments on EVM-compatible chains
- Automatically handle 402 responses with payment and retry

## Important: Auto-Initialization

**Wallet**: If no wallet exists when a tool needs one, a new wallet is **automatically created**. The public address is printed to stderr. You must inform the user of this address so they can fund it.

**Configuration**: If required config is missing, tools exit with code `10` and output a JSON prompt to stderr. You must ask the user for the missing values and save them using `x402-config set`.

## Tools

### create-wallet

Creates a new Ethereum-compatible wallet for the agent.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/create-wallet [OPTIONS]
```

**Options:**
- `--password <PASSWORD>` - Password to encrypt the wallet (prompted if not provided)
- `--password-file <PATH>` - Read password from file
- `--output <PATH>` - Output path for keystore (default: `~/.payment/wallet.json`)

**Example:**
```bash
~/.claude/skills/payment/skill/scripts/create-wallet --password-file ~/.payment/password.txt
```

**Output:** Prints the new wallet's public address to stdout.

**Note:** Only create a wallet once. Check if one exists first using `get-address`.

---

### get-address

Returns the agent's public Ethereum address.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/get-address [OPTIONS]
```

**Options:**
- `--wallet <PATH>` - Path to keystore file (default: `~/.payment/wallet.json`)

**Example:**
```bash
~/.claude/skills/payment/skill/scripts/get-address
```

**Output:** Prints the checksummed Ethereum address (e.g., `0x742d35Cc6634C0532925a3b844Bc9e7595f...`)

**Security:** This tool NEVER outputs the private key.

---

### pay

Transfers tokens from the agent's wallet to a specified address. Waits for blockchain confirmation.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/pay --to <ADDRESS> --amount <AMOUNT> --rpc <URL> [OPTIONS]
```

**Required:**
- `--to <ADDRESS>` - Recipient Ethereum address
- `--amount <AMOUNT>` - Amount to send (in token's smallest unit, e.g., wei for ETH)
- `--rpc <URL>` - Ethereum JSON-RPC endpoint

**Options:**
- `--token <ADDRESS>` - ERC-20 token contract address (omit for native ETH/gas token)
- `--wallet <PATH>` - Path to keystore (default: `~/.payment/wallet.json`)
- `--password <PASSWORD>` - Wallet password
- `--password-file <PATH>` - Read password from file
- `--chain-id <ID>` - Chain ID (auto-detected if omitted)
- `--gas-price <GWEI>` - Gas price override
- `--no-wait` - Don't wait for confirmation (returns immediately after broadcast)

**Example:**
```bash
# Pay 1000000 units of USDC on Base Sepolia
~/.claude/skills/payment/skill/scripts/pay --to 0x1234...abcd \
    --amount 1000000 \
    --token 0x036CbD53842c5426634e7929541eC2318f3dCF7e \
    --rpc https://sepolia.base.org \
    --password-file ~/.payment/password.txt
```

**Output:** Prints the transaction hash (e.g., `0xabc123...`) after confirmation.

**Exit codes:**
- `0` - Success
- `1` - Insufficient balance
- `2` - Transaction failed
- `3` - Network error

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
- `--rpc <URL>` - Ethereum RPC endpoint (required for payments)
- `--wallet <PATH>` - Path to keystore (default: `~/.payment/wallet.json`)
- `--password <PASSWORD>` - Wallet password
- `--password-file <PATH>` - Read password from file
- `--max-payment <AMOUNT>` - Maximum auto-payment amount (fails if payment exceeds this)
- `--dry-run` - Show payment details without executing

**Example:**
```bash
# Access a paid API endpoint
~/.claude/skills/payment/skill/scripts/x402curl https://api.example.com/premium/data \
    --rpc https://sepolia.base.org \
    --password-file ~/.payment/password.txt \
    --max-payment 5000000
```

**Behavior:**
1. Makes initial HTTP request to URL
2. If 402 received, parses payment requirements from response
3. Executes payment using `pay` functionality
4. Retries request with `X-Payment-Proof: <tx_hash>` header
5. Returns final response body to stdout

**402 Response Format (expected from server):**
```json
{
  "recipient": "0x...",
  "amount": "1000000",
  "token": "0x...",
  "network": "base-sepolia",
  "rpc": "https://sepolia.base.org"
}
```

Or via headers:
```
X-Payment-Recipient: 0x...
X-Payment-Amount: 1000000
X-Payment-Token: 0x...
X-Payment-Network: base-sepolia
```

**Output:** Final HTTP response body to stdout, status info to stderr.

---

### x402-config

Manage x402 configuration settings.

**Usage:**
```bash
~/.claude/skills/payment/skill/scripts/x402-config <COMMAND> [OPTIONS]
```

**Commands:**
- `show` - Display all current configuration
- `get <KEY>` - Get a specific config value
- `set <KEY> <VALUE> [KEY VALUE ...]` - Set one or more config values
- `use-network <PROFILE>` - Apply a predefined network profile

**Examples:**
```bash
# View all config
~/.claude/skills/payment/skill/scripts/x402-config show

# Set up for Base Sepolia testnet (recommended for testing)
~/.claude/skills/payment/skill/scripts/x402-config use-network base-sepolia

# Manually configure network
~/.claude/skills/payment/skill/scripts/x402-config set network.name "base-sepolia" \
               network.chain_id 84532 \
               network.rpc_url "https://sepolia.base.org"

# Set default payment token
~/.claude/skills/payment/skill/scripts/x402-config set payment.default_token "0x036CbD53842c5426634e7929541eC2318f3dCF7e" \
               payment.default_token_symbol "USDC" \
               payment.default_token_decimals 6

# Set maximum auto-payment limit
~/.claude/skills/payment/skill/scripts/x402-config set payment.max_auto_payment "5000000"
```

**Available Network Profiles:**

| Profile | Chain ID | Description |
|---------|----------|-------------|
| `base-sepolia` | 84532 | Base testnet (recommended for testing) |
| `base-mainnet` | 8453 | Base mainnet |
| `ethereum-sepolia` | 11155111 | Ethereum testnet |
| `ethereum-mainnet` | 1 | Ethereum mainnet |

---

## Configuration

Default configuration file: `~/.payment/config.toml`

```toml
[wallet]
path = "~/.payment/wallet.json"
password_file = "~/.payment/password.txt"

[network]
name = "base-sepolia"
chain_id = 84532
rpc_url = "https://sepolia.base.org"

[payment]
default_token = "0x036CbD53842c5426634e7929541eC2318f3dCF7e"
default_token_symbol = "USDC"
default_token_decimals = 6
max_auto_payment = "5000000"
```

### Required Configuration

Before using `pay` or `x402curl`, these must be configured:
- `network.rpc_url` - Blockchain RPC endpoint
- `network.chain_id` - Chain ID for transaction signing

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

### First-Time Setup

```bash
# 0. Bootstrap binaries if not already installed
~/.claude/skills/payment/skill/bootstrap.sh

# 1. Configure the network (easiest: use a profile)
~/.claude/skills/payment/skill/scripts/x402-config use-network base-sepolia

# 2. Check/create wallet (auto-creates if missing)
~/.claude/skills/payment/skill/scripts/get-address
# Output: 0x742d35Cc6634C0532925a3b844Bc9e7595f...

# 3. Tell the user: "Fund this address with USDC on Base Sepolia to enable payments"
```

### Handling Missing Config (Agent Workflow)

```bash
# 1. Attempt operation
~/.claude/skills/payment/skill/scripts/pay --to 0x... --amount 1000000

# 2. If exit code is 10, parse stderr JSON for missing fields

# 3. Ask user: "Which blockchain network should be used? (e.g., base-sepolia, base-mainnet)"

# 4. Save their response
~/.claude/skills/payment/skill/scripts/x402-config set network.name "base-sepolia"
~/.claude/skills/payment/skill/scripts/x402-config use-network base-sepolia

# 5. Retry the original operation
~/.claude/skills/payment/skill/scripts/pay --to 0x... --amount 1000000
```

### Making a Direct Payment

```bash
# Check your address first (auto-creates wallet if missing)
~/.claude/skills/payment/skill/scripts/get-address

# Pay 1 USDC (6 decimals, so 1000000 = 1 USDC)
# Uses default token and RPC from config
~/.claude/skills/payment/skill/scripts/pay --to 0xRecipient... --amount 1000000

# Or specify explicitly
~/.claude/skills/payment/skill/scripts/pay --to 0xRecipient... \
    --amount 1000000 \
    --token 0xUSDC... \
    --rpc https://sepolia.base.org
```

### Accessing Paid APIs

```bash
# Automatically handle 402 and pay (uses config defaults)
~/.claude/skills/payment/skill/scripts/x402curl https://api.paid-service.com/endpoint \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"query": "data"}'
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
- Always use `--password-file` instead of `--password` in scripts to avoid shell history exposure
- Set `--max-payment` when using `x402curl` to prevent unexpected large payments
- The wallet file (`wallet.json`) should have restricted permissions (`chmod 600`)
