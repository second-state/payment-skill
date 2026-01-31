use crate::error::{Error, Result};
use crate::{default_password_path, default_wallet_path, ensure_data_dir};
use alloy::signers::local::PrivateKeySigner;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Wallet information (public only - never contains private key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub path: PathBuf,
}

/// Encrypted wallet storage format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedWallet {
    pub address: String,
    pub crypto: serde_json::Value,
    pub id: String,
    pub version: u32,
}

/// Wallet operations
pub struct Wallet;

impl Wallet {
    /// Generate a secure random password
    pub fn generate_password() -> String {
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..62);
                match idx {
                    0..=9 => (b'0' + idx) as char,
                    10..=35 => (b'a' + idx - 10) as char,
                    _ => (b'A' + idx - 36) as char,
                }
            })
            .collect();
        chars.into_iter().collect()
    }

    /// Create a new wallet with the given password
    /// Returns the wallet info (address and path)
    pub fn create(
        password: Option<&str>,
        output_path: Option<&Path>,
        password_file: Option<&Path>,
    ) -> Result<WalletInfo> {
        ensure_data_dir()?;

        let wallet_path = output_path
            .map(PathBuf::from)
            .unwrap_or_else(default_wallet_path);

        // Check if wallet already exists
        if wallet_path.exists() {
            return Err(Error::WalletExists(wallet_path.display().to_string()));
        }

        // Generate or use provided password
        let (password_str, should_save_password) = match password {
            Some(p) => (p.to_string(), false),
            None => (Self::generate_password(), true),
        };

        // Generate new private key signer
        let signer = PrivateKeySigner::random();
        let address = format!("{}", signer.address());

        // Get the private key bytes
        let private_key_bytes = signer.credential().to_bytes();

        // Create encrypted keystore using eth-keystore
        let mut rng = rand::thread_rng();
        let keystore_dir = wallet_path.parent().unwrap_or(Path::new("."));
        let file_name = wallet_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("wallet.json");

        // Encrypt and save the keystore
        let _uuid = eth_keystore::encrypt_key(
            keystore_dir,
            &mut rng,
            &private_key_bytes,
            &password_str,
            Some(file_name),
        )
        .map_err(|e| Error::Wallet(format!("Failed to encrypt keystore: {}", e)))?;

        // Add address field to keystore (for easier address retrieval without decryption)
        let keystore_content = fs::read_to_string(&wallet_path)?;
        let mut keystore: serde_json::Value = serde_json::from_str(&keystore_content)?;
        // Store address without 0x prefix (standard keystore format)
        keystore["address"] = serde_json::Value::String(address[2..].to_lowercase());
        fs::write(&wallet_path, serde_json::to_string_pretty(&keystore)?)?;

        // Set restrictive permissions on the wallet file
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&wallet_path, fs::Permissions::from_mode(0o600))?;
        }

        // Save password to file if auto-generated
        if should_save_password {
            let pw_path = password_file
                .map(PathBuf::from)
                .unwrap_or_else(default_password_path);
            fs::write(&pw_path, &password_str)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&pw_path, fs::Permissions::from_mode(0o600))?;
            }
        }

        Ok(WalletInfo {
            address,
            path: wallet_path,
        })
    }

    /// Get the address from an existing wallet file (without decrypting)
    pub fn get_address(wallet_path: Option<&Path>) -> Result<String> {
        let path = wallet_path
            .map(PathBuf::from)
            .unwrap_or_else(default_wallet_path);

        if !path.exists() {
            return Err(Error::WalletNotFound(path.display().to_string()));
        }

        let content = fs::read_to_string(&path)?;
        let keystore: serde_json::Value = serde_json::from_str(&content)?;

        // Extract address from keystore (standard Web3 keystore format)
        let address = keystore
            .get("address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Wallet("No address field in keystore".to_string()))?;

        // Ensure 0x prefix and checksum
        let address = if address.starts_with("0x") {
            address.to_string()
        } else {
            format!("0x{}", address)
        };

        Ok(address)
    }

    /// Check if a wallet exists at the given path
    pub fn exists(wallet_path: Option<&Path>) -> bool {
        let path = wallet_path
            .map(PathBuf::from)
            .unwrap_or_else(default_wallet_path);
        path.exists()
    }

    /// Load password from file
    pub fn load_password(password_file: Option<&Path>) -> Result<String> {
        let path = password_file
            .map(PathBuf::from)
            .unwrap_or_else(default_password_path);

        if !path.exists() {
            return Err(Error::Wallet(format!(
                "Password file not found: {}",
                path.display()
            )));
        }

        let password = fs::read_to_string(&path)?.trim().to_string();
        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_password() {
        let password = Wallet::generate_password();
        assert_eq!(password.len(), 32);
        assert!(password.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_create_wallet() {
        let dir = tempdir().unwrap();
        let wallet_path = dir.path().join("test_wallet.json");
        let password_path = dir.path().join("password.txt");

        let info = Wallet::create(
            Some("test_password"),
            Some(&wallet_path),
            Some(&password_path),
        )
        .unwrap();

        assert!(wallet_path.exists());
        assert!(info.address.starts_with("0x"));
        assert_eq!(info.address.len(), 42); // 0x + 40 hex chars
    }

    #[test]
    fn test_wallet_already_exists() {
        let dir = tempdir().unwrap();
        let wallet_path = dir.path().join("test_wallet.json");

        // Create first wallet
        Wallet::create(Some("password"), Some(&wallet_path), None).unwrap();

        // Try to create second wallet at same path
        let result = Wallet::create(Some("password"), Some(&wallet_path), None);
        assert!(matches!(result, Err(Error::WalletExists(_))));
    }

    #[test]
    fn test_get_address() {
        let dir = tempdir().unwrap();
        let wallet_path = dir.path().join("test_wallet.json");

        let info = Wallet::create(Some("password"), Some(&wallet_path), None).unwrap();
        let address = Wallet::get_address(Some(&wallet_path)).unwrap();

        assert_eq!(address.to_lowercase(), info.address.to_lowercase());
    }
}
