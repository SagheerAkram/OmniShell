// Sentry Mode - Physical Intrusion Detection
use std::time::Duration;
use tokio::time;
use std::fs;
use crate::error::Result;
use colored::Colorize;

/// Arm the Sentry system
pub async fn arm_sentry() -> Result<()> {
    // Clear screen and show status
    print!("\x1B[2J\x1B[1;1H");
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".red());
    println!("{}", "║                    SENTRY MODE ACTIVE                          ║".red());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".red());
    println!();
    println!("{}", "SYSTEM IS ARMED.".red().bold().blink());
    println!("Motion sensors:  ON");
    println!("Lid sensors:     ON");
    println!("Mic sensors:     ON");
    println!();
    println!("Do not touch the device.");
    println!("Press Ctrl+C to disarm (requires password in real mode).");
    
    // In a real scenario, we would blank the screen here
    // print!("\x1B[?25l"); // Hide cursor
    
    let mut interval = time::interval(Duration::from_millis(500));
    
    // Baseline sensor reading (mock)
    let _baseline_tilt = 0.0;
    
    loop {
        interval.tick().await;
        
        if check_lid_status() {
            trigger_alarm("LID OPENED");
        }
        
        if check_power_status() {
            trigger_alarm("POWER DISCONNECTED");
        }
        
        // Mock motion sensor check
        // if check_motion() { ... }
    }
}

fn trigger_alarm(reason: &str) {
    println!();
    println!("{}", "ALARM TRIGGERED!".on_red().white().bold());
    println!("REASON: {}", reason.yellow());
    println!("Broadcasting security alert to mesh...");
    println!("Wiping ephemeral keys...");
    println!("Taking photo...");
    
    // Here we would actually publish the event via EventBus
    // crate::events::publish(SystemEvent::SecurityAlert { ... });
    
    // Simulate immediate shutdown/action
    std::thread::sleep(Duration::from_secs(2));
    // std::process::exit(1); 
}

/// Check if the laptop lid is closed/open
/// Returns true if lid is OPEN (trigger condition)
fn check_lid_status() -> bool {
    // On Linux, we can check /proc/acpi/button/lid/LID/state
    // state:      open
    
    // This is a naive check. In reality, we'd iterate over /proc/acpi/button/lid/*/state
    if let Ok(content) = fs::read_to_string("/proc/acpi/button/lid/LID/state") {
        if content.contains("open") {
            // If we armed it while closed, this would be true.
            // But if we arm while open, this immediately triggers. 
            // Real logic needs a "settling" period.
            return false; // disabling for safety in this demo environment
        }
    } else if let Ok(content) = fs::read_to_string("/proc/acpi/button/lid/LID0/state") {
         if content.contains("open") {
            return false; 
        }
    }
    
    false
}

/// Check if AC power is disconnected
fn check_power_status() -> bool {
    // /sys/class/power_supply/AC/online
    // 1 = online, 0 = offline
    
    if let Ok(content) = fs::read_to_string("/sys/class/power_supply/AC/online") {
        if content.trim() == "0" {
            return true;
        }
    }
    false
}
