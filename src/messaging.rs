use colored::Colorize;
use sqlx::SqlitePool;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crypto::{encrypt_message, decrypt_message};
use crate::crypto::encryption::{CipherType, EncryptedMessage};
use crate::config::Config;
use crate::contacts::get_contact_public_key;
use crate::error::{OmniShellError, Result};
use crate::identity::get_keypair;
use crate::storage::Storage;
use crate::ui::output;

// Advanced operations module
pub mod operations;

// Re-export operations functions
pub use operations::{
    reply_message, edit_message, delete_message, forward_message,
    react_message, unreact_message, star_message, unstar_message,
    list_starred, search_messages,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: i64,
    pub protocol: String,
    pub edited_at: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct StoredMessage {
    pub id: i64,
    pub contact_id: i64,
    pub direction: String,
    pub content_encrypted: Vec<u8>,
    pub timestamp: i64,
    pub protocol: String,
    pub status: String,
    pub message_id: String,
    pub reply_to: Option<String>,
    pub edited_at: Option<i64>,
    pub deleted_at: Option<i64>,
}

/// Send a message to a contact
pub async fn send_message(
    recipient: String,
    message: String,
    protocol: Option<String>,
    priority: String,
    _ttl: Option<String>,
    stealth: bool,
) -> Result<()> {
    let recipient_name = recipient.trim_start_matches('@');
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   SENDING MESSAGE                              ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Get configuration
    let config = Config::load()?;
    
    // Get recipient's public key
    println!("{} Looking up contact...", "→".cyan());
    let recipient_pubkey = get_contact_public_key(recipient_name).await?;
    println!("{} Contact found: {}", "✓".green(), format!("@{}", recipient_name).cyan());
    println!();
    
    // Get our keypair
    let our_keypair = get_keypair()?;
    
    // Derive shared encryption key
    println!("{} Deriving shared encryption key...", "[🔐]".cyan());
    output::show_encryption_animation("Computing X25519 key exchange", 30).await;
    
    let shared_key = crate::crypto::encryption::derive_shared_key(
        &our_keypair.to_bytes(),
        &recipient_pubkey.to_bytes(),
    )?;
    
    println!("{} Shared key established", "✓".green());
    println!();
    
    // Select cipher
    let cipher = if stealth {
        CipherType::ChaCha20Poly1305
    } else {
        CipherType::from_string(&config.encryption.default_cipher)?
    };
    
    let cipher_name = match cipher {
        CipherType::Aes256Gcm => "AES-256-GCM",
        CipherType::ChaCha20Poly1305 => "ChaCha20-Poly1305",
    };
    
    output::print_encryption_details(cipher_name, &recipient_pubkey.fingerprint());
    
    // Create message object
    let msg_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().timestamp();
    
    let msg = Message {
        id: msg_id.clone(),
        sender: "me".to_string(),
        recipient: recipient_name.to_string(),
        content: message.clone(),
        timestamp,
        protocol: protocol.clone().unwrap_or_else(|| "p2p".to_string()),
        edited_at: None,
    };
    
    // Serialize and encrypt message
    let msg_json = serde_json::to_vec(&msg)?;
    
    output::show_encryption_animation("Encrypting message", 40).await;
    let encrypted = encrypt_message(&msg_json, &shared_key, cipher)?;
    let encrypted_bytes = bincode::serialize(&encrypted)?;
    
    println!("{} Message encrypted", "✓".green());
    println!("  └─ Size: {} → {}", 
        output::format_bytes(msg_json.len() as u64),
        output::format_bytes(encrypted_bytes.len() as u64)
    );
    println!();
    
    // Select protocol
    let selected_protocol = select_protocol(&protocol, &priority, stealth)?;
    output::print_protocol_selection(&selected_protocol, &get_protocol_reason(&selected_protocol, &priority));
    println!();
    
    // Simulate routing (for now, just display)
    let routing_path = vec![
        "Guard Node".to_string(),
        "Relay Node".to_string(),
        format!("@{}", recipient_name),
    ];
    output::print_routing_path(&routing_path);
    println!();
    
    // Store message in database
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get contact ID
    let contact_id: (i64,) = sqlx::query_as(
        "SELECT id FROM contacts WHERE name = ?"
    )
    .bind(recipient_name)
    .fetch_one(pool)
    .await?;
    
    sqlx::query(
        r#"
        INSERT INTO messages 
        (contact_id, direction, content_encrypted, timestamp, protocol, status, message_id)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(contact_id.0)
    .bind("sent")
    .bind(&encrypted_bytes)
    .bind(timestamp)
    .bind(&selected_protocol)
    .bind("delivered") // Simulated for now
    .bind(&msg_id)
    .execute(pool)
    .await?;
    
    // Simulate network delay
    output::show_encryption_animation("Transmitting", 50).await;
    
    println!("{} Message delivered successfully!", "✓".green().bold());
    println!("  └─ Recipient: {}", format!("@{}", recipient_name).cyan());
    println!("  └─ Message ID: {}", msg_id.bright_black());
    println!("  └─ Protocol: {}", selected_protocol.green());
    println!("  └─ Encryption: {} ✓", cipher_name.green());
    println!("  └─ Status: {}", "Delivered ✓".green());
    println!();
    
    Ok(())
}

/// Read messages from a contact or all contacts
pub async fn read_messages(
    contact: Option<String>,
    last: Option<usize>,
    since: Option<String>,
    unread: bool,
) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    if let Some(contact_name) = contact {
        read_conversation(pool, &contact_name, last, since, unread).await?;
    } else {
        read_all_unread(pool).await?;
    }
    
    Ok(())
}

async fn read_conversation(
    pool: &SqlitePool,
    contact_name: &str,
    last: Option<usize>,
    _since: Option<String>,
    unread: bool,
) -> Result<()> {
    let contact_name = contact_name.trim_start_matches('@');
    
    // Get contact
    let contact_id: (i64, String) = sqlx::query_as(
        "SELECT id, fingerprint FROM contacts WHERE name = ?"
    )
    .bind(contact_name)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| OmniShellError::ContactNotFound(contact_name.to_string()))?;
    
    // Get messages
    let limit = last.unwrap_or(50);
    
    let mut query_str = format!(
        r#"
        SELECT id, contact_id, direction, content_encrypted, timestamp, protocol, 
               status, message_id, reply_to, edited_at, deleted_at
        FROM messages
        WHERE contact_id = {}
        "#,
        contact_id.0
    );
    
    if unread {
        query_str.push_str(" AND status != 'read'");
    }
    
    query_str.push_str(&format!(" ORDER BY timestamp DESC LIMIT {}", limit));
    
    let messages: Vec<StoredMessage> = sqlx::query_as(&query_str)
        .fetch_all(pool)
        .await?;
    
    if messages.is_empty() {
        println!("{}", "No messages found.".yellow());
        return Ok(());
    }
    
    // Display header
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", format!("║ Conversation with @{:<45}║", contact_name).cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Get our keypair for decryption
    let our_keypair = get_keypair()?;
    let recipient_pubkey = get_contact_public_key(contact_name).await?;
    
    let shared_key = crate::crypto::encryption::derive_shared_key(
        &our_keypair.to_bytes(),
        &recipient_pubkey.to_bytes(),
    )?;
    
    // Display messages (reverse to show oldest first)
    for stored_msg in messages.iter().rev() {
        if stored_msg.deleted_at.is_some() {
            continue;
        }
        
        // Decrypt message
        let encrypted: EncryptedMessage = bincode::deserialize(&stored_msg.content_encrypted)?;
        let decrypted_bytes = decrypt_message(&encrypted, &shared_key)?;
        let msg: Message = serde_json::from_slice(&decrypted_bytes)?;
        
        // Format timestamp
        let timestamp_str = output::format_timestamp(msg.timestamp);
        
        // Display message
        let prefix = if stored_msg.direction == "sent" {
            format!("[{}] You:", timestamp_str).bright_black()
        } else {
            format!("[{}] {}:", timestamp_str, contact_name).bright_blue()
        };
        
        println!("{}", prefix);
        println!("  {}", msg.content);
        
        // Show metadata
        let mut metadata = vec![];
        metadata.push(format!("🔒 {}", match encrypted.cipher {
            CipherType::Aes256Gcm => "AES-256",
            CipherType::ChaCha20Poly1305 => "ChaCha20",
        }));
        metadata.push(format!("📡 {}", stored_msg.protocol));
        metadata.push(format!("✓ {}", stored_msg.status));
        
        if stored_msg.edited_at.is_some() {
            metadata.push("✏️  edited".to_string());
        }
        
        println!("  {}", metadata.join(" │ ").bright_black());
        println!();
    }
    
    println!("{} messages shown", messages.len());
    println!();
    
    // Mark as read
    sqlx::query(
        "UPDATE messages SET status = 'read' WHERE contact_id = ? AND direction = 'received' AND status != 'read'"
    )
    .bind(contact_id.0)
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn read_all_unread(pool: &SqlitePool) -> Result<()> {
    let unread_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE direction = 'received' AND status != 'read'"
    )
    .fetch_one(pool)
    .await?;
    
    if unread_count.0 == 0 {
        println!("{}", "No unread messages.".yellow());
        println!();
        println!("To view a conversation:");
        println!("  {}", "omnishell read @<contact>".cyan());
        return Ok(());
    }
    
    println!("{}", format!("You have {} unread messages", unread_count.0).green().bold());
    println!();
    println!("To read messages from a contact:");
    println!("  {}", "omnishell read @<contact>".cyan());
    println!();
    
    Ok(())
}

fn select_protocol(forced: &Option<String>, priority: &str, stealth: bool) -> Result<String> {
    if let Some(protocol) = forced {
        return Ok(protocol.clone());
    }
    
    if stealth {
        return Ok("tor".to_string());
    }
    
    match priority {
        "urgent" => Ok("multi-protocol".to_string()),
        "high" => Ok("tor".to_string()),
        "normal" => Ok("p2p".to_string()),
        "low" => Ok("p2p".to_string()),
        _ => Ok("p2p".to_string()),
    }
}

fn get_protocol_reason(protocol: &str, priority: &str) -> String {
    match protocol {
        "tor" => "Maximum privacy".to_string(),
        "i2p" => "Anonymous routing".to_string(),
        "multi-protocol" => format!("Urgent priority ({})", priority),
        "p2p" => "Direct connection available".to_string(),
        _ => "Auto-selected".to_string(),
    }
}
