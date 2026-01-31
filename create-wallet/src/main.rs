use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use x402_common::{default_password_path, default_wallet_path, Wallet};

/// Create a new Ethereum-compatible wallet for x402 payments
#[derive(Parser, Debug)]
#[command(name = "create-wallet")]
#[command(about = "Create a new Ethereum-compatible wallet for x402 payments")]
#[command(version)]
struct Args {
    /// Password to encrypt the wallet (auto-generated if not provided)
    #[arg(long)]
    password: Option<String>,

    /// Read password from file
    #[arg(long, conflicts_with = "password")]
    password_file: Option<PathBuf>,

    /// Output path for the wallet keystore file
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    /// Force overwrite if wallet already exists
    #[arg(long, short = 'f')]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Determine the wallet output path
    let wallet_path = args.output.clone().unwrap_or_else(default_wallet_path);

    // Check if wallet already exists
    if wallet_path.exists() {
        if args.force {
            fs::remove_file(&wallet_path)?;
            eprintln!("Removed existing wallet at {}", wallet_path.display());
        } else {
            eprintln!(
                "Error: Wallet already exists at {}\nUse --force to overwrite.",
                wallet_path.display()
            );
            std::process::exit(1);
        }
    }

    // Read password from file or use provided password
    let password_from_file: Option<String> = if let Some(ref pw_file) = args.password_file {
        Some(fs::read_to_string(pw_file)?.trim().to_string())
    } else {
        None
    };

    let password_str = args.password.as_deref().or(password_from_file.as_deref());

    // Determine where to save the password file (only used if password is auto-generated)
    let password_save_path = if password_str.is_none() {
        Some(default_password_path())
    } else {
        None
    };

    // Create the wallet
    let info = Wallet::create(
        password_str,
        Some(&wallet_path),
        password_save_path.as_deref(),
    )?;

    // Output the address to stdout (this is the only stdout output)
    println!("{}", info.address);

    // Output additional info to stderr
    eprintln!("Wallet created successfully!");
    eprintln!("Keystore: {}", info.path.display());

    if password_str.is_none() {
        eprintln!(
            "Password saved to: {}",
            password_save_path.unwrap().display()
        );
        eprintln!("\nIMPORTANT: Keep your password file secure!");
    }

    eprintln!("\nFund this address to enable payments.");

    Ok(())
}
