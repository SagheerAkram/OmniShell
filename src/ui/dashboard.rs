// Blue Force Tracking TUI Dashboard
use colored::Colorize;
use std::time::Duration;
use tokio::time;
use std::collections::HashMap;

use crate::events::{self, SystemEvent, EntityStatus};
use crate::error::Result;

// Simulation struct for entities on the map
#[derive(Debug, Clone)]
struct Entity {
    callsign: String,
    lat: f64,
    lon: f64,
    status: EntityStatus,
    last_update: std::time::Instant,
}

pub async fn run_dashboard() -> Result<()> {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".green());
    println!("{}", "║          TACTICAL SITUATIONAL AWARENESS (TSA)                  ║".green());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".green());
    println!("Type Ctrl+C to exit dashboard");
    println!();
    
    let mut entities: HashMap<String, Entity> = HashMap::new();
    
    // Subscribe to events
    let bus = events::BUS.clone();
    let mut rx = bus.subscribe();
    
    // Simulate some initial data
    entities.insert("Alpha-1".to_string(), Entity {
        callsign: "Alpha-1".to_string(),
        lat: 34.0522,
        lon: -118.2437,
        status: EntityStatus::Ok,
        last_update: std::time::Instant::now(),
    });
    
    entities.insert("Bravo-2".to_string(), Entity {
        callsign: "Bravo-2".to_string(),
        lat: 34.0525,
        lon: -118.2430,
        status: EntityStatus::Moving,
        last_update: std::time::Instant::now(),
    });
    
    // Refresh loop
    let mut interval = time::interval(Duration::from_secs(1));
    
    loop {
        tokio::select! {
            // Handle incoming events
            Ok(event) = rx.recv() => {
                match event {
                    SystemEvent::PositionUpdate { entity_id, callsign, lat, lon, status } => {
                        entities.insert(entity_id, Entity {
                            callsign,
                            lat,
                            lon,
                            status,
                            last_update: std::time::Instant::now(),
                        });
                    }
                    SystemEvent::JammingAlert { frequency, intensity, .. } => {
                        println!("\x1B[H\x1B[5B"); // Move cursor to alert area
                        println!("{} JAMMING DETECTED: {:.1} MHz ({:.0}%)", 
                            "⚠️".red().blink(), frequency, intensity * 100.0);
                    }
                    _ => {}
                }
            }
            
            // Refresh display
            _ = interval.tick() => {
                render_map(&entities);
            }
        }
    }
}

fn render_map(entities: &HashMap<String, Entity>) {
    // Simulating a map render by printing a grid
    // In a real implementation, we would use ratatui to draw this properly
    
    println!("\x1B[H\x1B[6B"); // Move cursor below header
    
    println!("  STATUS REPORT:");
    println!("  ──────────────");
    
    for entity in entities.values() {
        let status_icon = match entity.status {
            EntityStatus::Ok => "🟢".green(),
            EntityStatus::Moving => "🔵".blue(),
            EntityStatus::Engaged => "🔴".red().blink(),
            EntityStatus::MIA => "⚫".white().dimmed(),
            EntityStatus::Duress => "🆘".yellow().blink(),
        };
        
        println!("  {} {:<10} [{:.4}, {:.4}] {}", 
            status_icon, 
            entity.callsign, 
            entity.lat, 
            entity.lon,
            format!("{:?}", entity.status).bright_black()
        );
    }
    
    println!();
    println!("  Active Nodes: {}", entities.len());
    println!("  Mesh Integrity: 100%");
}
