# get-address

Retrieve the public Ethereum address and token balance from a payment wallet.

## Usage

```bash
get-address [OPTIONS]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--wallet <PATH>` | `-w` | Path to wallet keystore file (default: `~/.payment/wallet.json`) |
| `--config <PATH>` | `-c` | Path to configuration file |
| `--help` | `-h` | Print help information |
| `--version` | `-V` | Print version |

## Examples

```bash
# Get address and balance from default wallet location
get-address

# Get address from specific wallet file
get-address --wallet /path/to/wallet.json
get-address -w /path/to/wallet.json
```

## Output

Outputs JSON with the wallet address and token balance (if network is configured):

```json
{
  "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f12345",
  "balance": "1000000",
  "token": "0x036CbD53842c5426634e7929541eC2318f3dCF7e",
  "token_symbol": "USDC",
  "network": "base-sepolia"
}
```

### Output Fields

| Field | Description |
|-------|-------------|
| `address` | The wallet's public Ethereum address (always present) |
| `balance` | Token balance in smallest units (present if network configured) |
| `token` | ERC-20 token contract address (present if configured) |
| `token_symbol` | Token symbol, e.g., "USDC" (present if configured) |
| `network` | Network name, e.g., "base-sepolia" (present if configured) |

### Without Network Configuration

If no network is configured, the output only includes the address:

```json
{
  "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f12345"
}
```

## Security

- This tool reads ONLY the address field from the keystore file
- No password is required
- The private key is NEVER accessed or output
- The encrypted keystore content is NOT output

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (wallet not found, network error, etc.) |

## How It Works

1. Reads the wallet keystore file (Web3 Secret Storage format) to get the public address
2. If network and token are configured, queries the blockchain for the token balance
3. Outputs all information as JSON

## Configuration

The tool uses `~/.payment/config.toml` for default settings:

- `wallet.path` - Default wallet file location
- `network.rpc_url` - RPC endpoint for balance queries
- `payment.default_token` - Token contract address for balance queries
- `payment.default_token_symbol` - Token symbol for display
- `network.name` - Network name for display

Run `x402-config use-network base-sepolia` to configure the network.
