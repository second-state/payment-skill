# create-wallet

A command-line tool to create Ethereum-compatible wallets for x402 payments.

## Overview

`create-wallet` generates a new Ethereum wallet with a secure random private key, encrypts it using the Web3 Secret Storage standard (keystore format), and saves it to disk. The tool is designed to be used by AI agents for x402 payment workflows.

## Installation

The binary is distributed as part of the x402 skill package. See the main [install.md](../install.md) for installation instructions.

## Usage

```bash
create-wallet [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--password <PASSWORD>` | Password to encrypt the wallet. If not provided, a secure 32-character password is auto-generated. |
| `--password-file <PATH>` | Read the password from a file instead of command line. |
| `-o, --output <PATH>` | Output path for the wallet keystore file. Default: `~/.payment/wallet.json` |
| `-f, --force` | Force overwrite if a wallet already exists at the output path. |
| `-h, --help` | Print help information. |
| `-V, --version` | Print version information. |

### Examples

#### Create a wallet with auto-generated password

```bash
create-wallet
```

Output:
```
0x742d35Cc6634C0532925a3b844Bc9e7595f...
Wallet created successfully!
Keystore: /home/user/.payment/wallet.json
Password saved to: /home/user/.payment/password.txt

IMPORTANT: Keep your password file secure!

Fund this address to enable payments.
```

#### Create a wallet with a specific password

```bash
create-wallet --password "my-secure-password-123"
```

#### Create a wallet using password from file

```bash
echo "my-secure-password" > /tmp/password.txt
create-wallet --password-file /tmp/password.txt
```

#### Create a wallet at a custom location

```bash
create-wallet --output /path/to/my-wallet.json
```

#### Overwrite an existing wallet

```bash
create-wallet --force
```

## Output

The tool outputs:
- **stdout**: The wallet's public Ethereum address (checksummed, 0x-prefixed)
- **stderr**: Status messages and file locations

This separation allows scripts to easily capture just the address:

```bash
ADDRESS=$(create-wallet 2>/dev/null)
echo "New wallet address: $ADDRESS"
```

## Security

### What the tool does:
- Generates a cryptographically secure random private key using the system's CSPRNG
- Encrypts the private key using scrypt key derivation and AES-128-CTR
- Sets file permissions to `600` (owner read/write only) on Unix systems
- Stores the address in the keystore for easy retrieval without decryption

### What the tool NEVER does:
- Print or log the private key
- Store the private key in plaintext
- Transmit any data over the network

### Password handling:
- If no password is provided, a secure 32-character alphanumeric password is generated
- Auto-generated passwords are saved to `~/.payment/password.txt` with `600` permissions
- When using `--password`, the password is NOT saved to disk

## File Format

The wallet is stored in the standard Web3 Secret Storage (keystore v3) format:

```json
{
  "address": "742d35cc6634c0532925a3b844bc9e7595f...",
  "crypto": {
    "cipher": "aes-128-ctr",
    "cipherparams": { "iv": "..." },
    "ciphertext": "...",
    "kdf": "scrypt",
    "kdfparams": {
      "dklen": 32,
      "n": 8192,
      "p": 1,
      "r": 8,
      "salt": "..."
    },
    "mac": "..."
  },
  "id": "uuid",
  "version": 3
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Wallet already exists (use `--force` to overwrite) |
| 1 | Other errors (IO, encryption, etc.) |

## Integration with Other Tools

After creating a wallet, use the other x402 tools:

```bash
# Get the wallet address
get-address

# Make a payment
pay --to 0xRecipient... --amount 1000000

# Make HTTP requests with automatic 402 handling
x402curl https://api.example.com/paid-endpoint
```

## Troubleshooting

### "Wallet already exists"

A wallet already exists at the default or specified location. Either:
- Use a different output path with `--output`
- Use `--force` to overwrite (WARNING: this will delete the existing wallet)

### "Permission denied"

Ensure you have write permissions to the output directory. The default directory `~/.payment/` is created automatically with proper permissions.

### Password file issues

- Ensure the password file exists and is readable
- The password is trimmed of leading/trailing whitespace
- Avoid newlines in the password file (use `echo -n` or `printf`)
