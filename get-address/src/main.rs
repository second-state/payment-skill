use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;
use x402_common::{Config, Wallet};

/// Get the public Ethereum address from an x402 wallet
///
/// Reads the wallet keystore file and outputs the public address.
/// Does NOT require the wallet password as only the address field is read.
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

fn main() -> ExitCode {
    let args = Args::parse();

    // Load config to get default wallet path if not specified
    let wallet_path = if let Some(path) = args.wallet {
        Some(path)
    } else {
        // Try to load config for default wallet path
        match Config::load_from(args.config.as_deref()) {
            Ok(config) => Some(config.wallet_path()),
            Err(_) => None, // Use library default
        }
    };

    match Wallet::get_address(wallet_path.as_deref()) {
        Ok(address) => {
            println!("{}", address);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(e.exit_code() as u8)
        }
    }
}
