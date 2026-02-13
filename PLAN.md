# x402 Payment Skill - Implementation Plan

## Overview

This skill enables Claude agents to handle HTTP 402 (Payment Required) responses by managing an Ethereum-compatible wallet and making payments.

## Directory Structure

```
x402_skill/
├── Cargo.toml              # Workspace manifest
├── PLAN.md                 # This file
├── payment-common/         # Shared library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── config.rs       # Configuration loading/saving
│       ├── wallet.rs       # Wallet operations
│       └── error.rs        # Common error types
├── create-wallet/
│   ├── Cargo.toml
│   └── src/main.rs
├── get-address/
│   ├── Cargo.toml
│   └── src/main.rs
├── pay/
│   ├── Cargo.toml
│   └── src/main.rs
├── x402curl/
│   ├── Cargo.toml
│   └── src/main.rs
└── payment-config/            # Configuration management tool
    ├── Cargo.toml
    └── src/main.rs
```

## Environment

- **Rust version**: 1.93.0
- **Cargo version**: 1.93.0
- **Rust environment**: `source "$HOME/.cargo/env"` (required before running cargo commands)

## Tool Specifications

### 1. `create-wallet`

**Purpose**: Generate a new Ethereum-compatible wallet for the agent.

**Inputs**:
- `--password <PASSWORD>` (optional): Password to encrypt the wallet keystore
- `--output <PATH>` (optional): Path to store the encrypted keystore (default: `wallet.json` in skill root)

**Outputs**:
- Creates encrypted keystore file
- Prints the public address to stdout

**Security Considerations**:
- Private key must NEVER be printed or logged
- Use industry-standard keystore encryption (Web3 Secret Storage)
- Derive address from secp256k1 keypair using keccak256 hash

**Dependencies**:
- `secp256k1` - Key generation
- `ethers` - Address derivation, keystore format
- `aes-gcm` / `argon2` - Encryption
- `rand` - Secure random generation
- `clap` - CLI parsing

---

### 2. `get-address`

**Purpose**: Retrieve the agent's public Ethereum address from the stored wallet.

**Inputs**:
- `--wallet <PATH>` (optional): Path to keystore file (default: `wallet.json` in skill root)

**Outputs**:
- Prints the public address (0x-prefixed, checksummed) to stdout

**Security Considerations**:
- Must NOT output the private key
- Must NOT output the encrypted keystore content
- Only reads the address field from keystore

**Dependencies**:
- `ethers` - Keystore parsing
- `serde_json` - JSON parsing
- `clap` - CLI parsing

---

### 3. `pay`

**Purpose**: Transfer tokens from the agent's wallet to a specified address.

**Inputs**:
- `--to <ADDRESS>` (required): Recipient Ethereum address
- `--amount <AMOUNT>` (required): Amount to transfer (in smallest unit or with decimal)
- `--token <ADDRESS>` (optional): ERC-20 token contract address (native ETH if omitted)
- `--rpc <URL>` (required): Ethereum RPC endpoint
- `--wallet <PATH>` (optional): Path to keystore file
- `--password <PASSWORD>` or `--password-file <PATH>`: Password to decrypt wallet
- `--chain-id <ID>` (optional): Chain ID for EIP-155 (auto-detect if omitted)
- `--gas-price <GWEI>` (optional): Gas price override
- `--wait` (flag): Wait for transaction confirmation (default: true)

**Outputs**:
- Prints transaction hash to stdout
- Exits with code 0 on success, non-zero on failure

**Behavior**:
- Decrypts wallet using password
- Constructs and signs transaction
- Broadcasts to network
- Waits for inclusion in a block (with `--wait`)
- Returns transaction hash

**Security Considerations**:
- Password should be read from environment/file, not command line in production
- Validate recipient address format
- Check balance before sending

**Dependencies**:
- `ethers` - Transaction construction, signing, provider
- `tokio` - Async runtime
- `clap` - CLI parsing
- `anyhow` - Error handling

---

### 4. `x402curl`

**Purpose**: HTTP client that automatically handles 402 Payment Required responses.

**Inputs**:
- Standard curl-like arguments: URL, headers, method, body
- `--wallet <PATH>` (optional): Path to keystore file
- `--password <PASSWORD>` or `--password-file <PATH>`: Wallet password
- `--rpc <URL>` (required for payments): Ethereum RPC endpoint
- `--max-payment <AMOUNT>` (optional): Maximum payment amount to auto-approve

**Outputs**:
- Final HTTP response body to stdout
- Status information to stderr

**Behavior (x402 Protocol Flow)**:
1. Make initial HTTP request
2. If response is 402:
   - Parse `X-Payment-Required` header or JSON body for payment details
   - Extract: recipient address, amount, token, payment network
   - Execute payment using `pay` functionality
   - Retry original request with `X-Payment-Proof: <tx_hash>` header
3. Return final response

**402 Response Format** (expected):
```json
{
  "recipient": "0x...",
  "amount": "1000000",
  "token": "0x...",
  "network": "base-sepolia",
  "rpc": "https://..."
}
```

**Dependencies**:
- `reqwest` - HTTP client
- `ethers` - Payment execution
- `tokio` - Async runtime
- `serde_json` - JSON parsing
- `clap` - CLI parsing

---

## Shared Components

Consider creating a shared library crate (`payment-common`) for:
- Wallet loading/decryption
- Configuration file handling (`config.toml` in skill root)
- Common types and error handling
- Auto-initialization logic

---

## Configuration

Default config location: `config.toml` in the skill root directory (resolved relative to the executable binary).

```toml
[wallet]
path = "wallet.json"
password_file = "password.txt"

[network]
# Default blockchain network
name = "base-sepolia"
chain_id = 84532
rpc_url = "https://sepolia.base.org"

[payment]
# Default token for payments (empty = native gas token like ETH)
default_token = "0x036CbD53842c5426634e7929541eC2318f3dCF7e"
default_token_symbol = "USDC"
default_token_decimals = 6
max_auto_payment = "5000000"  # in smallest token unit
```

### Configuration Fields

| Field | Description | Required |
|-------|-------------|----------|
| `network.name` | Human-readable network name (e.g., "base-sepolia", "ethereum-mainnet") | Yes |
| `network.chain_id` | EIP-155 chain ID for transaction signing | Yes |
| `network.rpc_url` | JSON-RPC endpoint URL for the blockchain | Yes |
| `payment.default_token` | ERC-20 token contract address (empty for native token) | No |
| `payment.default_token_symbol` | Token symbol for display (e.g., "USDC", "ETH") | No |
| `payment.default_token_decimals` | Token decimals for amount formatting | No |
| `payment.max_auto_payment` | Maximum auto-approved payment amount | No |

---

## Auto-Initialization Behavior

### Missing Wallet Handling

When any tool requires a wallet and none exists at the configured path:

1. **Automatically create a new wallet**
   - Generate secure random private key
   - Encrypt with auto-generated password (stored in `password_file`)
   - Save to configured wallet path
   - Set file permissions to `600`

2. **Output to stderr:**
   ```
   No wallet found. Created new wallet.
   Public address: 0x742d35Cc6634C0532925a3b844Bc9e7595f...

   IMPORTANT: Fund this address before making payments.
   ```

3. **Continue with the requested operation** (if possible)

### Missing Configuration Handling

When a tool requires configuration that is missing or incomplete:

1. **Exit with specific error code** (exit code `10` for missing config)

2. **Output structured prompt to stderr:**
   ```json
   {
     "error": "missing_config",
     "missing_fields": ["network.rpc_url", "network.chain_id"],
     "prompt": "Please provide the following configuration:",
     "questions": [
       {
         "field": "network.name",
         "question": "Which blockchain network should be used for payments?",
         "examples": ["base-sepolia", "base-mainnet", "ethereum-mainnet"],
         "default": "base-sepolia"
       },
       {
         "field": "network.rpc_url",
         "question": "What is the RPC endpoint URL for this network?",
         "examples": ["https://sepolia.base.org", "https://mainnet.base.org"]
       }
     ]
   }
   ```

3. **Agent responsibility:**
   - Parse the JSON error output
   - Ask the user for the missing values
   - Call a config tool to save the values (see below)

### New Tool: `payment-config`

**Purpose**: View and set configuration values.

**Usage:**
```bash
# View all config
payment-config show

# Get specific value
payment-config get network.rpc_url

# Set specific value
payment-config set network.rpc_url "https://sepolia.base.org"

# Set multiple values
payment-config set network.name "base-sepolia" \
                 network.chain_id 84532 \
                 network.rpc_url "https://sepolia.base.org"

# Initialize with interactive prompts (for manual use)
payment-config init
```

**Behavior:**
- Creates the skill root directory if it doesn't exist
- Creates `config.toml` if it doesn't exist
- Validates values where possible (e.g., chain_id is numeric, rpc_url is valid URL)

---

## Tool Behavior Summary

| Scenario | Behavior |
|----------|----------|
| Wallet missing | Auto-create, output address to stderr, continue |
| Config file missing | Create empty config, then report missing fields |
| Required config field missing | Exit code 10, output JSON prompt to stderr |
| Optional config field missing | Use sensible default or skip |

---

## Predefined Network Profiles

The `payment-config` tool should support predefined network profiles:

```bash
# Set up for Base Sepolia testnet
payment-config use-network base-sepolia

# Set up for Base mainnet
payment-config use-network base-mainnet
```

**Built-in profiles:**

| Profile | Chain ID | RPC URL | Default Token |
|---------|----------|---------|---------------|
| `base-sepolia` | 84532 | https://sepolia.base.org | USDC (0x036CbD53842c5426634e7929541eC2318f3dCF7e) |
| `base-mainnet` | 8453 | https://mainnet.base.org | USDC (0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913) |
| `ethereum-sepolia` | 11155111 | https://rpc.sepolia.org | - |
| `ethereum-mainnet` | 1 | https://eth.llamarpc.com | USDC (0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48) |

## Implementation Order

1. **payment-common** - Shared library with config and wallet utilities
2. **payment-config** - Configuration management; needed to set up environment
3. **create-wallet** - Wallet generation (uses payment-common)
4. **get-address** - Simple, validates wallet format
5. **pay** - Core payment logic
6. **x402curl** - Integrates everything

## Exit Codes

All tools use consistent exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Insufficient balance |
| 2 | Transaction failed |
| 3 | Network error |
| 10 | Missing required configuration |
| 11 | Invalid configuration |
| 12 | Wallet not found (when auto-create disabled) |
| 20 | Invalid arguments |

## Testing Strategy

- Unit tests for each crate
- Integration tests using local Ethereum node (Anvil/Hardhat)
- End-to-end test with mock 402 server

## Open Questions

1. Should wallet password be cached in memory/keyring during a session?
2. Support for hardware wallets (Ledger/Trezor)?
3. Multi-wallet support?
4. Should x402curl support other payment protocols beyond x402?
5. What token standards beyond ERC-20 (ERC-721 for NFT payments)?
6. Should auto-generated wallet passwords use OS keyring instead of plaintext file?
7. Support for custom network profiles defined by user?
