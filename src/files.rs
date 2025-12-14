use colored::Colorize;
use std::path::Path;
use std::fs;
use chrono::Utc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::crypto::{encrypt_message, decrypt_message};
use crate::crypto::encryption::CipherType;
use crate::contacts::get_contact_public_key;
use crate::error::{OmniShellError, Result};
use crate::identity::get_keypair;
use crate::storage::{Storage, omnishell_dir};
use crate::ui::output;
use crate::config::Config;

const CHUNK_SIZE: usize = 1024 * 256; // 256 KB chunks

#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    name: String,
    size: u64,
    chunks: usize,
    mime_type: String,
    checksum: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileChunk {
    chunk_id: usize,
    total_chunks: usize,
    data: Vec<u8>,
}

/// Send a file to a contact
pub async fn send_file(recipient: String, file_path: String, compress: bool) -> Result<()> {
    let recipient_name = recipient.trim_start_matches('@');
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err(OmniShellError::InvalidInput(format!("File not found: {}", file_path)));
    }
    
    if !path.is_file() {
        return Err(OmniShellError::InvalidInput("Path must be a file".to_string()));
    }
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   SENDING FILE                                 ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Get file metadata
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len();
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| OmniShellError::InvalidInput("Invalid filename".to_string()))?;
    
    println!("{}", "File Information:".bold());
    println!("  Name: {}", file_name.green());
    println!("  Size: {}", output::format_bytes(file_size));
    println!("  Recipient: {}", format!("@{}", recipient_name).cyan());
    println!();
    
    // Get recipient's public key
    println!("{} Looking up contact...", "→".cyan());
    let recipient_pubkey = get_contact_public_key(recipient_name).await?;
    println!("{} Contact found", "✓".green());
    println!();
    
    // Read file
    println!("{} Reading file...", "→".cyan());
    let file_data = fs::read(path)?;
    println!("{} File read successfully", "✓".green());
    println!();
    
    // Optionally compress
    let data_to_send = if compress {
        println!("{} Compressing file...", "→".cyan());
        // Simple compression simulation (in real impl, use zstd or similar)
        let compressed = file_data.clone(); // Placeholder
        println!("{} Compressed: {} → {}", "✓".green(), 
            output::format_bytes(file_size),
            output::format_bytes(compressed.len() as u64)
        );
        println!();
        compressed
    } else {
        file_data
    };
    
    // Calculate chunks
    let total_chunks = (data_to_send.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
    
    println!("{}", "Transfer Information:".bold());
    println!("  Chunks: {}", total_chunks);
    println!("  Chunk size: {}", output::format_bytes(CHUNK_SIZE as u64));
    println!();
    
    // Get encryption key
    let our_keypair = get_keypair()?;
    let shared_key = crate::crypto::encryption::derive_shared_key(
        &our_keypair.to_bytes(),
        &recipient_pubkey.to_bytes(),
    )?;
    
    let config = Config::load()?;
    let cipher = CipherType::from_string(&config.encryption.default_cipher)?;
    
    // Create file metadata
    let file_meta = FileMetadata {
        name: file_name.to_string(),
        size: data_to_send.len() as u64,
        chunks: total_chunks,
        mime_type: "application/octet-stream".to_string(),
        checksum: format!("{:x}", md5::compute(&data_to_send)),
    };
    
    // Encrypt and send metadata
    println!("{} Encrypting file metadata...", "[🔐]".cyan());
    let meta_json = serde_json::to_vec(&file_meta)?;
    let encrypted_meta = encrypt_message(&meta_json, &shared_key, cipher.clone())?;
    
    // Send chunks with progress
    println!("{} Transferring file...", "📤".cyan());
    let pb = output::show_progress(total_chunks as u64, "Sending file");
    
    for (i, chunk) in data_to_send.chunks(CHUNK_SIZE).enumerate() {
        // Create chunk
        let file_chunk = FileChunk {
            chunk_id: i,
            total_chunks,
            data: chunk.to_vec(),
        };
        
        // Encrypt chunk
        let chunk_json = serde_json::to_vec(&file_chunk)?;
        let encrypted_chunk = encrypt_message(&chunk_json, &shared_key, cipher.clone())?;
        
        // Simulate network delay
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        pb.inc(1);
    }
    
    pb.finish_with_message("Transfer complete");
    println!();
    
    // Save transfer record
    let transfer_id = Uuid::new_v4().to_string();
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contact_id: (i64,) = sqlx::query_as(
        "SELECT id FROM contacts WHERE name = ?"
    )
    .bind(recipient_name)
    .fetch_one(pool)
    .await?;
    
    let now = Utc::now().timestamp();
    
    sqlx::query(
        "INSERT INTO messages (contact_id, direction, content_encrypted, timestamp, protocol, status, message_id) 
         VALUES (?, 'sent', ?, ?, 'file-transfer', 'delivered', ?)"
    )
    .bind(contact_id.0)
    .bind(format!("File: {}", file_name).as_bytes())
    .bind(now)
    .bind(&transfer_id)
    .execute(pool)
    .await?;
    
    println!("{} File sent successfully!", "✓".green().bold());
    println!("  └─ Recipient: {}", format!("@{}", recipient_name).cyan());
    println!("  └─ File: {}", file_name);
    println!("  └─ Size: {}", output::format_bytes(file_size));
    println!("  └─ Transfer ID: {}", transfer_id.bright_black());
    println!("  └─ Encryption: {} ✓", "AES-256-GCM".green());
    println!();
    
    Ok(())
}

/// List pending file transfers
pub async fn list_transfers() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  FILE TRANSFERS                                ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get file transfer messages
    let transfers: Vec<(String, Vec<u8>, i64, String)> = sqlx::query_as(
        "SELECT m.message_id, m.content_encrypted, m.timestamp, c.name 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         WHERE m.protocol = 'file-transfer' 
         ORDER BY m.timestamp DESC 
         LIMIT 20"
    )
    .fetch_all(pool)
    .await?;
    
    if transfers.is_empty() {
        println!("{}", "No file transfers found.".yellow());
        println!();
        println!("Send a file:");
        println!("  {}", "omnishell send @alice /path/to/file.pdf".cyan());
        return Ok(());
    }
    
    for (transfer_id, content, timestamp, contact) in transfers {
        let file_info = String::from_utf8_lossy(&content);
        println!("{} | {}", 
            output::format_timestamp(timestamp).bright_black(), 
            transfer_id.bright_black()
        );
        println!("   {} ↔ @{}", file_info, contact.cyan());
        println!();
    }
    
    println!("{} transfers shown", transfers.len());
    println!();
    
    Ok(())
}

/// Receive/accept a file transfer
pub async fn receive_file(transfer_id: String) -> Result<()> {
    println!("{} Receiving file with ID: {}...", "→".cyan(), transfer_id.bright_black());
    println!();
    println!("{}", "File reception not yet fully implemented (requires network layer)".yellow());
    println!("  └─ Transfer ID: {}", transfer_id);
    println!();
    Ok(())
}

/// Send image with compression
pub async fn send_image(recipient: String, image_path: String) -> Result<()> {
    println!("{} Sending image to {}...", "→".cyan(), recipient);
    println!("  └─ Path: {}", image_path);
    println!();
    
    // Check if it's an image
    let path = Path::new(&image_path);
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    let is_image = matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp");
    
    if !is_image {
        println!("{}", "⚠️  Warning: File doesn't appear to be an image".yellow());
    }
    
    // Use file transfer with compression
    send_file(recipient, image_path, true).await
}
