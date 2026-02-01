use clap::{Parser, Subcommand};
use std::process::ExitCode;
use x402_common::config::NETWORK_PROFILES;
use x402_common::Config;

/// Configuration management for x402 tools
#[derive(Parser, Debug)]
#[command(name = "payment-config")]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display all current configuration
    Show,

    /// Get a specific config value
    Get {
        /// Config key (e.g., network.rpc_url)
        key: String,
    },

    /// Set one or more config values
    Set {
        /// Key-value pairs (e.g., network.name base-sepolia network.chain_id 84532)
        #[arg(num_args = 2.., value_names = ["KEY", "VALUE"])]
        pairs: Vec<String>,
    },

    /// Apply a predefined network profile
    UseNetwork {
        /// Network profile name (e.g., base-sepolia, base-mainnet)
        profile: String,
    },

    /// List available network profiles
    ListNetworks,

    /// List all valid config keys
    ListKeys,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(e.exit_code() as u8)
        }
    }
}

fn run(args: Args) -> x402_common::Result<()> {
    match args.command {
        Commands::Show => cmd_show(),
        Commands::Get { key } => cmd_get(&key),
        Commands::Set { pairs } => cmd_set(&pairs),
        Commands::UseNetwork { profile } => cmd_use_network(&profile),
        Commands::ListNetworks => cmd_list_networks(),
        Commands::ListKeys => cmd_list_keys(),
    }
}

fn cmd_show() -> x402_common::Result<()> {
    let config = Config::load()?;

    println!("[wallet]");
    println!("path = \"{}\"", config.wallet.path);
    println!("password_file = \"{}\"", config.wallet.password_file);
    println!();

    println!("[network]");
    if let Some(name) = &config.network.name {
        println!("name = \"{}\"", name);
    }
    if let Some(chain_id) = config.network.chain_id {
        println!("chain_id = {}", chain_id);
    }
    if let Some(rpc_url) = &config.network.rpc_url {
        println!("rpc_url = \"{}\"", rpc_url);
    }
    println!();

    println!("[payment]");
    if let Some(token) = &config.payment.default_token {
        println!("default_token = \"{}\"", token);
    }
    if let Some(symbol) = &config.payment.default_token_symbol {
        println!("default_token_symbol = \"{}\"", symbol);
    }
    if let Some(decimals) = config.payment.default_token_decimals {
        println!("default_token_decimals = {}", decimals);
    }
    if let Some(max) = &config.payment.max_auto_payment {
        println!("max_auto_payment = \"{}\"", max);
    }

    Ok(())
}

fn cmd_get(key: &str) -> x402_common::Result<()> {
    let config = Config::load()?;

    match config.get(key) {
        Some(value) => {
            println!("{}", value);
            Ok(())
        }
        None => {
            // Check if it's a valid key that's just not set
            if Config::valid_keys().contains(&key) {
                // Key is valid but not set - output nothing (empty)
                Ok(())
            } else {
                Err(x402_common::Error::Config(format!(
                    "Unknown config key: {}",
                    key
                )))
            }
        }
    }
}

fn cmd_set(pairs: &[String]) -> x402_common::Result<()> {
    if !pairs.len().is_multiple_of(2) {
        return Err(x402_common::Error::InvalidArgument(
            "Arguments must be key-value pairs".to_string(),
        ));
    }

    let mut config = Config::load()?;

    for chunk in pairs.chunks(2) {
        let key = &chunk[0];
        let value = &chunk[1];
        config.set(key, value)?;
        eprintln!("Set {} = {}", key, value);
    }

    config.save()?;
    eprintln!("Configuration saved.");

    Ok(())
}

fn cmd_use_network(profile: &str) -> x402_common::Result<()> {
    let mut config = Config::load()?;
    config.apply_network_profile(profile)?;
    config.save()?;

    eprintln!("Applied network profile: {}", profile);
    eprintln!();
    eprintln!("Network configuration:");
    eprintln!("  name = {}", config.network.name.as_deref().unwrap_or(""));
    eprintln!(
        "  chain_id = {}",
        config
            .network
            .chain_id
            .map(|v| v.to_string())
            .unwrap_or_default()
    );
    eprintln!(
        "  rpc_url = {}",
        config.network.rpc_url.as_deref().unwrap_or("")
    );

    if config.payment.default_token.is_some() {
        eprintln!();
        eprintln!("Payment defaults:");
        eprintln!(
            "  token = {} ({})",
            config.payment.default_token.as_deref().unwrap_or(""),
            config.payment.default_token_symbol.as_deref().unwrap_or("")
        );
        eprintln!(
            "  decimals = {}",
            config
                .payment
                .default_token_decimals
                .map(|v| v.to_string())
                .unwrap_or_default()
        );
    }

    Ok(())
}

fn cmd_list_networks() -> x402_common::Result<()> {
    println!("Available network profiles:");
    println!();
    for profile in NETWORK_PROFILES {
        println!(
            "  {:<20} chain_id={:<10} {}",
            profile.name, profile.chain_id, profile.rpc_url
        );
        if let Some(token) = profile.default_token {
            println!(
                "  {:<20} default_token={} ({})",
                "",
                token,
                profile.default_token_symbol.unwrap_or("")
            );
        }
    }
    println!();
    println!("Usage: payment-config use-network <profile-name>");

    Ok(())
}

fn cmd_list_keys() -> x402_common::Result<()> {
    println!("Valid configuration keys:");
    println!();
    for key in Config::valid_keys() {
        println!("  {}", key);
    }
    println!();
    println!("Usage: payment-config get <key>");
    println!("       payment-config set <key> <value> [<key> <value> ...]");

    Ok(())
}
