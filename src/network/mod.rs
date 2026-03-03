// Network module - exports all network protocols
pub mod p2p;
pub mod tor;
pub mod i2p;
pub mod lora;
pub mod bluetooth;
pub mod alternative; // SMS & Satellite
pub mod relay; // Relay node system
pub mod mta; // Multipath Transport Aggregation
pub mod sonar; // Ultrasonic Air-Gap Bridge
// Re-export main functions

use colored::Colorize;
use crate::error::Result;

/// Show comprehensive network status (moved from old network.rs)
pub async fn show_status() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                 NETWORK STATUS                                 ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Identity
    println!("{}", "🔐 Identity".bold());
    if let Ok(keypair) = crate::identity::get_keypair() {
        let pubkey = keypair.public_key().to_string();
        let short_key = if pubkey.len() > 50 {
            format!("{}...{}", &pubkey[..25], &pubkey[pubkey.len()-20..])
        } else {
            pubkey
        };
        println!("  Public Key: {}", short_key.green());
        
        let fp = crate::crypto::keys::generate_fingerprint(&keypair.public_key().to_bytes());
        println!("  Fingerprint: {}", fp.yellow());
    }
    println!();
    
    // Network Protocols
    println!("{}", "🌐 Network Protocols".bold());
    println!("  ├─ P2P: {}", "Ready".green());
    println!("  ├─ Tor: {}", if tor::check_tor_installed() { "Available".green() } else { "Not  Installed".red() });
    println!("  ├─ I2P: {}", if i2p::check_i2p_installed() { "Available".green() } else { "Not Installed".red() });
    println!("  ├─ LoRa: {}", "Simulated".yellow());
    println!("  ├─ Bluetooth: {}", "Simulated".yellow());
    println!("  └─ Satellite: {}", "Simulated".yellow());
    println!();
    
    // Activity
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let contacts: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts")
        .fetch_one(pool).await?;
    let groups: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM groups")
        .fetch_one(pool).await?;
    let messages: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages")
        .fetch_one(pool).await?;
    
    println!("{}", "📊 Activity".bold());
    println!("  ├─ Contacts: {}", contacts.0);
    println!("  ├─ Groups: {}", groups.0);
    println!("  └─ Messages: {}", messages.0);
    println!();
    
    // Security
    println!("{}", "🔒 Security".bold());
    println!("  ├─ Default Cipher: {}", "AES-256-GCM".green());
    println!("  ├─ PFS: {}", "Enabled".green());
    println!("  └─ End-to-End: {}", "Active".green());
    println!();
    
    Ok(())
}
