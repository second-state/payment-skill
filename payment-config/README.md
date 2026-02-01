# payment-config

Configuration management tool for x402 payment tools.

## Usage

```bash
payment-config <COMMAND>
```

### Commands

| Command | Description |
|---------|-------------|
| `show` | Display all current configuration |
| `get <KEY>` | Get a specific config value |
| `set <KEY> <VALUE> [...]` | Set one or more config values |
| `use-network <PROFILE>` | Apply a predefined network profile |
| `list-networks` | List available network profiles |
| `list-keys` | List all valid config keys |

## Examples

```bash
# Display all configuration
payment-config show

# Apply a network profile (recommended for quick setup)
payment-config use-network base-sepolia

# Get a specific value
payment-config get network.rpc_url

# Set individual values
payment-config set network.name "base-sepolia" network.chain_id 84532

# List available networks
payment-config list-networks

# List all valid config keys
payment-config list-keys
```

## Network Profiles

| Profile | Chain ID | RPC URL | Default Token |
|---------|----------|---------|---------------|
| `base-sepolia` | 84532 | https://sepolia.base.org | USDC |
| `base-mainnet` | 8453 | https://mainnet.base.org | USDC |
| `ethereum-sepolia` | 11155111 | https://rpc.sepolia.org | - |
| `ethereum-mainnet` | 1 | https://eth.llamarpc.com | USDC |

## Configuration Keys

| Key | Description |
|-----|-------------|
| `wallet.path` | Path to wallet keystore file |
| `wallet.password_file` | Path to wallet password file |
| `network.name` | Network name (e.g., "base-sepolia") |
| `network.chain_id` | EIP-155 chain ID |
| `network.rpc_url` | JSON-RPC endpoint URL |
| `payment.default_token` | Default ERC-20 token address |
| `payment.default_token_symbol` | Token symbol (e.g., "USDC") |
| `payment.default_token_decimals` | Token decimals |
| `payment.max_auto_payment` | Maximum auto-approved payment |

## Configuration File

Configuration is stored in `~/.payment/config.toml`:

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

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 11 | Invalid configuration |
| 20 | Invalid arguments |
