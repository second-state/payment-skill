pub mod config;
pub mod error;
pub mod wallet;

pub use config::Config;
pub use error::{Error, Result};
pub use wallet::{Wallet, WalletInfo};

use std::path::{Path, PathBuf};

/// Resolve the default data directory.
///
/// Resolution order:
/// 1. `PAYMENT_SKILL_DATA_DIR` environment variable (if set)
/// 2. Installed layout: if executable is in `.../scripts/`, use its parent
/// 3. Local development (`cargo run` from `target/{debug,release}`): use current working directory
/// 4. Fallback: executable directory
pub fn default_data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("PAYMENT_SKILL_DATA_DIR") {
        let trimmed = dir.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    std::env::current_exe()
        .ok()
        .and_then(|exe| resolve_data_dir_from_exe(&exe, &cwd))
        .unwrap_or(cwd)
}

fn resolve_data_dir_from_exe(exe: &Path, cwd: &Path) -> Option<PathBuf> {
    let exe_dir = exe.parent()?;

    // Installed skill layout: <skill-root>/scripts/<binary>
    if exe_dir.file_name().and_then(|n| n.to_str()) == Some("scripts") {
        return exe_dir.parent().map(|p| p.to_path_buf());
    }

    // Local cargo layout: <workspace>/target/{debug,release}/<binary>
    let is_target_profile = matches!(
        exe_dir.file_name().and_then(|n| n.to_str()),
        Some("debug" | "release")
    );
    let is_under_target = exe_dir
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        == Some("target");

    if is_target_profile && is_under_target {
        return Some(cwd.to_path_buf());
    }

    Some(exe_dir.to_path_buf())
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
