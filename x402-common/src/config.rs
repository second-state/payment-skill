use crate::error::{Error, Result};
use crate::{default_config_path, default_password_path, default_wallet_path, ensure_data_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub wallet: WalletConfig,
    #[serde(default)]
    pub network: NetworkConfig,
    #[serde(default)]
    pub payment: PaymentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    #[serde(default = "default_wallet_path_string")]
    pub path: String,
    #[serde(default = "default_password_path_string")]
    pub password_file: String,
}

fn default_wallet_path_string() -> String {
    default_wallet_path().display().to_string()
}

fn default_password_path_string() -> String {
    default_password_path().display().to_string()
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            path: default_wallet_path_string(),
            password_file: default_password_path_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub chain_id: Option<u64>,
    #[serde(default)]
    pub rpc_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentConfig {
    #[serde(default)]
    pub default_token: Option<String>,
    #[serde(default)]
    pub default_token_symbol: Option<String>,
    #[serde(default)]
    pub default_token_decimals: Option<u8>,
    #[serde(default)]
    pub max_auto_payment: Option<String>,
}

impl Config {
    /// Load config from the default path or create empty config
    pub fn load() -> Result<Self> {
        Self::load_from(None)
    }

    /// Load config from a specific path
    pub fn load_from(path: Option<&Path>) -> Result<Self> {
        let config_path = path.map(PathBuf::from).unwrap_or_else(default_config_path);

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save config to the default path
    pub fn save(&self) -> Result<()> {
        self.save_to(None)
    }

    /// Save config to a specific path
    pub fn save_to(&self, path: Option<&Path>) -> Result<()> {
        ensure_data_dir()?;

        let config_path = path.map(PathBuf::from).unwrap_or_else(default_config_path);

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&config_path, fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    /// Get the wallet path from config, expanding ~ to home directory
    pub fn wallet_path(&self) -> PathBuf {
        expand_tilde(&self.wallet.path)
    }

    /// Get the password file path from config
    pub fn password_path(&self) -> PathBuf {
        expand_tilde(&self.wallet.password_file)
    }
}

/// Expand ~ to home directory in paths
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    }
    PathBuf::from(path)
}

/// Predefined network profiles
pub struct NetworkProfile {
    pub name: &'static str,
    pub chain_id: u64,
    pub rpc_url: &'static str,
    pub default_token: Option<&'static str>,
    pub default_token_symbol: Option<&'static str>,
    pub default_token_decimals: Option<u8>,
}

pub const NETWORK_PROFILES: &[NetworkProfile] = &[
    NetworkProfile {
        name: "base-sepolia",
        chain_id: 84532,
        rpc_url: "https://sepolia.base.org",
        default_token: Some("0x036CbD53842c5426634e7929541eC2318f3dCF7e"),
        default_token_symbol: Some("USDC"),
        default_token_decimals: Some(6),
    },
    NetworkProfile {
        name: "base-mainnet",
        chain_id: 8453,
        rpc_url: "https://mainnet.base.org",
        default_token: Some("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"),
        default_token_symbol: Some("USDC"),
        default_token_decimals: Some(6),
    },
    NetworkProfile {
        name: "ethereum-sepolia",
        chain_id: 11155111,
        rpc_url: "https://rpc.sepolia.org",
        default_token: None,
        default_token_symbol: None,
        default_token_decimals: None,
    },
    NetworkProfile {
        name: "ethereum-mainnet",
        chain_id: 1,
        rpc_url: "https://eth.llamarpc.com",
        default_token: Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
        default_token_symbol: Some("USDC"),
        default_token_decimals: Some(6),
    },
];

impl Config {
    /// Apply a network profile to the config
    pub fn apply_network_profile(&mut self, profile_name: &str) -> Result<()> {
        let profile = NETWORK_PROFILES
            .iter()
            .find(|p| p.name == profile_name)
            .ok_or_else(|| Error::Config(format!("Unknown network profile: {}", profile_name)))?;

        self.network.name = Some(profile.name.to_string());
        self.network.chain_id = Some(profile.chain_id);
        self.network.rpc_url = Some(profile.rpc_url.to_string());

        if let Some(token) = profile.default_token {
            self.payment.default_token = Some(token.to_string());
        }
        if let Some(symbol) = profile.default_token_symbol {
            self.payment.default_token_symbol = Some(symbol.to_string());
        }
        if let Some(decimals) = profile.default_token_decimals {
            self.payment.default_token_decimals = Some(decimals);
        }

        Ok(())
    }

    /// Get a config value by key path (e.g., "network.rpc_url")
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "wallet.path" => Some(self.wallet.path.clone()),
            "wallet.password_file" => Some(self.wallet.password_file.clone()),
            "network.name" => self.network.name.clone(),
            "network.chain_id" => self.network.chain_id.map(|v| v.to_string()),
            "network.rpc_url" => self.network.rpc_url.clone(),
            "payment.default_token" => self.payment.default_token.clone(),
            "payment.default_token_symbol" => self.payment.default_token_symbol.clone(),
            "payment.default_token_decimals" => {
                self.payment.default_token_decimals.map(|v| v.to_string())
            }
            "payment.max_auto_payment" => self.payment.max_auto_payment.clone(),
            _ => None,
        }
    }

    /// Set a config value by key path (e.g., "network.rpc_url")
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "wallet.path" => self.wallet.path = value.to_string(),
            "wallet.password_file" => self.wallet.password_file = value.to_string(),
            "network.name" => self.network.name = Some(value.to_string()),
            "network.chain_id" => {
                let chain_id = value
                    .parse::<u64>()
                    .map_err(|_| Error::Config(format!("Invalid chain_id: {}", value)))?;
                self.network.chain_id = Some(chain_id);
            }
            "network.rpc_url" => self.network.rpc_url = Some(value.to_string()),
            "payment.default_token" => self.payment.default_token = Some(value.to_string()),
            "payment.default_token_symbol" => {
                self.payment.default_token_symbol = Some(value.to_string())
            }
            "payment.default_token_decimals" => {
                let decimals = value
                    .parse::<u8>()
                    .map_err(|_| Error::Config(format!("Invalid decimals: {}", value)))?;
                self.payment.default_token_decimals = Some(decimals);
            }
            "payment.max_auto_payment" => self.payment.max_auto_payment = Some(value.to_string()),
            _ => return Err(Error::Config(format!("Unknown config key: {}", key))),
        }
        Ok(())
    }

    /// Get list of all valid config keys
    pub fn valid_keys() -> &'static [&'static str] {
        &[
            "wallet.path",
            "wallet.password_file",
            "network.name",
            "network.chain_id",
            "network.rpc_url",
            "payment.default_token",
            "payment.default_token_symbol",
            "payment.default_token_decimals",
            "payment.max_auto_payment",
        ]
    }

    /// Check if network configuration is complete for making payments
    pub fn check_network_config(&self) -> std::result::Result<(), MissingConfigPrompt> {
        let mut missing_fields = Vec::new();

        if self.network.rpc_url.is_none() {
            missing_fields.push("network.rpc_url".to_string());
        }
        if self.network.chain_id.is_none() {
            missing_fields.push("network.chain_id".to_string());
        }

        if missing_fields.is_empty() {
            Ok(())
        } else {
            Err(MissingConfigPrompt {
                error: "missing_config".to_string(),
                missing_fields,
                prompt: "Configuration is incomplete. Please configure the network settings."
                    .to_string(),
                questions: vec![ConfigQuestion {
                    field: "network".to_string(),
                    question: "Which blockchain network should be used for payments?".to_string(),
                    examples: vec![
                        "base-sepolia".to_string(),
                        "base-mainnet".to_string(),
                        "ethereum-mainnet".to_string(),
                    ],
                    default: Some("base-sepolia".to_string()),
                }],
                hint: "Run: payment-config use-network <network-name>".to_string(),
            })
        }
    }
}

/// Structured prompt for missing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingConfigPrompt {
    pub error: String,
    pub missing_fields: Vec<String>,
    pub prompt: String,
    pub questions: Vec<ConfigQuestion>,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigQuestion {
    pub field: String,
    pub question: String,
    pub examples: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.wallet.path.contains(".payment"));
        assert!(config.network.chain_id.is_none());
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        let mut config = Config::default();
        config.network.name = Some("test-network".to_string());
        config.network.chain_id = Some(12345);

        config.save_to(Some(&config_path)).unwrap();
        let loaded = Config::load_from(Some(&config_path)).unwrap();

        assert_eq!(loaded.network.name, Some("test-network".to_string()));
        assert_eq!(loaded.network.chain_id, Some(12345));
    }

    #[test]
    fn test_apply_network_profile() {
        let mut config = Config::default();
        config.apply_network_profile("base-sepolia").unwrap();

        assert_eq!(config.network.name, Some("base-sepolia".to_string()));
        assert_eq!(config.network.chain_id, Some(84532));
        assert_eq!(
            config.network.rpc_url,
            Some("https://sepolia.base.org".to_string())
        );
    }
}
