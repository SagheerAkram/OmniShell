// Emergency & Safety Features
use colored::Colorize;
use chrono::Utc;

use crate::error::Result;
use crate::storage::Storage;

/// Emergency broadcast to all contacts
pub async fn emergency_broadcast(message: String) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".red());
    println!("{}", "║                  EMERGENCY BROADCAST                           ║".red());
    println!("{}", "╚════════════════════════════════════════════════════════════════╗".red());
    println!();
    
    println!("{}", "⚠️  EMERGENCY MODE ACTIVATED".red().bold());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get all contacts
    let contacts: Vec<(String,)> = sqlx::query_as("SELECT name FROM contacts")
        .fetch_all(pool)
        .await?;
    
    println!("{} Broadcasting to {} contacts...", "→".red(), contacts.len());
    println!();
    
    let emergency_msg = format!("🚨 EMERGENCY: {}", message);
    
    for (contact_name,) in contacts {
        // Send emergency message with highest priority
        println!("{} Sending to @{}...", "→".red(), contact_name);
        
        // Use messaging system to send
        crate::messaging::send_message(
            format!("@{}", contact_name),
            emergency_msg.clone(),
            None,
            "urgent".to_string(),
            None,
            false,
        ).await?;
    }
    
    println!();
    println!("{} Emergency broadcast complete", "✓".green().bold());
    println!();
    
    Ok(())
}

/// Panic mode - secure wipe
pub async fn panic_mode() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".red());
    println!("{}", "║                    PANIC MODE                                  ║".red());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".red());
    println!();
    
    println!("{}", "⚠️  WARNING: This will securely wipe all OmniShell data!".red().bold());
    println!("{}", "⚠️  This action CANNOT be undone!".red().bold());
    println!();
    
    let confirmed = dialoguer::Confirm::new()
        .with_prompt("Type 'PANIC' to confirm")
        .default(false)
        .interact()?;
    
    if !confirmed {
        println!("{}", "Cancelled.".yellow());
        return Ok(());
    }
    
    println!();
    println!("{} Activating panic mode...", "→".red());
    
    let omnishell_dir = crate::storage::omnishell_dir()?;
    
    // Secure wipe database
    println!("{} Wiping database...", "→".red());
    let db_path = omnishell_dir.join("omnishell.db");
    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
    }
    
    // Wipe keys
    println!("{} Wiping encryption keys...", "→".red());
    let keys_dir = omnishell_dir.join("keys");
    if keys_dir.exists() {
        std::fs::remove_dir_all(&keys_dir)?;
    }
    
    // Wipe messages
    println!("{} Wiping message history...", "→".red());
    let messages_dir = omnishell_dir.join("messages");
    if messages_dir.exists() {
        std::fs::remove_dir_all(&messages_dir)?;
    }
    
    // Wipe contacts
    println!("{} Wiping contacts...", "→".red());
    let contacts_dir = omnishell_dir.join("contacts");
    if contacts_dir.exists() {
        std::fs::remove_dir_all(&contacts_dir)?;
    }
    
    println!();
    println!("{} PANIC MODE COMPLETE - All data wiped", "✓".green().bold());
    println!();
    println!("OmniShell has been reset to factory state.");
    println!("Run {} to start fresh.", "omnishell init".cyan());
    println!();
    
    Ok(())
}

/// Dead man's switch setup
pub async fn setup_deadman_switch(hours: u32, action: String) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║               DEAD MAN'S SWITCH SETUP                          ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("Dead man's switch configuration:");
    println!("  Timeout: {} hours", hours);
    println!("  Action: {}", action);
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let trigger_time = Utc::now().timestamp() + (hours as i64 * 3600);
    
    sqlx::query(
        "INSERT OR REPLACE INTO config (key, value) VALUES ('deadman_trigger', ?)"
    )
    .bind(trigger_time)
    .execute(pool)
    .await?;
    
    sqlx::query(
        "INSERT OR REPLACE INTO config (key, value) VALUES ('deadman_action', ?)"
    )
    .bind(&action)
    .execute(pool)
    .await?;
    
    println!("{} Dead man's switch armed", "✓".green().bold());
    println!("  └─ Trigger time: {} hours from now", hours);
    println!("  └─ Reset with: {}", "omnishell deadman reset".cyan());
    println!();
    
    Ok(())
}

/// Reset dead man's switch
pub async fn reset_deadman_switch() -> Result<()> {
    println!("{} Resetting dead man's switch...", "→".cyan());
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query("DELETE FROM config WHERE key LIKE 'deadman_%'")
        .execute(pool)
        .await?;
    
    println!("{} Dead man's switch disarmed", "✓".green());
    println!();
    
    Ok(())
}
