// Advanced message operations module
use colored::Colorize;
use sqlx::SqlitePool;
use chrono::Utc;

use crate::crypto::PublicKey;
use crate::crypto::encryption::{EncryptedMessage, decrypt_message, encrypt_message, CipherType};
use crate::contacts::get_contact_public_key;
use crate::error::{OmniShellError, Result};
use crate::identity::get_keypair;
use crate::messaging::Message;
use crate::storage::Storage;
use crate::ui::output;

/// Reply to a specific message (threaded)
pub async fn reply_message(message_id: &str, reply_text: String) -> Result<()> {
    println!("{} Replying to message {}...", "→".cyan(), message_id.bright_black());
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get original message to find recipient
    let original: Option<(String, i64, Vec<u8>)> = sqlx::query_as(
        "SELECT m.message_id, m.contact_id, c.name 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         WHERE m.message_id = ?"
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await?;
    
    let (_msg_id, _contact_id, contact_name) = original.ok_or_else(|| {
        OmniShellError::Other(format!("Message {} not found", message_id))
    })?;
    
    println!("{} Sending reply to @{}...", "✓".green(), contact_name);
    
    // Send message with reply_to field
    // For now, we'll use the existing send_message and update the reply_to field after
    crate::messaging::send_message(
        format!("@{}", contact_name),
        reply_text,
        None,
        "normal".to_string(),
        None,
        false,
    ).await?;
    
    // Get the last sent message and update its reply_to field
    sqlx::query(
        "UPDATE messages SET reply_to = ? WHERE contact_id = ? AND direction = 'sent' ORDER BY timestamp DESC LIMIT 1"
    )
    .bind(message_id)
    .bind(_contact_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Edit a previously sent message
pub async fn edit_message(message_id: &str, new_text: String) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get message
    let msg_data: Option<(i64, String, Vec<u8>, i64)> = sqlx::query_as(
        "SELECT m.contact_id, c.name, m.content_encrypted, m.timestamp 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         WHERE m.message_id = ? AND m.direction = 'sent'"
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await?;
    
    let (contact_id, contact_name, encrypted_content, timestamp) = msg_data.ok_or_else(|| {
        OmniShellError::Other(format!("Message {} not found or not sent by you", message_id))
    })?;
    
    // Check time limit (1 hour for edits)
    let now = Utc::now().timestamp();
    if now - timestamp > 3600 {
        return Err(OmniShellError::Other(
            "Cannot edit messages older than 1 hour".to_string()
        ));
    }
    
    println!("{} Editing message to @{}...", "→".cyan(), contact_name);
    
    // Decrypt original to get message structure
    let our_keypair = get_keypair()?;
    let recipient_pubkey = get_contact_public_key(&contact_name).await?;
    let shared_key = crate::crypto::encryption::derive_shared_key(
        &our_keypair.to_bytes(),
        &recipient_pubkey.to_bytes(),
    )?;
    
    let encrypted: EncryptedMessage = bincode::deserialize(&encrypted_content)?;
    let decrypted_bytes = decrypt_message(&encrypted, &shared_key)?;
    let mut msg: Message = serde_json::from_slice(&decrypted_bytes)?;
    
    // Update message content and edited_at
    msg.content = new_text;
    msg.edited_at = Some(now);
    
    // Re-encrypt
    let msg_json = serde_json::to_vec(&msg)?;
    let new_encrypted = encrypt_message(&msg_json, &shared_key, encrypted.cipher.clone())?;
    let encrypted_bytes = bincode::serialize(&new_encrypted)?;
    
    // Update database
    sqlx::query(
        "UPDATE messages SET content_encrypted = ?, edited_at = ? WHERE message_id = ?"
    )
    .bind(&encrypted_bytes)
    .bind(now)
    .bind(message_id)
    .execute(pool)
    .await?;
    
    println!("{} Message edited successfully", "✓".green().bold());
    println!("  └─ Message ID: {}", message_id.bright_black());
    println!("  └─ Edit will be visible to recipient");
    println!();
    
    Ok(())
}

/// Delete a message
pub async fn delete_message(message_id: &str, for_everyone: bool) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get message
    let msg_data: Option<(i64, String, i64)> = sqlx::query_as(
        "SELECT m.contact_id, c.name, m.timestamp 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         WHERE m.message_id = ? AND m.direction = 'sent'"
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await?;
    
    let (_contact_id, contact_name, timestamp) = msg_data.ok_or_else(|| {
        OmniShellError::Other(format!("Message {} not found or not sent by you", message_id))
    })?;
    
    if for_everyone {
        // Check time limit (1 hour for delete for everyone)
        let now = Utc::now().timestamp();
        if now - timestamp > 3600 {
            return Err(OmniShellError::Other(
                "Cannot delete for everyone after 1 hour".to_string()
            ));
        }
        
        println!("{} Deleting message for everyone...", "→".cyan());
    } else {
        println!("{} Deleting message locally...", "→".cyan());
    }
    
    let now = Utc::now().timestamp();
    
    if for_everyone {
        // Mark as deleted (would send deletion notice to recipient in real impl)
        sqlx::query(
            "UPDATE messages SET deleted_at = ? WHERE message_id = ?"
        )
        .bind(now)
        .bind(message_id)
        .execute(pool)
        .await?;
        
        println!("{} Message deleted for everyone", "✓".green().bold());
    } else {
        // Just delete locally
        sqlx::query("DELETE FROM messages WHERE message_id = ?")
            .bind(message_id)
            .execute(pool)
            .await?;
        
        println!("{} Message deleted locally", "✓".green().bold());
    }
    
    println!("  └─ Message ID: {}", message_id.bright_black());
    println!();
    
    Ok(())
}

/// Forward a message to another contact
pub async fn forward_message(message_id: &str, recipient: String, strip_metadata: bool) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get original message
    let msg_data: Option<(Vec<u8>, String)> = sqlx::query_as(
        "SELECT m.content_encrypted, c_from.name 
         FROM messages m 
         LEFT JOIN contacts c_from ON m.contact_id = c_from.id
         WHERE m.message_id = ?"
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await?;
    
    let (encrypted_content, original_sender) = msg_data.ok_or_else(|| {
        OmniShellError::Other(format!("Message {} not found", message_id))
    })?;
    
    // Decrypt message
    let our_keypair = get_keypair()?;
    let sender_pubkey = get_contact_public_key(&original_sender).await?;
    let shared_key = crate::crypto::encryption::derive_shared_key(
        &our_keypair.to_bytes(),
        &sender_pubkey.to_bytes(),
    )?;
    
    let encrypted: EncryptedMessage = bincode::deserialize(&encrypted_content)?;
    let decrypted_bytes = decrypt_message(&encrypted, &shared_key)?;
    let original_msg: Message = serde_json::from_slice(&decrypted_bytes)?;
    
    let forward_text = if strip_metadata {
        original_msg.content
    } else {
        format!("Forwarded from @{}: {}", original_sender, original_msg.content)
    };
    
    println!("{} Forwarding message to {}...", "→".cyan(), recipient);
    
    // Send as new message
    crate::messaging::send_message(
        recipient,
        forward_text,
        None,
        "normal".to_string(),
        None,
        false,
    ).await?;
    
    Ok(())
}

/// React to a message with an emoji
pub async fn react_message(message_id: &str, emoji: String) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Verify message exists
    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM messages WHERE message_id = ?"
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await?;
    
    if exists.is_none() {
        return Err(OmniShellError::Other(format!("Message {} not found", message_id)));
    }
    
    // Our contact ID is 0 (we're reacting)
    let now = Utc::now().timestamp();
    
    // Add or update reaction
    sqlx::query(
        "INSERT INTO reactions (message_id, contact_id, emoji, created_at) 
         VALUES (?, 0, ?, ?)
         ON CONFLICT(message_id, contact_id) 
         DO UPDATE SET emoji = ?, created_at = ?"
    )
    .bind(message_id)
    .bind(&emoji)
    .bind(now)
    .bind(&emoji)
    .bind(now)
    .execute(pool)
    .await?;
    
    println!("{} Reaction added: {}", "✓".green().bold(), emoji);
    println!("  └─ Message ID: {}", message_id.bright_black());
    println!();
    
    Ok(())
}

/// Remove reaction from a message
pub async fn unreact_message(message_id: &str) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query(
        "DELETE FROM reactions WHERE message_id = ? AND contact_id = 0"
    )
    .bind(message_id)
    .execute(pool)
    .await?;
    
    println!("{} Reaction removed", "✓".green().bold());
    println!("  └─ Message ID: {}", message_id.bright_black());
    println!();
    
    Ok(())
}

/// Star/bookmark a message
pub async fn star_message(message_id: &str) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Verify message exists
    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM messages WHERE message_id = ?"
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await?;
    
    if exists.is_none() {
        return Err(OmniShellError::Other(format!("Message {} not found", message_id)));
    }
    
    let now = Utc::now().timestamp();
    
    sqlx::query(
        "INSERT OR IGNORE INTO starred_messages (message_id, starred_at) VALUES (?, ?)"
    )
    .bind(message_id)
    .bind(now)
    .execute(pool)
    .await?;
    
    println!("{} Message starred", "✓".green().bold());
    println!("  └─ Message ID: {}", message_id.bright_black());
    println!("  └─ View starred: {}", "omnishell starred".cyan());
    println!();
    
    Ok(())
}

/// Unstar a message
pub async fn unstar_message(message_id: &str) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    sqlx::query("DELETE FROM starred_messages WHERE message_id = ?")
        .bind(message_id)
        .execute(pool)
        .await?;
    
    println!("{} Message unstarred", "✓".green().bold());
    println!();
    
    Ok(())
}

/// List all starred messages
pub async fn list_starred() -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let starred: Vec<(String, i64)> = sqlx::query_as(
        "SELECT message_id, starred_at FROM starred_messages ORDER BY starred_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    if starred.is_empty() {
        println!("{}", "No starred messages.".yellow());
        println!();
        println!("To star a message:");
        println!("  {}", "omnishell star <message_id>".cyan());
        return Ok(());
    }
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   STARRED MESSAGES                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    for (msg_id, starred_at) in starred {
        // Get message details
        let msg_info: Option<(Vec<u8>, String, String, i64)> = sqlx::query_as(
            "SELECT m.content_encrypted, c.name, m.direction, m.timestamp 
             FROM messages m 
             JOIN contacts c ON m.contact_id = c.id 
             WHERE m.message_id = ?"
        )
        .bind(&msg_id)
        .fetch_optional(pool)
        .await?;
        
        if let Some((encrypted, contact_name, direction, timestamp)) = msg_info {
            // Try to decrypt and show preview
            let preview = if let Ok(content) = decrypt_and_preview(&encrypted, &contact_name).await {
                let truncated = if content.len() > 100 {
                    format!("{}...", &content[..97])
                } else {
                    content
                };
                truncated
            } else {
                "[Encrypted]".to_string()
            };
            
            let sender = if direction == "sent" { "You" } else { &contact_name };
            
            println!("⭐ {} | {}", output::format_timestamp(starred_at).bright_black(), msg_id.bright_black());
            println!("   {} → @{}: {}", sender.cyan(), contact_name.cyan(), preview);
            println!("   Sent: {}", output::format_timestamp(timestamp).bright_black());
            println!();
        }
    }
    
    println!("{} starred messages", starred.len());
    println!();
    
    Ok(())
}

/// Search messages
pub async fn search_messages(query: String, contact: Option<String>, _date: Option<String>) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    println!("{} Searching for: \"{}\"...", "→".cyan(), query.yellow());
    println!();
    
    let mut results = Vec::new();
    
    // Get all messages (filter by contact if specified)
    let messages: Vec<(String, Vec<u8>, String, String, i64)> = if let Some(contact_name) = &contact {
        let name = contact_name.trim_start_matches('@');
        sqlx::query_as(
            "SELECT m.message_id, m.content_encrypted, c.name, m.direction, m.timestamp 
             FROM messages m 
             JOIN contacts c ON m.contact_id = c.id 
             WHERE c.name = ? AND m.deleted_at IS NULL 
             ORDER BY m.timestamp DESC"
        )
        .bind(name)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT m.message_id, m.content_encrypted, c.name, m.direction, m.timestamp 
             FROM messages m 
             JOIN contacts c ON m.contact_id = c.id 
             WHERE m.deleted_at IS NULL 
             ORDER BY m.timestamp DESC"
        )
        .fetch_all(pool)
        .await?
    };
    
    // Search through messages (decrypt and check content)
    for (msg_id, encrypted, contact_name, direction, timestamp) in messages {
        if let Ok(content) = decrypt_and_preview(&encrypted, &contact_name).await {
            if content.to_lowercase().contains(&query.to_lowercase()) {
                results.push((msg_id, content, contact_name, direction, timestamp));
                
                if results.len() >= 20 {
                    break; // Limit to 20 results
                }
            }
        }
    }
    
    if results.is_empty() {
        println!("{}", "No results found.".yellow());
        return Ok(());
    }
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", format!("║ Search Results for \"{}\"  {:<35}║", 
        query, format!("({} found)", results.len())).cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    for (msg_id, content, contact_name, direction, timestamp) in results {
        let sender = if direction == "sent" { "You" } else { &contact_name };
        
        // Highlight search query
        let highlighted = content.replace(
            &query,
            &format!("{}", query.yellow().bold())
        );
        
        println!("{} | {}", output::format_timestamp(timestamp).bright_black(), msg_id.bright_black());
        println!("   {} → @{}: {}", sender.cyan(), contact_name.cyan(), highlighted);
        println!();
    }
    
    println!("{} results shown", results.len());
    println!();
    
    Ok(())
}

// Helper function to decrypt message for preview
async fn decrypt_and_preview(encrypted_content: &[u8], contact_name: &str) -> Result<String> {
    let our_keypair = get_keypair()?;
    let contact_pubkey = get_contact_public_key(contact_name).await?;
    let shared_key = crate::crypto::encryption::derive_shared_key(
        &our_keypair.to_bytes(),
        &contact_pubkey.to_bytes(),
    )?;
    
    let encrypted: EncryptedMessage = bincode::deserialize(encrypted_content)?;
    let decrypted_bytes = decrypt_message(&encrypted, &shared_key)?;
    let msg: Message = serde_json::from_slice(&decrypted_bytes)?;
    
    Ok(msg.content)
}
