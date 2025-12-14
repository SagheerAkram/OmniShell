// Offline queue management
use colored::Colorize;
use sqlx::SqlitePool;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::storage::Storage;
use crate::ui::output;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuedMessage {
    pub id: i64,
    pub recipient: String,
    pub encrypted_content: Vec<u8>,
    pub priority: String,
    pub created_at: i64,
    pub retry_count: i64,
    pub status: String,
}

/// Show message queue
pub async fn show_queue() -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let pending: Vec<(i64, String, i64, String, i64)> = sqlx::query_as(
        "SELECT id, contact_id, created_at, priority, retry_count FROM queue WHERE status = 'pending' ORDER BY priority DESC, created_at ASC"
    )
    .fetch_all(pool)
    .await?;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    MESSAGE QUEUE                               ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    if pending.is_empty() {
        println!("{}", "Queue is empty".green());
        println!();
        return Ok(());
    }
    
    println!("{} pending messages in queue", pending.len());
    println!();
    
    for (id, contact_id, created, priority, retries) in pending {
        let age = Utc::now().timestamp() - created;
        println!("#{} Priority: {} | Age: {}s | Retries: {}", 
            id, 
            priority.yellow(), 
            age, 
            retries
        );
    }
    
    println!();
   println!("Commands:");
    println!("  {} - Process queue", "omnishell queue process".cyan());
    println!("  {} - Clear failed", "omnishell queue clear".cyan());
    println!();
    
    Ok(())
}

/// Process queued messages
pub async fn process_queue() -> Result<()> {
    println!("{} Processing message queue...", "→".cyan());
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get pending messages
    let pending: Vec<(i64,)> = sqlx::query_as(
        "SELECT id FROM queue WHERE status = 'pending' AND retry_count < 5 ORDER BY priority DESC LIMIT 10"
    )
    .fetch_all(pool)
    .await?;
    
    if pending.is_empty() {
        println!("{} No messages to process", "✓".green());
        return Ok(());
    }
    
    let mut sent = 0;
    let mut failed = 0;
    
    for (msg_id,) in pending {
        // Try to send message
        // In real implementation, this would attempt network delivery
        
        // Update retry count
        sqlx::query("UPDATE queue SET retry_count = retry_count + 1, last_retry = ? WHERE id = ?")
            .bind(Utc::now().timestamp())
            .bind(msg_id)
            .execute(pool)
            .await?;
        
        // Simulate: mark as sent or failed
        if rand::random::<bool>() {
            sqlx::query("UPDATE queue SET status = 'sent' WHERE id = ?")
                .bind(msg_id)
                .execute(pool)
                .await?;
            sent += 1;
        } else {
            failed += 1;
        }
    }
    
    println!("{} Queue processed", "✓".green().bold());
    println!("  └─ Sent: {}", sent);
    println!("  └─ Failed: {}", failed);
    println!();
    
    Ok(())
}

/// Clear failed messages from queue
pub async fn clear_queue() -> Result<()> {
    println!("{} Clearing failed messages...", "→".cyan());
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let deleted = sqlx::query("DELETE FROM queue WHERE status = 'failed' OR retry_count >= 5")
        .execute(pool)
        .await?;
    
    println!("{} Cleared {} failed messages", "✓".green(), deleted.rows_affected());
    println!();
    
    Ok(())
}

/// Queue a message for later delivery
pub async fn queue_message(recipient: String, encrypted_content: Vec<u8>, priority: &str) -> Result<i64> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get or create contact ID
    let contact_id: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM contacts WHERE name = ?"
    )
    .bind(&recipient)
    .fetch_optional(pool)
    .await?;
    
    let contact_id = contact_id.map(|(id,)| id).unwrap_or(0);
    
    let now = Utc::now().timestamp();
    
    let result = sqlx::query(
        "INSERT INTO queue (contact_id, encrypted_content, priority, status, retry_count, created_at) VALUES (?, ?, ?, 'pending', 0, ?)"
    )
    .bind(contact_id)
    .bind(&encrypted_content)
    .bind(priority)
    .bind(now)
    .execute(pool)
    .await?;
    
    Ok(result.last_insert_rowid())
}
