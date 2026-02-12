// Spectrum Agility - Software-Defined Frequency Hopping
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use colored::Colorize;
use crate::error::Result;

// ISM Band limits (MHz)
const FREQ_MIN: f64 = 902.0;
const FREQ_MAX: f64 = 928.0;
const CHANNEL_WIDTH: f64 = 0.2; // 200kHz channels

pub struct FrequencyHopper {
    seed: String,
}

impl FrequencyHopper {
    pub fn new(seed: &str) -> Self {
        Self { seed: seed.to_string() }
    }

    /// Run the frequency hopping monitor
    pub fn monitor(&self) -> Result<()> {
        println!("{} Initializing Spectrum Agility...", "🔄".cyan());
        println!("Mission Seed: {}", self.seed.bold().green());
        println!("Band: {}-{} MHz (ISM)", FREQ_MIN, FREQ_MAX);
        println!("Syncing time slots...\n");

        loop {
            let hop = self.calculate_next_hop();
            
            // Clear current line to create a TUI update effect (simple)
            print!("\r\x1B[K"); 
            print!("{} [T+{}] Active Freq: {:.3} MHz | Status: {} ", 
                "📡".blue(),
                hop.time_slot % 1000, 
                hop.frequency,
                "SECURE".green().bold()
            );
            
            use std::io::Write;
            std::io::stdout().flush().unwrap();

            // Wait for next time slot boundary
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
            let next_slot = (hop.time_slot + 1) * 1000;
            if next_slot > now {
                thread::sleep(Duration::from_millis(next_slot - now));
            }
        }
    }

    fn calculate_next_hop(&self) -> HopInfo {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let time_slot = now / 1000; // 1-second hops

        // Deterministic RNG based on seed + time_slot
        // We combine the seed string bytes with the time slot bytes to create a unique seed state for this second
        let mut seed_bytes = [0u8; 32];
        let input = format!("{}:{}", self.seed, time_slot);
        let hash = md5::compute(input); // Using MD5 as a quick mixer, not for crypto security here
        
        // Copy hash to seed_bytes (StdRng needs 32 bytes)
        for (i, &b) in hash.iter().enumerate() {
            seed_bytes[i] = b;
            seed_bytes[i+16] = b; // Duplicating to fill 32 bytes
        }

        let mut rng = StdRng::from_seed(seed_bytes);
        
        // Pick a channel
        let num_channels = ((FREQ_MAX - FREQ_MIN) / CHANNEL_WIDTH) as u32;
        let channel_idx = rng.gen_range(0..num_channels);
        
        let frequency = FREQ_MIN + (channel_idx as f64 * CHANNEL_WIDTH);

        HopInfo {
            frequency,
            time_slot,
        }
    }
}

struct HopInfo {
    frequency: f64,
    time_slot: u64,
}
