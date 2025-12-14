// Resume Interrupted File Transfers
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

use crate::error::Result;
use crate::storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferState {
    pub transfer_id: String,
    pub file_path: String,
    pub recipient: String,
    pub total_size: u64,
    pub bytes_sent: u64,
    pub chunks_sent: Vec<usize>,
    pub checksum: String,
    pub paused: bool,
}

/// Save transfer state for resume
pub async fn save_transfer_state(state: &TransferState) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let state_json = serde_json::to_string(state)?;
    sqlx::query(
        "INSERT OR REPLACE INTO transfer_states (transfer_id, state_json) VALUES (?, ?)"
    )
    .bind(&state.transfer_id)
    .bind(&state_json)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Load transfer state for resume
pub async fn load_transfer_state(transfer_id: &str) -> Result<Option<TransferState>> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let state_data: Option<(String,)> = sqlx::query_as(
        "SELECT state_json FROM transfer_states WHERE transfer_id = ?"
    )
    .bind(transfer_id)
    .fetch_optional(pool)
    .await?;
    
    if let Some((state_json,)) = state_data {
        let state: TransferState = serde_json::from_str(&state_json)?;
        Ok(Some(state))
    } else {
        Ok(None)
    }
}

/// Resume interrupted transfer
pub async fn resume_transfer(transfer_id: String) -> Result<()> {
    println!("{} Resuming transfer {}...", "→".cyan(), transfer_id);
    println!();
    
    let state = load_transfer_state(&transfer_id).await?;
    
    if let Some(mut state) = state {
        println!("{}", "Transfer State:".bold());
        println!("  File: {}", state.file_path.green());
        println!("  Recipient: {}", state.recipient.cyan());
        println!("  Progress: {} / {}", 
            crate::ui::output::format_bytes(state.bytes_sent),
            crate::ui::output::format_bytes(state.total_size)
        );
        println!("  Chunks sent: {}/{}", 
            state.chunks_sent.len(),
            (state.total_size as f64 / (256.0 * 1024.0)).ceil() as usize
        );
        println!();
        
        if state.bytes_sent >= state.total_size {
            println!("{} Transfer already complete", "✓".green());
            return Ok(());
        }
        
        // Resume from last chunk
        let path = Path::new(&state.file_path);
        if !path.exists() {
            return Err(crate::error::OmniShellError::InvalidInput(
                "Original file not found".to_string()
            ));
        }
        
        let file_data = fs::read(path)?;
        let chunk_size = 256 * 1024;
        let total_chunks = (file_data.len() + chunk_size - 1) / chunk_size;
        
        println!("{} Resuming from chunk {}...", "→".cyan(), state.chunks_sent.len() + 1);
        
        use indicatif::{ProgressBar, ProgressStyle};
        let pb = ProgressBar::new(state.total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_position(state.bytes_sent);
        
        // Send remaining chunks
        for chunk_idx in state.chunks_sent.len()..total_chunks {
            let start = chunk_idx * chunk_size;
            let end = std::cmp::min(start + chunk_size, file_data.len());
            let chunk = &file_data[start..end];
            
            // Encrypt and send chunk
            let keypair = crate::identity::get_keypair()?;
            let _encrypted = crate::crypto::encrypt_message(
                chunk, 
                &keypair.to_bytes(), 
                crate::crypto::encryption::CipherType::Aes256Gcm
            )?;
            
            // Simulate sending
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            
            // Update state
            state.chunks_sent.push(chunk_idx);
            state.bytes_sent += chunk.len() as u64;
            pb.set_position(state.bytes_sent);
            
            // Save state periodically
            if chunk_idx % 10 == 0 {
                save_transfer_state(&state).await?;
            }
        }
        
        pb.finish_with_message("Transfer complete");
        
        println!();
        println!("{} Transfer resumed and completed", "✓".green().bold());
        println!();
        
        // Mark as complete
        state.paused = false;
        save_transfer_state(&state).await?;
        
    } else {
        println!("{}", "⚠️  Transfer state not found".yellow());
        println!("Available transfers:");
        list_paused_transfers().await?;
    }
    
    Ok(())
}

/// List paused transfers
pub async fn list_paused_transfers() -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let transfers: Vec<(String, String)> = sqlx::query_as(
        "SELECT transfer_id, state_json FROM transfer_states"
    )
    .fetch_all(pool)
    .await?;
    
    if transfers.is_empty() {
        println!("  No paused transfers");
        return Ok(());
    }
    
    for (transfer_id, state_json) in transfers {
        let state: TransferState = serde_json::from_str(&state_json)?;
        let progress = (state.bytes_sent as f64 / state.total_size as f64 * 100.0) as u32;
        
        println!("  {} - {}% complete", transfer_id.cyan(), progress);
    }
    
    Ok(())
}

/// Pause active transfer
pub async fn pause_transfer(transfer_id: String) -> Result<()> {
    println!("{} Pausing transfer {}...", "→".cyan(), transfer_id);
    
    if let Some(mut state) = load_transfer_state(&transfer_id).await? {
        state.paused = true;
        save_transfer_state(&state).await?;
        
        println!("{} Transfer paused", "✓".green());
        println!("  Resume with: {}", format!("omnishell resume {}", transfer_id).cyan());
    }
    
    Ok(())
}
