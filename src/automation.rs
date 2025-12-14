// Automation & Filters
use colored::Colorize;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::error::Result;
use crate::storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageFilter {
    pub id: i64,
    pub name: String,
    pub pattern: String,
    pub action: String, // "block", "auto-reply", "forward", "delete"
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledMessage {
    pub id: i64,
    pub recipient: String,
    pub message: String,
    pub send_at: i64,
    pub status: String,
}

/// Create a message filter
pub async fn create_filter(name: String, pattern: String, action: String) -> Result<()> {
    println!("{} Creating message filter '{}'...", "→".cyan(), name);
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query(
        "INSERT INTO filters (name, pattern, action, enabled) VALUES (?, ?, ?, 1)"
    )
    .bind(&name)
    .bind(&pattern)
    .bind(&action)
    .execute(pool)
    .await?;
    
    println!("{} Filter created", "✓".green().bold());
    println!("  └─ Name: {}", name);
    println!("  └─ Pattern: {}", pattern);
    println!("  └─ Action: {}", action);
    println!();
    
    Ok(())
}

/// List all filters
pub async fn list_filters() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  MESSAGE FILTERS                               ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let filters: Vec<(i64, String, String, String, bool)> = sqlx::query_as(
        "SELECT id, name, pattern, action, enabled FROM filters"
    )
    .fetch_all(pool)
    .await?;
    
    if filters.is_empty() {
        println!("{}", "No filters configured".yellow());
        println!();
        return Ok(());
    }
    
    for (id, name, pattern, action, enabled) in filters {
        let status = if enabled { "ENABLED".green() } else { "DISABLED".red() };
        println!("#{} {} [{}]", id, name.bold(), status);
        println!("  Pattern: {}", pattern);
        println!("  Action: {}", action);
        println!();
    }
    
    Ok(())
}

/// Schedule a message for future delivery
pub async fn schedule_message(recipient: String, message: String, delay_minutes: i64) -> Result<()> {
    println!("{} Scheduling message to @{}...", "→".cyan(), recipient);
    
    let send_at = Utc::now().timestamp() + (delay_minutes * 60);
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query(
        "INSERT INTO scheduled_messages (recipient, message, send_at, status) VALUES (?, ?, ?, 'pending')"
    )
    .bind(&recipient)
    .bind(&message)
    .bind(send_at)
    .execute(pool)
    .await?;
    
    println!("{} Message scheduled", "✓".green().bold());
    println!("  └─ Recipient: @{}", recipient);
    println!("  └─ Send in: {} minutes", delay_minutes);
    println!();
    
    Ok(())
}

/// Process scheduled messages
pub async fn process_scheduled() -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let now = Utc::now().timestamp();
    
    let due: Vec<(i64, String, String)> = sqlx::query_as(
        "SELECT id, recipient, message FROM scheduled_messages WHERE send_at <= ? AND status = 'pending'"
    )
    .bind(now)
    .fetch_all(pool)
    .await?;
    
    if due.is_empty() {
        return Ok(());
    }
    
    println!("{} Processing {} scheduled messages...", "→".cyan(), due.len());
    
    for (msg_id, recipient, message) in due {
        // Send message
        match crate::messaging::send_message(
            format!("@{}", recipient),
            message,
            None,
            "normal".to_string(),
            None,
            false,
        ).await {
            Ok(_) => {
                sqlx::query("UPDATE scheduled_messages SET status = 'sent' WHERE id = ?")
                    .bind(msg_id)
                    .execute(pool)
                    .await?;
                println!("{} Sent to @{}", "✓".green(), recipient);
            }
            Err(_) => {
                sqlx::query("UPDATE scheduled_messages SET status = 'failed' WHERE id = ?")
                    .bind(msg_id)
                    .execute(pool)
                    .await?;
            }
        }
    }
    
    println!();
    Ok(())
}

/// Set up auto-reply
pub async fn set_autoreply(message: String, enabled: bool) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query(
        "INSERT OR REPLACE INTO config (key, value) VALUES ('autoreply_message', ?)"
    )
    .bind(&message)
    .execute(pool)
    .await?;
    
    sqlx::query(
        "INSERT OR REPLACE INTO config (key, value) VALUES ('autoreply_enabled', ?)"
    )
    .bind(if enabled { "1" } else { "0" })
    .execute(pool)
    .await?;
    
    if enabled {
        println!("{} Auto-reply enabled", "✓".green().bold());
        println!("  Message: {}", message);
    } else {
        println!("{} Auto-reply disabled", "✓".green());
    }
    println!();
    
    Ok(())
}
