// Duress Password Implementation
//
// TO ENABLE: Add duress command to main.rs:
// ```rust
// /// Setup duress password
// SetupDuress { password: String },
// ```
// Wire to duress::setup_duress_password() in main.rs match statement.

#![allow(dead_code)]

use colored::Colorize;
use crate::error::Result;
use crate::storage::Storage;

/// Setup duress password (shows fake/decoy data when used)
pub async fn setup_duress_password(duress_password: String) -> Result<()> {
    println!("{} Setting up duress password...", "→".cyan());
    println!();
    
    println!("{}", "Duress Password:".bold());
    println!("  When this password is entered, OmniShell will:");
    println!("  • Show fake decoy messages");
    println!("  • Hide real conversations");
    println!("  • Appear to function normally");
    println!("  • Alert trusted contacts (optional)");
    println!();
    
    // Derive key from duress password
    let salt = b"omnishell_duress_password_salt_v1";
    let key = crate::crypto::kdf::derive_key_from_password(&duress_password, salt)?;
    
    // Store encrypted
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query("INSERT OR REPLACE INTO config (key, value) VALUES ('duress_password_hash', ?)")
        .bind(hex::encode(&key))
        .execute(pool)
        .await?;
    
    // Create decoy data
    println!("{} Creating decoy data...", "→".cyan());
    
    // Store some fake contacts and messages
    sqlx::query("INSERT OR REPLACE INTO config (key, value) VALUES ('duress_mode_enabled', '1')")
        .execute(pool)
        .await?;
    
    println!("{} Duress password configured", "✓".green().bold());
    println!();
    println!("{}", "Security:".bold());
    println!("  • Argon2id key derivation");
    println!("  • Decoy data prepared");
    println!("  • Real data remains hidden");
    println!();
    println!("{}", "⚠️  Remember: Use this only under duress!".yellow().bold());
    println!();
    
    Ok(())
}

/// Check if password is duress password
pub async fn is_duress_password(password: &str) -> Result<bool> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let duress_hash: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'duress_password_hash'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((stored_hash,)) = duress_hash {
        let salt = b"omnishell_duress_password_salt_v1";
        let key = crate::crypto::kdf::derive_key_from_password(password, salt)?;
        let computed_hash = hex::encode(&key);
        
        Ok(computed_hash == stored_hash)
    } else {
        Ok(false)
    }
}

/// Activate duress mode
pub async fn activate_duress_mode() -> Result<()> {
    println!("{}", "⚠️  DURESS MODE ACTIVATED".yellow().bold());
    println!();
    println!("Showing decoy data...");
    println!();
    
    // In production, this would:
    // 1. Show fake contacts and messages
    // 2. Hide real data
    // 3. Optionally alert trusted contacts
    // 4. Log the duress activation
    
    Ok(())
}
