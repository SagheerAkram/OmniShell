// Interactive Tutorials
use colored::Colorize;
use crate::error::Result;

/// Start interactive tutorial
pub async fn start_tutorial(topic: Option<String>) -> Result<()> {
    if let Some(t) = topic {
        match t.as_str() {
            "basics" => tutorial_basics().await?,
            "security" => tutorial_security().await?,
            "groups" => tutorial_groups().await?,
            "protocols" => tutorial_protocols().await?,
            _ => {
                println!("{}", "Unknown tutorial topic".yellow());
                list_tutorials().await?;
            }
        }
    } else {
        list_tutorials().await?;
    }
    
    Ok(())
}

async fn list_tutorials() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                 INTERACTIVE TUTORIALS                          ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "Available Tutorials:".bold());
    println!();
    println!("1. {} - Getting started with OmniShell", "basics".cyan());
    println!("2. {} - Understanding encryption & security", "security".cyan());
    println!("3. {} - Creating and managing groups", "groups".cyan());
    println!("4. {} - Using different network protocols", "protocols".cyan());
    println!();
    println!("Start a tutorial:");
    println!("  {}", "omnishell tutorial basics".green());
    println!();
    
    Ok(())
}

async fn tutorial_basics() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║              TUTORIAL: GETTING STARTED                         ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "Step 1: Initialize OmniShell".bold());
    println!("  Run: {}", "omnishell init".green());
    println!("  This creates your encryption keys and sets up OmniShell");
    println!();
    
    wait_for_enter("Press Enter to continue...").await;
    
    println!("{}", "Step 2: View Your Identity".bold());
    println!("  Run: {}", "omnishell whoami".green());
    println!("  This shows your public key and fingerprint");
    println!();
    
    wait_for_enter("Press Enter to continue...").await;
    
    println!("{}", "Step 3: Add a Contact".bold());
    println!("  Run: {}", "omnishell add alice omni:THEIR_PUBLIC_KEY".green());
    println!("  Replace THEIR_PUBLIC_KEY with your contact's actual key");
    println!();
    
    wait_for_enter("Press Enter to continue...").await;
    
    println!("{}", "Step 4: Send a Message".bold());
    println!("  Run: {}", "omnishell msg @alice \"Hello!\"".green());
    println!("  Your message is encrypted end-to-end");
    println!();
    
    wait_for_enter("Press Enter to continue...").await;
    
    println!("{}", "✓ Tutorial Complete!".green().bold());
    println!();
    println!("Next steps:");
    println!("  • Try: {}", "omnishell help".cyan());
    println!("  • Learn about security: {}", "omnishell tutorial security".cyan());
    println!();
    
    Ok(())
}

async fn tutorial_security() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║           TUTORIAL: SECURITY & ENCRYPTION                      ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "OmniShell Security Features:".bold());
    println!();
    println!("1. {} - All messages encrypted with AES-256-GCM", "End-to-End Encryption".green());
    println!("2. {} - Keys rotate automatically", "Perfect Forward Secrecy".green());
    println!("3. {} - Verify contact fingerprints", "Key Verification".green());
    println!("4. {} - Anonymous routing available", "Tor/I2P Support".green());
    println!();
    
    wait_for_enter("Press Enter to continue...").await;
    
    println!("{}", "Verifying Contacts:".bold());
    println!("  Run: {}", "omnishell verify @alice".green());
    println!("  Compare fingerprints out-of-band (phone call, in person)");
    println!();
    
    wait_for_enter("Press Enter to continue...").await;
    
    println!("{}", "Using Stealth Mode:".bold());
    println!("  Run: {}", "omnishell msg @alice \"Secret\" --stealth".green());
    println!("  Uses ChaCha20-Poly1305 + Tor routing");
    println!();
    
    println!("{}", "✓ Security Tutorial Complete!".green().bold());
    println!();
    
    Ok(())
}

async fn tutorial_groups() -> Result<()> {
    println!("{}", "Tutorial: Group Chat".bold());
    println!();
    println!("Create a group:");
    println!("  {}", "omnishell group create team @alice @bob".green());
    println!();
    println!("Send to group:");
    println!("  {}", "omnishell group msg team \"Hello team!\"".green());
    println!();
    Ok(())
}

async fn tutorial_protocols() -> Result<()> {
    println!("{}", "Tutorial: Network Protocols".bold());
    println!();
    println!("Available protocols:");
    println!("  • P2P - Direct peer-to-peer");
    println!("  • Tor - Anonymous .onion routing");
    println!("  • I2P - Garlic routing");
    println!("  • LoRa - Long-range mesh");
    println!("  • Bluetooth - Nearby devices");
    println!();
    Ok(())
}

async fn wait_for_enter(prompt: &str) {
    use std::io::{self, Write};
    print!("{}", prompt.bright_black());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
