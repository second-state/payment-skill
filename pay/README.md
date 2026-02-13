# pay

A command-line tool to make token payments from an x402 wallet.

## Overview

`pay` transfers ERC-20 tokens or native ETH/gas tokens from your x402 wallet to a recipient address. It reads configuration from `config.toml` (in the skill root directory) and handles wallet decryption, balance checking, and transaction confirmation.

## Installation

The binary is distributed as part of the x402 skill package. See the main [install.md](../install.md) for installation instructions.

## Usage

```bash
pay [OPTIONS] --to <ADDRESS> --amount <AMOUNT>
```

### Options

| Option | Description |
|--------|-------------|
| `--to <ADDRESS>` | Recipient Ethereum address (required) |
| `--amount <AMOUNT>` | Amount to transfer in smallest unit (required) |
| `--token <ADDRESS>` | ERC-20 token contract address (omit for native ETH) |
| `--rpc <URL>` | Ethereum RPC endpoint URL (uses config default) |
| `-w, --wallet <PATH>` | Path to wallet keystore file |
| `--password <PASSWORD>` | Wallet password |
| `--password-file <PATH>` | Read wallet password from file |
| `--chain-id <ID>` | Chain ID (auto-detected from RPC if not specified) |
| `-c, --config <PATH>` | Path to configuration file |
| `--no-wait` | Don't wait for transaction confirmation |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

### Examples

#### Transfer USDC (using config defaults)

```bash
pay --to 0x742d35Cc6634C0532925a3b844Bc9e7595f... --amount 1000000
```

This sends 1 USDC (6 decimals = 1,000,000 smallest units) using the default token from config.

#### Transfer native ETH

```bash
pay --to 0x742d35Cc6634C0532925a3b844Bc9e7595f... --amount 1000000000000000000
```

When no `--token` is specified and no default token is configured, this sends 1 ETH (18 decimals).

#### Transfer a specific ERC-20 token

```bash
pay --to 0x742d35Cc6634C0532925a3b844Bc9e7595f... \
    --amount 1000000 \
    --token 0x036CbD53842c5426634e7929541eC2318f3dCF7e
```

#### Use a custom RPC endpoint

```bash
pay --to 0x742d35... --amount 1000000 --rpc https://my-rpc.example.com
```

#### Don't wait for confirmation

```bash
pay --to 0x742d35... --amount 1000000 --no-wait
```

## Output

The tool outputs:
- **stdout**: The transaction hash (0x-prefixed)
- **stderr**: Status messages (decrypting, connecting, sending, confirming)

This separation allows scripts to easily capture just the transaction hash:

```bash
TX_HASH=$(pay --to 0x742d35... --amount 1000000 2>/dev/null)
echo "Transaction: $TX_HASH"
```

### Sample output

```
Decrypting wallet...
From: 0xYourAddress...
To: 0xRecipient...
Connecting to https://sepolia.base.org...
Sending 1000000 tokens to 0xRecipient...
Transaction sent: 0xabc123...
Waiting for confirmation...
Confirmed in block 12345678
0xabc123def456...
```

## Configuration

The tool reads from `config.toml` (in the skill root directory). Key settings:

| Key | Description |
|-----|-------------|
| `wallet.path` | Default wallet keystore path |
| `wallet.password_file` | Default password file path |
| `network.rpc_url` | JSON-RPC endpoint URL |
| `network.chain_id` | Expected chain ID for verification |
| `payment.default_token` | Default ERC-20 token address |

Run `payment-config show` to see current configuration.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Insufficient balance |
| 2 | Transaction failed or reverted |
| 3 | Network error |
| 10 | Missing configuration |
| 11 | Invalid configuration |
| 12 | Wallet not found |
| 20 | Invalid argument |

## Balance Checking

Before sending a transaction, the tool checks:

- **For ERC-20 transfers**: Token balance >= amount
- **For ETH transfers**: ETH balance >= amount + gas cost

If the balance is insufficient, the tool exits with code 1 without sending a transaction.

## Chain ID Verification

If a chain ID is specified (via `--chain-id` or config), the tool verifies that the RPC endpoint returns the expected chain ID. This prevents accidentally sending transactions to the wrong network.

## Transaction Confirmation

By default, the tool waits for the transaction to be included in a block and verifies that it succeeded (didn't revert). Use `--no-wait` to skip this and return immediately after the transaction is broadcast.

## Troubleshooting

### "Network configuration is incomplete"

Run `payment-config use-network <network-name>` to apply a network profile, or manually set the required fields.

### "Wallet not found"

Ensure you have created a wallet with `create-wallet`, or specify the wallet path with `--wallet`.

### "No password provided"

Provide the password via `--password`, `--password-file`, or configure `wallet.password_file` in the config.

### "Insufficient balance"

Your wallet doesn't have enough tokens/ETH for the transfer. Check your balance and fund the wallet if needed.

### "Chain ID mismatch"

The RPC endpoint is for a different network than expected. Check your `network.rpc_url` and `network.chain_id` settings.

### "Transaction reverted"

The transaction was included in a block but failed. For ERC-20 transfers, this might mean:
- The token contract rejected the transfer
- You don't have approval for the transfer (for some tokens)
- The recipient address is a contract that rejected the transfer

## Integration with Other Tools

```bash
# Create a wallet first
create-wallet

# Configure the network
payment-config use-network base-sepolia

# Fund your wallet (external step)
# ...

# Check your address
get-address

# Make a payment
pay --to 0xRecipient... --amount 1000000
```
