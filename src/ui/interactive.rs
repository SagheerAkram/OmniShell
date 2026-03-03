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
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}", "TACTICAL INTERFACE".red().bold()))
            .default(0)
            .items(&selections[..])
            .interact()?;

        match selection {
            0 => {
                println!("{}", "Initializing OmniShell Daemon... (Ctrl+C to stop)".yellow());
                let _ = tokio::join!(
                    crate::network::p2p::start_listener(),
                    crate::network::bluetooth::init_bluetooth(),
                    crate::network::sonar::AudioModem::new().listen()
                );
            },
            1 => {
                println!("{}", "Testing Sonar Vector...".cyan());
                crate::network::sonar::run_test().await?;
            },
            2 => {
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
            3 => {
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
            4 => {
                println!("{}", "To run single-shot commands or interactive chat, run:".yellow());
                println!("omnishell --help");
            },
            5 | _ => {
                println!("Shutting down terminal.");
                break;
            }
        }
    }
    
    Ok(())
}
