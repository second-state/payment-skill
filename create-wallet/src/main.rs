use clap::Parser;
use payment_common::{default_config_path, Config, Wallet};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

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

    /// Path to configuration file
    #[arg(long, short = 'c')]
    config: Option<PathBuf>,

    /// Force overwrite if wallet already exists
    #[arg(long, short = 'f')]
    force: bool,
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

fn run(args: Args) -> payment_common::Result<()> {
    // Check if config file exists
    let config_path = args.config.clone().unwrap_or_else(default_config_path);

    if !config_path.exists() {
        eprintln!(
            "Missing the {} file. Please create one using:\n  payment-config use-network base-sepolia",
            config_path.display()
        );
        return Err(payment_common::Error::MissingConfig(
            config_path.display().to_string(),
        ));
    }

    // Load config
    let config = Config::load_from(args.config.as_deref())?;

    // Determine the wallet output path (CLI arg > config > default)
    let wallet_path = args.output.unwrap_or_else(|| config.wallet_path());

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
            return Err(payment_common::Error::WalletExists(
                wallet_path.display().to_string(),
            ));
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
    // Use config's password_file path
    let password_save_path = if password_str.is_none() {
        Some(config.password_path())
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

    if let Some(pw_path) = password_save_path {
        eprintln!("Password saved to: {}", pw_path.display());
        eprintln!("\nIMPORTANT: Keep your password file secure!");
    }

    eprintln!("\nFund this address to enable payments.");

    Ok(())
}
