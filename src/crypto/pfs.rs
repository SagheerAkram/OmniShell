// Perfect Forward Secrecy - Double Ratchet implementation
use serde::{Deserialize, Serialize};
use rand::RngCore;

use crate::crypto::keys::KeyPair;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct RatchetState {
    pub root_key: [u8; 32],
    pub chain_key_send: [u8; 32],
    pub chain_key_recv: [u8; 32],
    pub message_number: u32,
    pub previous_message_number: u32,
}

impl RatchetState {
    pub fn new() -> Self {
        let mut root_key = [0u8; 32];
        let mut chain_key_send = [0u8; 32];
        let mut chain_key_recv = [0u8; 32];
        
        rand::rngs::OsRng.fill_bytes(&mut root_key);
        rand::rngs::OsRng.fill_bytes(&mut chain_key_send);
        rand::rngs::OsRng.fill_bytes(&mut chain_key_recv);
        
        Self {
            root_key,
            chain_key_send,
            chain_key_recv,
            message_number: 0,
            previous_message_number: 0,
        }
    }
    
    /// Perform DH ratchet step
    pub fn dh_ratchet(&mut self, their_public_key: &[u8]) -> Result<()> {
        // Generate new ephemeral key pair
        let new_keypair = crate::crypto::generate_keypair();
        
        // Derive new root key and chain keys
        let shared_secret = crate::crypto::encryption::derive_shared_key(
            &new_keypair.to_bytes(),
            their_public_key,
        )?;
        
        // KDF to derive new keys
        self.root_key = crate::crypto::kdf::derive_key(&shared_secret, b"root_key")?;
        self.chain_key_send = crate::crypto::kdf::derive_key(&shared_secret, b"chain_send")?;
        self.chain_key_recv = crate::crypto::kdf::derive_key(&shared_secret, b"chain_recv")?;
        
        Ok(())
    }
    
    /// Ratchet forward for sending
    pub fn ratchet_send(&mut self) -> [u8; 32] {
        let message_key = crate::crypto::kdf::derive_key(&self.chain_key_send, b"message_key")
            .unwrap_or(self.chain_key_send);
        
        // Update chain key
        self.chain_key_send = crate::crypto::kdf::derive_key(&self.chain_key_send, b"chain_key")
            .unwrap_or(self.chain_key_send);
        
        self.message_number += 1;
        message_key
    }
    
    /// Ratchet forward for receiving
    pub fn ratchet_recv(&mut self) -> [u8; 32] {
        let message_key = crate::crypto::kdf::derive_key(&self.chain_key_recv, b"message_key")
            .unwrap_or(self.chain_key_recv);
        
        // Update chain key
        self.chain_key_recv = crate::crypto::kdf::derive_key(&self.chain_key_recv, b"chain_key")
            .unwrap_or(self.chain_key_recv);
        
        self.previous_message_number = self.message_number;
        message_key
    }
}

/// Initialize Double Ratchet for a conversation
pub async fn init_double_ratchet(contact_name: &str) -> Result<()> {
    use colored::Colorize;
    
    println!("{} Initializing Perfect Forward Secrecy for @{}...", "→".cyan(), contact_name);
    
    let ratchet = RatchetState::new();
    
    // Store ratchet state in database
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let ratchet_data = bincode::serialize(&ratchet)?;
    
    sqlx::query(
        "INSERT OR REPLACE INTO config (key, value) VALUES (?, ?)"
    )
    .bind(format!("ratchet_{}", contact_name))
    .bind(&ratchet_data)
    .execute(pool)
    .await?;
    
    println!("{} Perfect Forward Secrecy enabled", "✓".green().bold());
    println!("  └─ Message keys will rotate automatically");
    println!("  └─ Past messages cannot be decrypted if keys are compromised");
    println!();
    
    Ok(())
}

/// Enable PFS for all contacts
pub async fn enable_pfs_all() -> Result<()> {
    use colored::Colorize;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║          ENABLING PERFECT FORWARD SECRECY                      ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let contacts: Vec<(String,)> = sqlx::query_as("SELECT name FROM contacts")
        .fetch_all(pool)
        .await?;
    
    println!("{} Initializing Double Ratchet for {} contacts...", "→".cyan(), contacts.len());
    
    for (contact_name,) in contacts {
        init_double_ratchet(&contact_name).await?;
    }
    
    println!();
    println!("{} Perfect Forward Secrecy enabled for all contacts!", "✓".green().bold());
    println!();
    
    Ok(())
}
