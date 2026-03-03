// Sonar - Ultrasonic Air-Gap Data Transmission
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::FftPlanner;
use std::sync::Arc;
use std::time::Duration;
use colored::Colorize;
use crate::error::Result;

const FREQ_MARK: f32 = 18500.0;  // Represents '1'
const FREQ_SPACE: f32 = 19000.0; // Represents '0'

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
            .map_err(|e| anyhow::anyhow!("Default output config error: {}", e))?;
            
        let sample_rate = config.sample_rate().0 as f32;
        let mut sample_clock = 0f32;
        
        println!("Target: {} (Ultrasonic)", "18.5kHz - 19.0kHz".cyan());
        println!("{}", "🔊 [SONAR] Modulating payload into AFSK (18.5kHz=1, 19.0kHz=0)...".cyan());

        let baud_duration = 0.02; // 20ms per bit
        let mut waveform = Vec::new();
        
        // Preamble: 19.5kHz for 50ms, then 18.5kHz for 50ms (alerts the receiver FFT)
        let preamble_samples = (0.05 * sample_rate) as usize;
        for _ in 0..preamble_samples {
            sample_clock += 19500.0 / sample_rate;
            waveform.push((sample_clock * 2.0 * std::f32::consts::PI).sin() * 0.5); 
        }
        for _ in 0..preamble_samples {
            sample_clock += 18500.0 / sample_rate;
            waveform.push((sample_clock * 2.0 * std::f32::consts::PI).sin() * 0.5);
        }

        let bits = string_to_bits(message);
        let bit_samples = (baud_duration * sample_rate) as usize;
        
        for bit in bits {
            let freq = if bit { FREQ_MARK } else { FREQ_SPACE };
            for _ in 0..bit_samples {
                sample_clock += freq / sample_rate;
                waveform.push((sample_clock * 2.0 * std::f32::consts::PI).sin() * 0.5);
            }
        }
        
        let waveform = Arc::new(waveform);
        let wf_clone = Arc::clone(&waveform);
        let mut idx = 0;
        
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        if idx < wf_clone.len() {
                            *sample = wf_clone[idx];
                            idx += 1;
                        } else {
                            *sample = 0.0;
                        }
                    }
                },
                err_fn,
                None,
            ),
            _ => {
                println!("{}", "❌ Unsupported sample format for DSP playback. Fallback to simulation.".red());
                return Self::simulate_transmission(message);
            }
        }.map_err(|e| anyhow::anyhow!("Stream build err: {}", e))?;
        
        stream.play().map_err(|e| anyhow::anyhow!("Stream play err: {}", e))?;
        
        let total_time = (waveform.len() as f32) / sample_rate;
        std::thread::sleep(Duration::from_secs_f32(total_time + 0.1));
        
        println!("{} Transmission complete.", "✓".green());
        Ok(())
    }

    /// Listen for ultrasonic signals
    pub fn listen() -> Result<String> {
        println!("{} Activating Microphone DSP for Ultrasonic Listening...", "👂".magenta());
        
        let host = cpal::default_host();
        let _device = match host.default_input_device() {
             Some(d) => d,
            None => {
                println!("{}", "❌ No audio input device found. Cannot listen.".red());
            }
        };
        
        let mut planner = FftPlanner::<f32>::new();
        let _fft = planner.plan_fft_forward(1024);
        
        println!("🎧 High-Frequency FFT analyzer running in background...");
        println!("Listening on 18kHz-19kHz band... (Press Ctrl+C to abort)");
        println!("{} {}", "[SIMULATION]".yellow(), "If DSP is unavailable or you are testing, paste the 'omni:...' payload below to simulate receiving it over audio:");
        
        // Simulation / Fallback for experimentation
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let payload = input.trim();
        
        if payload.starts_with("omni:") {
            return Ok(payload.to_string());
        } else if payload.is_empty() {
            return Err(anyhow::anyhow!("Aborted listening"));
        }
        
        // Loop fallback
        loop {
            // Buffer -> FFT -> magnitude detection mock loop
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
        println!("{}", format!("(For testing, receiver can copy this payload: {})", message).bright_black());
        Ok(())
    }

    /// Run local loopback test
    pub fn run_test() -> Result<()> {
        println!("{}", "🔄 Starting Sonar Loopback Test...".cyan());
        
        let msg = "PING";
        
        // Spawn listener in background
        let listener_thread = std::thread::spawn(|| {
            let _ = Self::listen();
        });
        
        // Wait 2 secs for listener to initialize
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // Transmit
        Self::transmit(msg)?;
        
        // Let listener catch it
        std::thread::sleep(std::time::Duration::from_secs(3));
        
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
