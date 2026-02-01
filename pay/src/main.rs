use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use x402_common::Config;

// ERC-20 transfer function
sol! {
    #[sol(rpc)]
    contract IERC20 {
        function transfer(address to, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
}

/// Make token payments from an x402 wallet
#[derive(Parser, Debug)]
#[command(name = "pay")]
#[command(about = "Transfer tokens from your x402 wallet")]
#[command(version)]
struct Args {
    /// Recipient Ethereum address
    #[arg(long)]
    to: String,

    /// Amount to transfer (in smallest unit, e.g., wei for ETH, 6 decimals for USDC)
    #[arg(long)]
    amount: String,

    /// ERC-20 token contract address (omit for native ETH/gas token)
    #[arg(long)]
    token: Option<String>,

    /// Ethereum RPC endpoint URL (uses config default if not specified)
    #[arg(long)]
    rpc: Option<String>,

    /// Path to wallet keystore file
    #[arg(long, short = 'w')]
    wallet: Option<PathBuf>,

    /// Wallet password
    #[arg(long)]
    password: Option<String>,

    /// Read wallet password from file
    #[arg(long, conflicts_with = "password")]
    password_file: Option<PathBuf>,

    /// Chain ID (auto-detected from RPC if not specified)
    #[arg(long)]
    chain_id: Option<u64>,

    /// Gas price in Gwei (auto-detected from network if not specified)
    #[arg(long)]
    gas_price: Option<f64>,

    /// Path to configuration file
    #[arg(long, short = 'c')]
    config: Option<PathBuf>,

    /// Don't wait for transaction confirmation
    #[arg(long)]
    no_wait: bool,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    match run(args).await {
        Ok(tx_hash) => {
            println!("{}", tx_hash);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // Map error to exit code
            let code = match &e {
                PayError::InsufficientBalance(_) => 1,
                PayError::TransactionFailed(_) => 2,
                PayError::NetworkError(_) => 3,
                PayError::MissingConfig(_) => 10,
                PayError::InvalidConfig(_) => 11,
                PayError::WalletNotFound(_) => 12,
                PayError::InvalidArgument(_) => 20,
                _ => 1,
            };
            ExitCode::from(code)
        }
    }
}

#[derive(Debug)]
enum PayError {
    InsufficientBalance(String),
    TransactionFailed(String),
    NetworkError(String),
    MissingConfig(String),
    InvalidConfig(String),
    WalletNotFound(String),
    InvalidArgument(String),
    Other(String),
}

impl std::fmt::Display for PayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PayError::InsufficientBalance(msg) => write!(f, "Insufficient balance: {}", msg),
            PayError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            PayError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PayError::MissingConfig(msg) => write!(f, "Missing configuration: {}", msg),
            PayError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            PayError::WalletNotFound(msg) => write!(f, "Wallet not found: {}", msg),
            PayError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            PayError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

async fn run(args: Args) -> Result<String, PayError> {
    // Load config
    let config =
        Config::load_from(args.config.as_deref()).map_err(|e| PayError::Other(e.to_string()))?;

    // Check network config
    if let Err(prompt) = config.check_network_config() {
        let json =
            serde_json::to_string_pretty(&prompt).map_err(|e| PayError::Other(e.to_string()))?;
        eprintln!("{}", json);
        return Err(PayError::MissingConfig(
            "Network configuration is incomplete. Run: x402-config use-network <network-name>"
                .to_string(),
        ));
    }

    // Get RPC URL (CLI > config)
    let rpc_url = args
        .rpc
        .or(config.network.rpc_url.clone())
        .ok_or_else(|| PayError::MissingConfig("RPC URL not configured".to_string()))?;

    // Get chain ID (CLI > config > auto-detect)
    let chain_id = args.chain_id.or(config.network.chain_id);

    // Get wallet path
    let wallet_path = args.wallet.unwrap_or_else(|| config.wallet_path());
    if !wallet_path.exists() {
        return Err(PayError::WalletNotFound(wallet_path.display().to_string()));
    }

    // Get password
    let password = if let Some(pw) = args.password {
        pw
    } else if let Some(pw_file) = args.password_file {
        fs::read_to_string(&pw_file)
            .map_err(|e| PayError::Other(format!("Failed to read password file: {}", e)))?
            .trim()
            .to_string()
    } else {
        // Try config's password file
        let pw_path = config.password_path();
        if pw_path.exists() {
            fs::read_to_string(&pw_path)
                .map_err(|e| PayError::Other(format!("Failed to read password file: {}", e)))?
                .trim()
                .to_string()
        } else {
            return Err(PayError::InvalidArgument(
                "No password provided. Use --password, --password-file, or configure wallet.password_file".to_string(),
            ));
        }
    };

    // Parse recipient address
    let to_address: Address = args.to.parse().map_err(|_| {
        PayError::InvalidArgument(format!("Invalid recipient address: {}", args.to))
    })?;

    // Get decimals (default to 6 for USDC)
    let decimals = config.payment.default_token_decimals.unwrap_or(6);

    // Parse amount (human-readable) and convert to blockchain units
    let amount: U256 = human_to_raw(&args.amount, decimals).map_err(|e| {
        PayError::InvalidArgument(format!("Invalid amount '{}': {}", args.amount, e))
    })?;

    eprintln!(
        "Amount: {} (raw: {} with {} decimals)",
        args.amount, amount, decimals
    );

    // Get token address (CLI > config default)
    let token_address: Option<Address> =
        if let Some(token) = args.token {
            Some(token.parse().map_err(|_| {
                PayError::InvalidArgument(format!("Invalid token address: {}", token))
            })?)
        } else {
            config
                .payment
                .default_token
                .as_ref()
                .and_then(|t| t.parse().ok())
        };

    // Decrypt wallet
    eprintln!("Decrypting wallet...");
    let keystore_content = fs::read_to_string(&wallet_path)
        .map_err(|e| PayError::Other(format!("Failed to read wallet: {}", e)))?;
    let _keystore: serde_json::Value = serde_json::from_str(&keystore_content)
        .map_err(|e| PayError::Other(format!("Invalid keystore format: {}", e)))?;

    // Get the directory and filename for eth_keystore
    let keystore_dir = wallet_path.parent().unwrap_or(std::path::Path::new("."));
    let keystore_name = wallet_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("wallet.json");

    let private_key = eth_keystore::decrypt_key(keystore_dir.join(keystore_name), &password)
        .map_err(|e| PayError::Other(format!("Failed to decrypt wallet: {}", e)))?;

    let signer: PrivateKeySigner = PrivateKeySigner::from_slice(&private_key)
        .map_err(|e| PayError::Other(format!("Invalid private key: {}", e)))?;

    let from_address = signer.address();
    eprintln!("From: {}", from_address);
    eprintln!("To: {}", to_address);

    // Create provider
    eprintln!("Connecting to {}...", rpc_url);
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().wallet(wallet).connect_http(
        rpc_url
            .parse()
            .map_err(|_| PayError::InvalidConfig(format!("Invalid RPC URL: {}", rpc_url)))?,
    );

    // Verify chain ID if specified
    if let Some(expected_chain_id) = chain_id {
        let actual_chain_id = provider
            .get_chain_id()
            .await
            .map_err(|e| PayError::NetworkError(format!("Failed to get chain ID: {}", e)))?;
        if actual_chain_id != expected_chain_id {
            return Err(PayError::InvalidConfig(format!(
                "Chain ID mismatch: expected {}, got {}",
                expected_chain_id, actual_chain_id
            )));
        }
    }

    // Get gas price (CLI override or fetch from network)
    let gas_price_wei: u128 = if let Some(gwei) = args.gas_price {
        // Convert Gwei to Wei (1 Gwei = 10^9 Wei)
        let wei = (gwei * 1_000_000_000.0) as u128;
        eprintln!("Using gas price: {} Gwei", gwei);
        wei
    } else {
        let price = provider
            .get_gas_price()
            .await
            .map_err(|e| PayError::NetworkError(format!("Failed to get gas price: {}", e)))?;
        eprintln!("Network gas price: {} Gwei", price / 1_000_000_000);
        price
    };

    // Send transaction
    let tx_hash = if let Some(token_addr) = token_address {
        // ERC-20 transfer
        eprintln!("Sending {} tokens to {}...", amount, to_address);

        // Check token balance first
        let token_contract = IERC20::new(token_addr, &provider);
        let balance = token_contract
            .balanceOf(from_address)
            .call()
            .await
            .map_err(|e| PayError::NetworkError(format!("Failed to get token balance: {}", e)))?;

        if balance < amount {
            return Err(PayError::InsufficientBalance(format!(
                "Token balance {} is less than amount {}",
                balance, amount
            )));
        }

        // Send transfer transaction with gas price
        let tx = token_contract
            .transfer(to_address, amount)
            .gas_price(gas_price_wei);
        let pending_tx = tx.send().await.map_err(|e| {
            PayError::TransactionFailed(format!("Failed to send transaction: {}", e))
        })?;

        let tx_hash = *pending_tx.tx_hash();
        eprintln!("Transaction sent: {}", tx_hash);

        if !args.no_wait {
            eprintln!("Waiting for confirmation...");
            let receipt = pending_tx
                .get_receipt()
                .await
                .map_err(|e| PayError::TransactionFailed(format!("Transaction failed: {}", e)))?;

            if !receipt.status() {
                return Err(PayError::TransactionFailed(
                    "Transaction reverted".to_string(),
                ));
            }
            eprintln!(
                "Confirmed in block {}",
                receipt.block_number.unwrap_or_default()
            );
        }

        format!("{}", tx_hash)
    } else {
        // Native ETH transfer
        eprintln!("Sending {} wei to {}...", amount, to_address);

        // Check ETH balance first
        let balance = provider
            .get_balance(from_address)
            .await
            .map_err(|e| PayError::NetworkError(format!("Failed to get balance: {}", e)))?;

        let gas_limit = U256::from(21000); // Standard ETH transfer gas
        let total_cost = amount + (gas_limit * U256::from(gas_price_wei));

        if balance < total_cost {
            return Err(PayError::InsufficientBalance(format!(
                "Balance {} is less than amount + gas ({})",
                balance, total_cost
            )));
        }

        // Build and send transaction with gas price
        let tx = alloy::rpc::types::TransactionRequest::default()
            .with_to(to_address)
            .with_value(amount)
            .with_gas_price(gas_price_wei);

        let pending_tx = provider.send_transaction(tx).await.map_err(|e| {
            PayError::TransactionFailed(format!("Failed to send transaction: {}", e))
        })?;

        let tx_hash = *pending_tx.tx_hash();
        eprintln!("Transaction sent: {}", tx_hash);

        if !args.no_wait {
            eprintln!("Waiting for confirmation...");
            let receipt = pending_tx
                .get_receipt()
                .await
                .map_err(|e| PayError::TransactionFailed(format!("Transaction failed: {}", e)))?;

            if !receipt.status() {
                return Err(PayError::TransactionFailed(
                    "Transaction reverted".to_string(),
                ));
            }
            eprintln!(
                "Confirmed in block {}",
                receipt.block_number.unwrap_or_default()
            );
        }

        format!("{}", tx_hash)
    };

    Ok(tx_hash)
}

/// Convert human-readable amount to raw blockchain units
fn human_to_raw(human: &str, decimals: u8) -> Result<U256, String> {
    let decimals = decimals as usize;

    // Handle both integer and decimal inputs
    let (integer_part, decimal_part) = if let Some(pos) = human.find('.') {
        let (int_str, dec_str) = human.split_at(pos);
        (int_str.to_string(), dec_str[1..].to_string()) // Skip the '.'
    } else {
        (human.to_string(), String::new())
    };

    // Validate parts are numeric
    if !integer_part.chars().all(|c| c.is_ascii_digit()) {
        return Err("Invalid integer part".to_string());
    }
    if !decimal_part.chars().all(|c| c.is_ascii_digit()) {
        return Err("Invalid decimal part".to_string());
    }

    // Pad or truncate decimal part to match decimals
    let decimal_padded = if decimal_part.len() < decimals {
        format!("{:0<width$}", decimal_part, width = decimals)
    } else if decimal_part.len() > decimals {
        // Truncate (could also error here)
        decimal_part[..decimals].to_string()
    } else {
        decimal_part
    };

    // Combine integer and decimal parts
    let raw_str = format!("{}{}", integer_part, decimal_padded);

    // Remove leading zeros but keep at least one digit
    let raw_trimmed = raw_str.trim_start_matches('0');
    let raw_final = if raw_trimmed.is_empty() {
        "0"
    } else {
        raw_trimmed
    };

    raw_final
        .parse::<U256>()
        .map_err(|e| format!("Failed to parse amount: {}", e))
}
