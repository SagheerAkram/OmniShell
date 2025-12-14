use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{OmniShellError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub identity: IdentityConfig,
    pub encryption: EncryptionConfig,
    pub protocols: ProtocolsConfig,
    pub security: SecurityConfig,
    pub privacy: PrivacyConfig,
    pub network: NetworkConfig,
    pub messages: MessagesConfig,
    pub notifications: NotificationsConfig,
    pub backup: BackupConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    pub username: Option<String>,
    pub device_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub default_cipher: String,
    pub perfect_forward_secrecy: bool,
    pub key_rotation_interval: String,
    pub compression: bool,
    pub compression_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolsConfig {
    pub default: String,
    pub auto_fallback: bool,
    pub tor_enabled: bool,
    pub i2p_enabled: bool,
    pub lora_enabled: bool,
    pub bluetooth_enabled: bool,
    pub sms_gateway: Option<String>,
    pub satellite_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub master_password: bool,
    pub two_factor: bool,
    pub screenshot_detection: bool,
    pub anti_forensics: bool,
    pub honeypot_mode: bool,
    pub duress_password: Option<String>,
    pub geofencing: bool,
    pub canary_tokens: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub typing_indicators: bool,
    pub read_receipts: bool,
    pub last_seen: bool,
    pub profile_photo: bool,
    pub location_sharing: bool,
    pub metadata_minimization: bool,
    pub onion_routing_hops: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enable_relay: bool,
    pub max_relay_bandwidth: String,
    pub relay_reputation_threshold: f64,
    pub ipfs_enabled: bool,
    pub dht_enabled: bool,
    pub mesh_network: bool,
    pub auto_announce: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesConfig {
    pub retention_days: u32,
    pub auto_delete_read: bool,
    pub queue_max_size: String,
    pub queue_retry_interval: String,
    pub max_message_size: String,
    pub attachment_storage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsConfig {
    pub enabled: bool,
    pub sound: bool,
    pub vibrate: bool,
    pub desktop_notifications: bool,
    pub keyword_alerts: Vec<String>,
    pub dnd_schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub auto_backup: bool,
    pub backup_schedule: String,
    pub backup_location: PathBuf,
    pub backup_encryption: bool,
    pub backup_retention: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub color_scheme: String,
    pub show_encryption_details: bool,
    pub show_protocol_info: bool,
    pub show_routing_path: bool,
    pub animate_encryption: bool,
    pub show_progress_bars: bool,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let omnishell_dir = home_dir.join(".omnishell");
        
        Self {
            identity: IdentityConfig {
                username: None,
                device_name: hostname::get()
                    .ok()
                    .and_then(|h| h.into_string().ok())
                    .unwrap_or_else(|| "unknown".to_string()),
            },
            encryption: EncryptionConfig {
                default_cipher: "aes256-gcm".to_string(),
                perfect_forward_secrecy: true,
                key_rotation_interval: "7d".to_string(),
                compression: true,
                compression_algorithm: "zstd".to_string(),
            },
            protocols: ProtocolsConfig {
                default: "auto".to_string(),
                auto_fallback: true,
                tor_enabled: true,
                i2p_enabled: false,
                lora_enabled: false,
                bluetooth_enabled: true,
                sms_gateway: None,
                satellite_enabled: false,
            },
            security: SecurityConfig {
                master_password: false,
                two_factor: false,
                screenshot_detection: true,
                anti_forensics: true,
                honeypot_mode: false,
                duress_password: None,
                geofencing: false,
                canary_tokens: true,
            },
            privacy: PrivacyConfig {
                typing_indicators: false,
                read_receipts: false,
                last_seen: false,
                profile_photo: false,
                location_sharing: false,
                metadata_minimization: true,
                onion_routing_hops: 3,
            },
            network: NetworkConfig {
                enable_relay: false,
                max_relay_bandwidth: "1MB/s".to_string(),
                relay_reputation_threshold: 0.8,
                ipfs_enabled: false,
                dht_enabled: true,
                mesh_network: true,
                auto_announce: true,
            },
            messages: MessagesConfig {
                retention_days: 365,
                auto_delete_read: false,
                queue_max_size: "100MB".to_string(),
                queue_retry_interval: "5m".to_string(),
                max_message_size: "10MB".to_string(),
                attachment_storage: "local".to_string(),
            },
            notifications: NotificationsConfig {
                enabled: true,
                sound: true,
                vibrate: false,
                desktop_notifications: true,
                keyword_alerts: vec![],
                dnd_schedule: Some("22:00-07:00".to_string()),
            },
            backup: BackupConfig {
                auto_backup: true,
                backup_schedule: "daily".to_string(),
                backup_location: omnishell_dir.join("backups"),
                backup_encryption: true,
                backup_retention: 30,
            },
            ui: UiConfig {
                color_scheme: "matrix".to_string(),
                show_encryption_details: true,
                show_protocol_info: true,
                show_routing_path: true,
                animate_encryption: true,
                show_progress_bars: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)
            .map_err(|e| OmniShellError::Config(e.to_string()))?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| OmniShellError::Config("Could not find home directory".to_string()))?;
        Ok(home_dir.join(".omnishell").join("config.toml"))
    }
}

pub async fn set(key: &str, value: &str) -> Result<()> {
    let mut config = Config::load()?;
    
    // Simple key-value setting (can be expanded)
    match key {
        "default_protocol" => config.protocols.default = value.to_string(),
        "encryption" => config.encryption.default_cipher = value.to_string(),
        "auto_relay" => config.network.enable_relay = value.parse().unwrap_or(false),
        _ => return Err(OmniShellError::Config(format!("Unknown config key: {}", key))),
    }
    
    config.save()?;
    println!("✓ Configuration updated: {} = {}", key, value);
    Ok(())
}

pub async fn get(key: &str) -> Result<()> {
    let config = Config::load()?;
    
    let value = match key {
        "default_protocol" => config.protocols.default,
        "encryption" => config.encryption.default_cipher,
        "auto_relay" => config.network.enable_relay.to_string(),
        _ => return Err(OmniShellError::Config(format!("Unknown config key: {}", key))),
    };
    
    println!("{} = {}", key, value);
    Ok(())
}

pub async fn interactive_config() -> Result<()> {
    use colored::Colorize;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  OMNISHELL CONFIGURATION                       ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    
    let config = Config::load()?;
    
    println!("\n{}", "Identity:".bold());
    println!("  Device: {}", config.identity.device_name);
    println!("  Username: {}", config.identity.username.unwrap_or_else(|| "Not set".to_string()));
    
    println!("\n{}", "Encryption:".bold());
    println!("  Cipher: {}", config.encryption.default_cipher);
    println!("  Perfect Forward Secrecy: {}", if config.encryption.perfect_forward_secrecy { "✓" } else { "✗" });
    
    println!("\n{}", "Protocols:".bold());
    println!("  Default: {}", config.protocols.default);
    println!("  Tor: {}", if config.protocols.tor_enabled { "✓" } else { "✗" });
    println!("  I2P: {}", if config.protocols.i2p_enabled { "✓" } else { "✗" });
    
    println!("\n{}", "Security:".bold());
    println!("  Master Password: {}", if config.security.master_password { "✓" } else { "✗" });
    println!("  2FA: {}", if config.security.two_factor { "✓" } else { "✗" });
    println!("  Anti-Forensics: {}", if config.security.anti_forensics { "✓" } else { "✗" });
    
    println!("\nUse 'omnishell config set <key> <value>' to modify settings");
    
    Ok(())
}
