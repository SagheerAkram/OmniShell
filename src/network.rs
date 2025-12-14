use colored::Colorize;
use crate::error::Result;
use crate::config::Config;
use crate::identity::get_public_key;
use crate::storage::Storage;
use crate::ui::output;

pub async fn show_status() -> Result<()> {
    let config = Config::load()?;
    let public_key = get_public_key()?;
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    OMNISHELL STATUS                            ║".cyan());
    println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());
    
    // Identity section
    println!("{}", "║ Identity                                                       ║".cyan());
    println!("{}", format!("║  ├─ Public Key: omni:{}...                      ║", 
        &hex::encode(public_key.to_bytes())[..20]).cyan());
    println!("{}", format!("║  ├─ Fingerprint: {:<44}║", public_key.fingerprint()).cyan());
    
    if let Some(username) = config.identity.username {
        println!("{}", format!("║  ├─ Username: @{:<47}║", username).cyan());
    }
    
    println!("{}", format!("║  └─ Device: {:<51}║", config.identity.device_name).cyan());
    
    println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());
    
    // Network Status
    println!("{}", "║ Network Status                                                 ║".cyan());
    
    let protocols = vec![
        ("Internet P2P", config.protocols.tor_enabled, "simulated", "45ms"),
        ("Tor", config.protocols.tor_enabled, if config.protocols.tor_enabled { "ready" } else { "disabled" }, "85ms"),
        ("I2P", config.protocols.i2p_enabled, if config.protocols.i2p_enabled { "ready" } else { "disabled" }, "120ms"),
        ("LoRa Mesh", config.protocols.lora_enabled, "simulated", "variable"),
        ("Bluetooth", config.protocols.bluetooth_enabled, if config.protocols.bluetooth_enabled { "simulated" } else { "disabled" }, "fast"),
        ("Satellite", config.protocols.satellite_enabled, if config.protocols.satellite_enabled { "simulated" } else { "disabled" }, "high"),
    ];
    
    for (name, enabled, status, latency) in protocols {
        let icon = if enabled && status != "disabled" { "✓" } else { "✗" };
        let color = if enabled && status != "disabled" { icon.green() } else { icon.red() };
        
        println!("{}", format!("║  {}─ {:<15} {:<12} ({:<16}║", 
            color, name, status, format!("{})", latency)).cyan());
    }
    
    if config.network.enable_relay {
        println!("{}", "║  ✓─ Mesh Relay: Enabled                                        ║".green());
    }
    
    println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());
    
    // Activity
    println!("{}", "║ Activity                                                       ║".cyan());
    
    // Count contacts
    let contact_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts")
        .fetch_one(pool)
        .await?;
    
    // Count unread messages
    let unread_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE direction = 'received' AND status != 'read'"
    )
    .fetch_one(pool)
    .await?;
    
    // Count total messages
    let total_messages: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages")
        .fetch_one(pool)
        .await?;
    
    // Count groups
    let group_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM groups")
        .fetch_one(pool)
        .await?;
    
    println!("{}", format!("║  ├─ Contacts: {:<52}║", contact_count.0).cyan());
    println!("{}", format!("║  ├─ Groups: {:<54}║", group_count.0).cyan());
    
    if unread_count.0 > 0 {
        println!("{}", format!("║  ├─ Unread: {:<54}║", unread_count.0).yellow());
    } else {
        println!("{}", format!("║  ├─ Unread: {:<54}║", unread_count.0).cyan());
    }
    
    println!("{}", format!("║  └─ Total Messages: {:<44}║", total_messages.0).cyan());
    
    println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());
    
    // Security
    println!("{}", "║ Security                                                       ║".cyan());
    println!("{}", format!("║  ├─ Encryption: {:<47}║", 
        config.encryption.default_cipher.to_uppercase()).cyan());
    
    let pfs_status = if config.encryption.perfect_forward_secrecy { "✓ Enabled" } else { "Disabled" };
    let pfs_color = if config.encryption.perfect_forward_secrecy { 
        format!("║  ├─ Perfect Forward Secrecy: {:<34}║", pfs_status).green()
    } else {
        format!("║  ├─ Perfect Forward Secrecy: {:<34}║", pfs_status).yellow()
    };
    println!("{}", pfs_color);
    
    println!("{}", format!("║  ├─ Key Rotation: Every {:<38}║", 
        config.encryption.key_rotation_interval).cyan());
    
    let security_features = vec![
        ("Master Password", config.security.master_password),
        ("2FA", config.security.two_factor),
        ("Anti-Forensics", config.security.anti_forensics),
        ("Honeypot Mode", config.security.honeypot_mode),
        ("Canary Tokens", config.security.canary_tokens),
    ];
    
    for (feature, enabled) in security_features {
        let status = if enabled { "✓" } else { "✗" };
        let color = if enabled { status.green() } else { status.red() };
        println!("{}", format!("║  {}─ {:<58}║", color, feature).cyan());
    }
    
    println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());
    
    // Storage
    println!("{}", "║ Storage                                                        ║".cyan());
    
    // Get database size
    let omnishell_dir = crate::storage::omnishell_dir()?;
    let db_path = omnishell_dir.join("omnishell.db");
    let db_size = if db_path.exists() {
        std::fs::metadata(&db_path)?.len()
    } else {
        0
    };
    
    println!("{}", format!("║  ├─ Database: {:<49}║", 
        output::format_bytes(db_size)).cyan());
    println!("{}", format!("║  ├─ Retention: {} days                                   ║", 
        config.messages.retention_days).cyan());
    println!("{}", format!("║  └─ Auto Backup: {:<46}║", 
        if config.backup.auto_backup { "Enabled" } else { "Disabled" }).cyan());
    
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    Ok(())
}
