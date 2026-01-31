use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::sol;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;
use std::process::ExitCode;
use x402_common::{Config, Wallet};

// ERC-20 balanceOf function
sol! {
    #[sol(rpc)]
    contract IERC20 {
        function balanceOf(address account) external view returns (uint256);
    }
}

/// Get the public Ethereum address and token balance from a payment wallet
///
/// Reads the wallet keystore file and outputs JSON with the public address
/// and current token balance. Does NOT require the wallet password.
#[derive(Parser, Debug)]
#[command(name = "get-address")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the wallet keystore file
    #[arg(long, short = 'w')]
    wallet: Option<PathBuf>,

    /// Path to configuration file
    #[arg(long, short = 'c')]
    config: Option<PathBuf>,
}

#[derive(Serialize)]
struct WalletInfo {
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<String>,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    match run(args).await {
        Ok(info) => {
            match serde_json::to_string_pretty(&info) {
                Ok(json) => {
                    println!("{}", json);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error serializing wallet info: {}", e);
                    ExitCode::from(1)
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(1)
        }
    }
}

async fn run(args: Args) -> Result<WalletInfo, Box<dyn std::error::Error>> {
    // Load config
    let config = Config::load_from(args.config.as_deref()).unwrap_or_default();

    // Get wallet path
    let wallet_path = args.wallet.unwrap_or_else(|| config.wallet_path());

    // Get address from wallet
    let address = Wallet::get_address(Some(&wallet_path))?;

    // Try to get balance if network is configured
    let (balance, token, token_symbol, network) = if let (Some(rpc_url), Some(token_addr)) =
        (&config.network.rpc_url, &config.payment.default_token)
    {
        match get_token_balance(&address, rpc_url, token_addr).await {
            Ok(bal) => (
                Some(bal),
                Some(token_addr.clone()),
                config.payment.default_token_symbol.clone(),
                config.network.name.clone(),
            ),
            Err(e) => {
                eprintln!("Warning: Could not fetch balance: {}", e);
                (
                    None,
                    Some(token_addr.clone()),
                    config.payment.default_token_symbol.clone(),
                    config.network.name.clone(),
                )
            }
        }
    } else {
        (None, None, None, None)
    };

    Ok(WalletInfo {
        address,
        balance,
        token,
        token_symbol,
        network,
    })
}

async fn get_token_balance(
    address: &str,
    rpc_url: &str,
    token_addr: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let wallet_address: Address = address.parse()?;
    let token_address: Address = token_addr.parse()?;

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    let token_contract = IERC20::new(token_address, &provider);
    let balance = token_contract.balanceOf(wallet_address).call().await?;

    Ok(balance.to_string())
}
