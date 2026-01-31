pub mod config;
pub mod error;
pub mod wallet;

pub use config::Config;
pub use error::{Error, Result};
pub use wallet::{Wallet, WalletInfo};

use std::path::PathBuf;

/// Get the default x402 data directory (~/.x402)
pub fn default_data_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".x402")
}

/// Get the default wallet path (~/.x402/wallet.json)
pub fn default_wallet_path() -> PathBuf {
    default_data_dir().join("wallet.json")
}

/// Get the default password file path (~/.x402/password.txt)
pub fn default_password_path() -> PathBuf {
    default_data_dir().join("password.txt")
}

/// Get the default config path (~/.x402/config.toml)
pub fn default_config_path() -> PathBuf {
    default_data_dir().join("config.toml")
}

/// Ensure the data directory exists with proper permissions
pub fn ensure_data_dir() -> Result<PathBuf> {
    let dir = default_data_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700))?;
        }
    }
    Ok(dir)
}
