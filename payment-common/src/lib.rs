pub mod config;
pub mod error;
pub mod wallet;

pub use config::Config;
pub use error::{Error, Result};
pub use wallet::{Wallet, WalletInfo};

use std::path::PathBuf;

/// Get the directory containing the current executable binary.
///
/// Given the installed layout where binaries live in `scripts/`, this returns
/// the `scripts/` directory. All default paths are then relative to this
/// (e.g., `../wallet.json`, `../config.toml`).
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Get the default wallet path (../wallet.json relative to the executable)
pub fn default_wallet_path() -> PathBuf {
    exe_dir().join("../wallet.json")
}

/// Get the default password file path (../password.txt relative to the executable)
pub fn default_password_path() -> PathBuf {
    exe_dir().join("../password.txt")
}

/// Get the default config path (../config.toml relative to the executable)
pub fn default_config_path() -> PathBuf {
    exe_dir().join("../config.toml")
}

/// Ensure the parent directory (skill root) exists with proper permissions
pub fn ensure_data_dir() -> Result<PathBuf> {
    let dir = exe_dir().join("..");
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
