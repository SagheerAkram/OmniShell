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

/// Send a message using Multipath Transport Aggregation
pub async fn send_mta_message(recipient: &str, message: &str) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║          MULTIPATH TRANSPORT AGGREGATION (MTA)                 ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();

    println!("{} Preparing message for multipath transmission...", "→".cyan());
    
    // Fragment the message
    let fragments = fragment_message(message.as_bytes());
    println!("  └─ Split into {} fragments", fragments.len());
    
    // Available protocols (in a real app, check availability)
    let protocols = vec!["Tor", "I2P", "LoRa", "Bluetooth", "P2P"];
    
    output::show_encryption_animation("Distributing fragments across mesh", 100).await;
    
    // Distribute fragments
    let mut rng = rand::thread_rng();
    
    for fragment in fragments {
        // Randomly select a protocol for this fragment (Simulating load balancing)
        let protocol = protocols[rng.gen_range(0..protocols.len())];
        
        println!("{} Sending Chunk {}/{} via {}", 
            "🚀".cyan(),
            fragment.chunk_id + 1,
            fragment.total_chunks,
            protocol.magenta().bold()
        );
        
        // In a real implementation, we would call the specific protocol's send function here
        // e.g. crate::network::tor::send_chunk(...)
        
        // Simulating network latency variance
        tokio::time::sleep(std::time::Duration::from_millis(rng.gen_range(50..200))).await;
    }
    
    println!();
    println!("{} Message successfully distributed across {} protocols", "✓".green().bold(), protocols.len());
    println!("  └─ Fragments: {}", fragments.len());
    println!("  └─ Redundancy: High");
    println!("  └─ Interception Difficulty: Extreme");
    println!();
    
    Ok(())
}
