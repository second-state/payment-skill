# get-address

Retrieve the public Ethereum address from an x402 wallet.

## Usage

```bash
get-address [OPTIONS]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--wallet <PATH>` | `-w` | Path to wallet keystore file (default: `~/.x402/wallet.json`) |
| `--config <PATH>` | `-c` | Path to configuration file |
| `--help` | `-h` | Print help information |
| `--version` | `-V` | Print version |

## Examples

```bash
# Get address from default wallet location
get-address

# Get address from specific wallet file
get-address --wallet /path/to/wallet.json
get-address -w /path/to/wallet.json
```

## Output

Prints the Ethereum address (0x-prefixed) to stdout:

```
0x742d35Cc6634C0532925a3b844Bc9e7595f12345
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
| 12 | Wallet not found |
| 1 | Other error |

## How It Works

The wallet keystore file (Web3 Secret Storage format) contains an unencrypted `address` field that stores the public address. This tool simply reads that field without requiring decryption of the private key.
