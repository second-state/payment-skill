pub mod config;
pub mod error;
pub mod wallet;

pub use config::Config;
pub use error::{Error, Result};
pub use wallet::{Wallet, WalletInfo};

use std::path::PathBuf;

/// Get the default data directory (parent of the directory containing the executable).
///
/// Given the installed layout where binaries live in `scripts/`, this returns the
/// skill root directory (e.g., `~/.openclaw/skills/payment/`).
pub fn default_data_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .and_then(|dir| dir.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Get the default wallet path (data_dir/wallet.json)
pub fn default_wallet_path() -> PathBuf {
    default_data_dir().join("wallet.json")
}

/// Get the default password file path (data_dir/password.txt)
pub fn default_password_path() -> PathBuf {
    default_data_dir().join("password.txt")
}

/// Get the default config path (data_dir/config.toml)
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
