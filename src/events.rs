// Event Bus Architecture for OmniShell
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use colored::Colorize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    // Communication Events
    MessageReceived {
        sender: String,
        content: String,
        protocol: String,
    },
    MessageSent {
        recipient: String,
        status: String,
    },
    
    // Tactical Events (Blue Force Tracking)
    PositionUpdate {
        entity_id: String,
        callsign: String,
        lat: f64,
        lon: f64,
        status: EntityStatus, // OK, MIA, DURESS
    },
    
    // SIGINT Events
    SignalDetected {
        frequency: f64, // MHz
        rssi: i16,      // dBm
        signal_type: String, // LoRa, BT, Unknown
    },
    JammingAlert {
        frequency: f64,
        intensity: f64, // 0.0 - 1.0
        confidence: f64,
    },
    
    // System Status
    NetworkStatusChange {
        protocol: String,
        connected: bool,
    },
    SecurityAlert {
        level: SecurityLevel,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityStatus {
    Ok,
    Moving,
    Engaged,
    MIA,
    Duress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical, // Immediate wipe recommended
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100); // Buffer up to 100 events
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: SystemEvent) {
        // We ignore the error if there are no subscribers
        let _ = self.sender.send(event.clone());
        
        // Log critical events to stdout for now (until TUI is ready)
        match event {
            SystemEvent::JammingAlert { frequency, intensity, .. } => {
                println!("{} JAMMING DETECTED on {:.1} MHz (Intensity: {:.0}%)", 
                    "⚠️  CRITICAL:".red().bold().blink(),
                    frequency,
                    intensity * 100.0
                );
            },
            SystemEvent::SecurityAlert { level, message } => {
                if level == SecurityLevel::Critical {
                    println!("{} SECURITY ALERT: {}", "☠️".red().bold(), message.red());
                } else {
                    println!("{} SECURITY: {}", "🛡️".yellow(), message);
                }
            },
            _ => {}
        }
    }
}

// Global instance (simplified for this architecture)
lazy_static::lazy_static! {
    pub static ref BUS: Arc<EventBus> = Arc::new(EventBus::new());
}

/// Helper function to publish events globally
pub fn publish(event: SystemEvent) {
    BUS.publish(event);
}
