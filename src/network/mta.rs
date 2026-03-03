// Multipath Transport Aggregation (MTA) Module
use serde::{Deserialize, Serialize};
use colored::Colorize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use crc32fast::Hasher;
use uuid::Uuid;

use crate::error::Result;
use crate::ui::output;

// Constants
const MAX_FRAGMENT_SIZE: usize = 1024; // 1KB chunks

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTAFragment {
    pub message_id: String,
    pub total_chunks: u32,
    pub chunk_id: u32,
    pub data: Vec<u8>,
    pub checksum: u32,
}

#[derive(Debug, Clone)]
struct ReassemblyBuffer {
    fragments: HashMap<u32, Vec<u8>>,
    total_chunks: u32,
    last_update: std::time::Instant,
}

// Global state for reassembly (simulation)
lazy_static::lazy_static! {
    static ref REASSEMBLY_MAP: Arc<Mutex<HashMap<String, ReassemblyBuffer>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

/// Fragment a message into multiple chunks for MTA
pub fn fragment_message(message: &[u8]) -> Vec<MTAFragment> {
    let message_id = Uuid::new_v4().to_string();
    let total_chunks = (message.len() as f64 / MAX_FRAGMENT_SIZE as f64).ceil() as u32;
    let mut fragments = Vec::new();

    for (i, chunk) in message.chunks(MAX_FRAGMENT_SIZE).enumerate() {
        let chunk_id = i as u32;
        
        // Calculate checksum
        let mut hasher = Hasher::new();
        hasher.update(chunk);
        let checksum = hasher.finalize();

        fragments.push(MTAFragment {
            message_id: message_id.clone(),
            total_chunks,
            chunk_id,
            data: chunk.to_vec(),
            checksum,
        });
    }

    fragments
}

/// Process an incoming fragment and attempt reassembly
pub fn receive_fragment(fragment: MTAFragment) -> Result<Option<Vec<u8>>> {
    let mut map = REASSEMBLY_MAP.lock().unwrap();
    
    // Check checksum
    let mut hasher = Hasher::new();
    hasher.update(&fragment.data);
    if hasher.finalize() != fragment.checksum {
        println!("{}", "⚠️  MTA: Fragment checksum mismatch, discarding".yellow());
        return Ok(None);
    }

    let buffer = map.entry(fragment.message_id.clone()).or_insert(ReassemblyBuffer {
        fragments: HashMap::new(),
        total_chunks: fragment.total_chunks,
        last_update: std::time::Instant::now(),
    });

    // Store fragment
    buffer.fragments.insert(fragment.chunk_id, fragment.data);
    buffer.last_update = std::time::Instant::now();

    // Check if we have all chunks
    if buffer.fragments.len() as u32 == buffer.total_chunks {
        let mut full_message = Vec::new();
        for i in 0..buffer.total_chunks {
            if let Some(data) = buffer.fragments.get(&i) {
                full_message.extend_from_slice(data);
            } else {
                // Should not happen due to length check
                return Ok(None);
            }
        }
        
        // Cleanup
        map.remove(&fragment.message_id);
        
        return Ok(Some(full_message));
    }

    println!("{} MTA: Received chunk {}/{}", 
        "🧩".blue(), 
        buffer.fragments.len(), 
        buffer.total_chunks
    );

    Ok(None)
}

// Multipath Transport Aggregation (MTA) Router
use colored::Colorize;

use crate::error::Result;
use crate::ui::output;

/// Send a message using Autonomous Transport Layer (MTA)
pub async fn send_mta_message(recipient: &str, message: &str) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║          AUTONOMOUS TRANSPORT LAYER (MTA) ROUTER               ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();

    println!("{} Analyzing network topology for best route to {}...", "→".cyan(), recipient);
    output::show_encryption_animation("Calculating offline transport paths", 30).await;
    
    // Attempt 1: High-Bandwidth Wi-Fi Direct / LAN TCP
    println!("{} Attempting High-Bandwidth Wi-Fi Direct...", "1️⃣".cyan());
    if let Ok(_) = crate::network::p2p::send_p2p_message(recipient, message.as_bytes().to_vec()).await {
        println!("{} MTA Routing Successful: High-Bandwidth Path Selected", "✓".green().bold());
        return Ok(());
    } else {
        println!("{} Wi-Fi Direct unreachable. Failing over...", "x".red());
    }
    
    // Attempt 2: Low-Level BLE Mesh Hardware Route
    println!();
    println!("{} Initiating BLE Mesh Failover...", "2️⃣".cyan());
    if let Ok(_) = crate::network::bluetooth::send_via_bluetooth(recipient, message).await {
        println!("{} MTA Routing Successful: BLE Mesh Path Selected", "✓".green().bold());
        return Ok(());
    } else {
        println!("{} BLE Mesh unreachable or disabled.", "x".red());
    }
    
    println!();
    println!("{} All autonomous transport layers failed to reach target.", "❌".red().bold());
    println!("  └─ Device may be completely out of range or offline.");
    
    anyhow::bail!("MTA Routing failed to find an offline path");
}

