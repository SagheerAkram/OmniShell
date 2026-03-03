use crate::error::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::fs;

pub async fn run_dashboard() -> Result<()> {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen
    
    let logo = r#"
   ____                  _ _____ __         ____ 
  / __ \____ ___  ____  (_) ___// /_  ___  / / / 
 / / / / __ `__ \/ __ \/ /\__ \/ __ \/ _ \/ / /  
/ /_/ / / / / / / / / / /___/ / / / /  __/ / /   
\____/_/ /_/ /_/_/ /_/_//____/_/ /_/\___/_/_/    
"#;
    println!("{}", logo.cyan().bold());
    println!("{}", "================================================================".blue());
    println!("{}", "            OMNISHELL SECURE OFFLINE TERMINAL                   ".bright_white().bold());
    println!("{}", "================================================================".blue());
    
    let selections = &[
        "📡 Start Background Node (Wi-Fi/BLE Daemon)",
        "🔊 Test Sonar Air-Gap (Audio Modem)",
        "💾 Export Physical ID Pack (To USB)",
        "📥 Import ID Pack (From USB)",
        "💬 Help & Docs",
        "🚪 Exit OmniShell",
    ];

    loop {
        println!();
        
        let unread_count = crate::messaging::get_unread_count().await.unwrap_or(0);
        let stealth_read_label = format!("🕵️ Read Unread {}({}){} Stealth mode", "(".cyan(), unread_count.to_string().yellow().bold(), ")".cyan());

        let selections = vec![
            "📡 Start Background Node (Wi-Fi/BLE Daemon)".to_string(),
            stealth_read_label,
            "🔊 Transmit ID via Audio (Sonar Modem)".to_string(),
            "👂 Receive ID via Audio (Sonar Modem)".to_string(),
            "💾 Export Physical ID Pack (To USB)".to_string(),
            "📥 Import ID Pack (From USB)".to_string(),
            "💬 Help & Docs".to_string(),
            "🚪 Exit OmniShell".to_string(),
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}", "TACTICAL INTERFACE".red().bold()))
            .default(0)
            .items(&selections)
            .interact()?;

        match selection {
            0 => {
                println!("{}", "Initializing OmniShell Daemon... (Ctrl+C to stop)".yellow());
                let _ = tokio::join!(
                    crate::network::p2p::start_listener(),
                    crate::network::bluetooth::init_bluetooth()
                    // Sonar modem removed from background nodes as it's meant to be triggered interactively
                );
            },
            1 => {
                println!("{}", "Checking for unread messages...".cyan());
                if let Err(e) = crate::messaging::stealth_read_unread_all().await {
                    println!("{} {}", "Failed to read messages:".red(), e);
                }
            },
            2 => {
                println!("\n{}", "--- SONAR ID TRANSMISSION ---".magenta().bold());
                match Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your Display Name for broadcast (e.g., Alice's Laptop)")
                    .interact_text() {
                    Ok(name) => {
                        println!("Retrieving local identity...");
                        if let Ok(store) = crate::storage::Storage::init() {
                            if let Ok(keys) = store.get_keypair("default") {
                                let id = format!("omni:{}:{}", hex::encode(keys.public.as_bytes()), name);
                                println!("{}", "Transmitting via ultrasonic audio. Ensure the receiver is listening...".yellow());
                                let _ = crate::network::sonar::AudioModem::transmit(&id);
                            } else {
                                println!("{}", "Identity not initialized. Run init first!".red());
                            }
                        } else {
                            println!("{}", "Storage failed.".red());
                        }
                    },
                    Err(_) => continue,
                }
            },
            3 => {
                println!("\n{}", "--- SONAR ID RECEPTION ---".magenta().bold());
                if let Ok(payload) = crate::network::sonar::AudioModem::listen() {
                    let parts: Vec<&str> = payload.split(':').collect();
                    if parts.len() >= 3 && parts[0] == "omni" {
                        let hex_key = parts[1];
                        let device_name = parts[2..].join(":");
                        println!("{} Received identity from \"{}\" [omni:{}...].", "✓".green(), device_name.cyan(), &hex_key[0..8]);
                        
                        match dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt("Do you want to save this contact?")
                            .default(true)
                            .interact() {
                            Ok(true) => {
                                let contact_name = Input::<String>::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Enter a short alias for this contact")
                                    .default(device_name.clone())
                                    .interact_text()
                                    .unwrap_or(device_name);
                                    
                                let _ = crate::contacts::add(contact_name.clone(), Some(hex_key.to_string()), false, false).await;
                                println!("{}", format!("Successfully added {}!", contact_name).green());
                            },
                            _ => {
                                println!("{}", "Contact discarded.".yellow());
                            }
                        }
                    } else {
                        println!("{}", "Unrecognized audio payload format.".red());
                    }
                }
            },
            4 => {
                println!("\n{}", "--- AIR GAP EXPORT ---".magenta().bold());
                match Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter Export Path (e.g., E:\\MyKey.txt)")
                    .interact_text() {
                    Ok(path) => {
                        println!("Retrieving local identity...");
                        if let Ok(store) = crate::storage::Storage::init() {
                            if let Ok(keys) = store.get_keypair("default") {
                                let id = format!("omni:{}", hex::encode(keys.public.as_bytes()));
                                if fs::write(&path, id).is_ok() {
                                    println!("{}", format!("Successfully secured key to {}!", path).green());
                                    println!("{}", "You may now hand the drive to your ally.".green());
                                } else {
                                    println!("{}", "Failed to write to path.".red());
                                }
                            } else {
                                println!("{}", "Identity not initialized. Run init first!".red());
                            }
                        } else {
                            println!("{}", "Storage failed.".red());
                        }
                    },
                    Err(_) => continue,
                }
            },
            5 => {
                println!("\n{}", "--- AIR GAP IMPORT ---".magenta().bold());
                match Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter Path to Friend's key file (e.g., E:\\FriendKey.txt)")
                    .interact_text() {
                    Ok(path) => {
                        if let Ok(key_data) = fs::read_to_string(&path) {
                            if let Ok(name) = Input::<String>::with_theme(&ColorfulTheme::default())
                                .with_prompt("Enter a Contact Name for this user")
                                .interact_text() {
                                let _ = crate::contacts::add(name.clone(), Some(key_data.trim().to_string()), false, false).await;
                                println!("{}", format!("Successfully added {}!", name).green());
                            }
                        } else {
                            println!("{}", "Failed to read file. Does the path exist?".red());
                        }
                    },
                    Err(_) => continue,
                }
            },
            6 => {
                println!("{}", "To run single-shot commands or interactive chat, run:".yellow());
                println!("omnishell --help");
            },
            7 | _ => {
                println!("Shutting down terminal.");
                break;
            }
        }
    }
    
    Ok(())
}
