// Desktop Notifications & Alert System
//
// TO ENABLE: Add notification commands to main.rs:
// ```rust
// /// Notification settings
// Notifications {
//     #[command(subcommand)]
//     action: NotificationAction,
// },
// ```
// Wire to notifications::init_notifications(), set_dnd_mode(), etc.

#![allow(dead_code)]

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub sound_enabled: bool,
    pub dnd_mode: bool,
    pub dnd_start: Option<String>,
    pub dnd_end: Option<String>,
    pub muted_contacts: Vec<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sound_enabled: true,
            dnd_mode: false,
            dnd_start: Some("22:00".to_string()),
            dnd_end: Some("08:00".to_string()),
            muted_contacts: Vec::new(),
        }
    }
}

/// Initialize notifications
pub async fn init_notifications() -> Result<()> {
    println!("{} Initializing desktop notifications...", "→".cyan());
    
    let config = NotificationConfig::default();
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let config_json = serde_json::to_string(&config)?;
    sqlx::query("INSERT OR REPLACE INTO config (key, value) VALUES ('notifications_config', ?)")
        .bind(&config_json)
        .execute(pool)
        .await?;
    
    println!("{} Notifications enabled", "✓".green().bold());
    println!();
    
    Ok(())
}

/// Send desktop notification
pub async fn send_notification(title: &str, body: &str, _priority: &str) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let config_data: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'notifications_config'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((config_json,)) = config_data {
        let config: NotificationConfig = serde_json::from_str(&config_json)?;
        
        if !config.enabled || config.dnd_mode {
            return Ok(());
        }
        
        // Platform-specific notification
        #[cfg(target_os = "windows")]
        {
            println!("🔔 {} - {}", title.bold(), body);
        }
        
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("notify-send")
                .arg(title)
                .arg(body)
                .spawn();
        }
        
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("osascript")
                .arg("-e")
                .arg(format!("display notification \"{}\" with title \"{}\"", body, title))
                .spawn();
        }
    }
    
    Ok(())
}

/// Enable/disable DND mode
pub async fn set_dnd_mode(enabled: bool) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let config_data: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'notifications_config'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((config_json,)) = config_data {
        let mut config: NotificationConfig = serde_json::from_str(&config_json)?;
        config.dnd_mode = enabled;
        
        let updated_json = serde_json::to_string(&config)?;
        sqlx::query("UPDATE config SET value = ? WHERE key = 'notifications_config'")
            .bind(&updated_json)
            .execute(pool)
            .await?;
        
        if enabled {
            println!("{} Do Not Disturb mode enabled", "✓".green());
        } else {
            println!("{} Do Not Disturb mode disabled", "✓".green());
        }
    }
    
    Ok(())
}

/// Mute/unmute contact
pub async fn mute_contact(contact: String, mute: bool) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let config_data: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'notifications_config'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((config_json,)) = config_data {
        let mut config: NotificationConfig = serde_json::from_str(&config_json)?;
        
        if mute {
            if !config.muted_contacts.contains(&contact) {
                config.muted_contacts.push(contact.clone());
            }
            println!("{} Muted {}", "✓".green(), contact);
        } else {
            config.muted_contacts.retain(|c| c != &contact);
            println!("{} Unmuted {}", "✓".green(), contact);
        }
        
        let updated_json = serde_json::to_string(&config)?;
        sqlx::query("UPDATE config SET value = ? WHERE key = 'notifications_config'")
            .bind(&updated_json)
            .execute(pool)
            .await?;
    }
    
    Ok(())
}
