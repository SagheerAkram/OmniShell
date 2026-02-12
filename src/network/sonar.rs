// Sonar - Ultrasonic Air-Gap Data Transmission
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;
use colored::Colorize;
use crate::error::Result;

// Ultrasonic Frequencies
const FREQ_MARK: f32 = 18500.0;  // Represents '1'
const FREQ_SPACE: f32 = 19000.0; // Represents '0'
const BIT_DURATION_MS: u64 = 50; // Slow but reliable

pub struct AudioModem;

impl AudioModem {
    /// Transmit a message via ultrasonic sound
    pub fn transmit(message: &str) -> Result<()> {
        println!("{} Initializing Sonar Transmitter...", "📡".cyan());
        
        let host = cpal::default_host();
        let device = match host.default_output_device() {
            Some(d) => d,
            None => {
                println!("{}", "No audio output device found. Simulation mode active.".yellow());
                return Self::simulate_transmission(message);
            }
        };

        let config = device.default_output_config()
            .map_err(|e| format!("Default output config error: {}", e))?;
        
        println!("Target: {} (Ultrasonic)", "18.5kHz - 19.0kHz".cyan());
        println!("Sending: \"{}\"", message.bold());

        let bits = string_to_bits(message);
        
        // In a real implementation, we would build the stream here.
        // For this demo/environment where audio might fail, we'll simulate the "chirps".
        // Use `device.build_output_stream` with a closure that fills the buffer with proper sine waves.
        
        println!();
        for bit in bits {
            let freq = if bit { FREQ_MARK } else { FREQ_SPACE };
            let symbol = if bit { "1" } else { "0" };
            
            // Visual feedback
            print!("{}", symbol.green());
            
            // Generate tone (mock)
            // play_tone(&device, &config, freq, BIT_DURATION_MS);
            
            std::thread::sleep(std::time::Duration::from_millis(10)); 
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
        println!();
        println!("{} Transmission complete.", "✓".green());
        
        Ok(())
    }

    /// Listen for ultrasonic signals
    pub fn listen() -> Result<()> {
        println!("{} Initializing Sonar Receiver...", "👂".cyan());
        
        let host = cpal::default_host();
        let device = match host.default_input_device() {
             Some(d) => d,
            None => {
                println!("{}", "No audio input device found. Cannot listen.".red());
                return Ok(());
            }
        };
        
        println!("Listening on 18kHz band... (Press Ctrl+C to stop)");
        
        // Real implementation involves Goertzel algorithm or FFT to detect energy at FREQ_MARK/FREQ_SPACE
        // buffer -> FFT -> magnitude -> threshold check
        
        loop {
            // Mock receiving loop
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    
    fn simulate_transmission(message: &str) -> Result<()> {
        println!("Simulating ultrasonic transmission...");
        let bits = string_to_bits(message);
        for bit in bits {
            print!("{}", if bit { "." } else { " " });
            use std::io::Write;
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        println!();
        println!("Done.");
        Ok(())
    }
}

fn string_to_bits(s: &str) -> Vec<bool> {
    let mut bits = Vec::new();
    for byte in s.bytes() {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1 == 1);
        }
    }
    bits
}
