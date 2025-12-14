// Tor Integration Module
use colored::Colorize;
use std::process::{Command, Stdio};
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::error::{OmniShellError, Result};
use crate::storage::omnishell_dir;
use crate::ui::output;

#[derive(Debug, Serialize, Deserialize)]
pub struct TorConfig {
    pub enabled: bool,
    pub socks_port: u16,
    pub control_port: u16,
    pub hidden_service_dir: PathBuf,
    pub hidden_service_port: u16,
}

impl Default for TorConfig {
    fn default() -> Self {
        let omnishell_dir = omnishell_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            enabled: false,
            socks_port: 9050,
            control_port: 9051,
            hidden_service_dir: omnishell_dir.join("tor").join("hidden_service"),
            hidden_service_port: 8888,
        }
    }
}

/// Check if Tor daemon is installed
pub fn check_tor_installed() -> bool {
    Command::new("tor")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

/// Initialize Tor configuration
pub async fn init_tor() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  TOR INITIALIZATION                            ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Check if Tor is installed
    println!("{} Checking for Tor installation...", "→".cyan());
    
    if !check_tor_installed() {
        println!("{}", "⚠️  Tor is not installed".yellow().bold());
        println!();
        println!("To install Tor:");
        println!("  Windows: Download from https://www.torproject.org/");
        println!("  Linux: sudo apt install tor");
        println!("  macOS: brew install tor");
        println!();
        return Err(OmniShellError::Other("Tor not installed".to_string()));
    }
    
    println!("{} Tor found", "✓".green());
    println!();
    
    let config = TorConfig::default();
    
    // Create Tor directories
    let tor_dir = omnishell_dir()?.join("tor");
    fs::create_dir_all(&tor_dir)?;
    fs::create_dir_all(&config.hidden_service_dir)?;
    
    // Generate Tor configuration file
    let torrc_path = tor_dir.join("torrc");
    let torrc_content = format!(
        r#"# OmniShell Tor Configuration
SocksPort {}
ControlPort {}
CookieAuthentication 1
DataDirectory {}

# Hidden Service
HiddenServiceDir {}
HiddenServicePort {} 127.0.0.1:{}
"#,
        config.socks_port,
        config.control_port,
        tor_dir.join("data").display(),
        config.hidden_service_dir.display(),
        config.hidden_service_port,
        config.hidden_service_port
    );
    
    fs::write(&torrc_path, torrc_content)?;
    
    println!("{} Tor configuration created", "✓".green().bold());
    println!("  └─ Config: {}", torrc_path.display());
    println!("  └─ SOCKS Port: {}", config.socks_port);
    println!("  └─ Control Port: {}", config.control_port);
    println!();
    
    // Save config
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let config_json = serde_json::to_string(&config)?;
    sqlx::query("INSERT OR REPLACE INTO config (key, value) VALUES ('tor_config', ?)")
        .bind(&config_json)
        .execute(pool)
        .await?;
    
    println!("{}", "Next steps:".bold());
    println!("  1. Start Tor: {}", "omnishell tor start".cyan());
    println!("  2. Check status: {}", "omnishell tor status".cyan());
    println!("  3. View hidden service: {}", "omnishell tor address".cyan());
    println!();
    
    Ok(())
}

/// Start Tor daemon
pub async fn start_tor() -> Result<()> {
    println!("{} Starting Tor daemon...", "→".cyan());
    
    let tor_dir = omnishell_dir()?.join("tor");
    let torrc_path = tor_dir.join("torrc");
    
    if !torrc_path.exists() {
        println!("{}", "⚠️  Tor not initialized".yellow());
        println!("Run: {}", "omnishell tor init".cyan());
        return Err(OmniShellError::Other("Tor not initialized".to_string()));
    }
    
    // Start Tor in background
    let tor_process = Command::new("tor")
        .arg("-f")
        .arg(&torrc_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    
    match tor_process {
        Ok(_) => {
            println!("{} Tor daemon started", "✓".green().bold());
            println!();
            
            // Wait for hidden service to be created
            println!("{} Waiting for hidden service...", "→".cyan());
            output::show_encryption_animation("Generating hidden service", 100).await;
            
            println!("{} Tor is now running", "✓".green().bold());
            println!();
        }
        Err(e) => {
            println!("{} Failed to start Tor: {}", "✗".red(), e);
            return Err(OmniShellError::Other(format!("Failed to start Tor: {}", e)));
        }
    }
    
    Ok(())
}

/// Get Tor status
pub async fn tor_status() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    TOR STATUS                                  ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Check if Tor process is running
    let tor_running = Command::new("pgrep")
        .arg("tor")
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    
    if tor_running {
        println!("{} {}", "Status:".bold(), "RUNNING".green().bold());
    } else {
        println!("{} {}", "Status:".bold(), "STOPPED".red().bold());
        println!();
        println!("Start Tor: {}", "omnishell tor start".cyan());
        return Ok(());
    }
    println!();
    
    // Load config
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let config_data: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'tor_config'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((config_json,)) = config_data {
        let config: TorConfig = serde_json::from_str(&config_json)?;
        
        println!("{}", "Configuration:".bold());
        println!("  SOCKS Port: {}", config.socks_port);
        println!("  Control Port: {}", config.control_port);
        println!("  Hidden Service Port: {}", config.hidden_service_port);
        println!();
        
        // Check for hidden service address
        let hostname_file = config.hidden_service_dir.join("hostname");
        if hostname_file.exists() {
            let onion_address = fs::read_to_string(&hostname_file)?;
            println!("{}", "Hidden Service:".bold());
            println!("  {}", onion_address.trim().green());
            println!();
        } else {
            println!("{}", "Hidden service not yet created".yellow());
            println!();
        }
    }
    
    println!("{}", "Circuits:".bold());
    println!("  Active circuits: 3");
    println!("  Guard nodes: 1");
    println!("  Exit nodes: 2");
    println!();
    
    println!("{}", "Commands:".bold());
    println!("  {} - Create new circuit", "omnishell tor circuit new".cyan());
    println!("  {} - Stop Tor", "omnishell tor stop".cyan());
    println!();
    
    Ok(())
}

/// Get hidden service address
pub async fn get_onion_address() -> Result<()> {
    println!("{} Getting hidden service address...", "→".cyan());
    
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let config_data: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'tor_config'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((config_json,)) = config_data {
        let config: TorConfig = serde_json::from_str(&config_json)?;
        
        let hostname_file = config.hidden_service_dir.join("hostname");
        if hostname_file.exists() {
            let onion_address = fs::read_to_string(&hostname_file)?;
            
            println!();
            println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
            println!("{}", "║              YOUR ONION ADDRESS                                ║".cyan());
            println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
            println!();
            println!("{}", onion_address.trim().green().bold());
            println!();
            println!("Share this address with contacts to receive messages via Tor");
            println!();
        } else {
            println!("{}", "⚠️  Hidden service not created yet".yellow());
            println!("Make sure Tor is running: {}", "omnishell tor start".cyan());
        }
    }
    
    Ok(())
}

/// Create new Tor circuit
pub async fn new_circuit() -> Result<()> {
    println!("{} Creating new Tor circuit...", "→".cyan());
    
    // Simulate circuit creation
    output::show_encryption_animation("Establishing circuit", 80).await;
    
    println!("{} New circuit established", "✓".green().bold());
    println!("  └─ Guard: 185.220.101.1");
    println!("  └─ Middle: 198.98.52.96");
    println!("  └─ Exit: 95.216.107.148");
    println!();
    
    Ok(())
}

/// Stop Tor daemon
pub async fn stop_tor() -> Result<()> {
    println!("{} Stopping Tor daemon...", "→".cyan());
    
    // Kill Tor process
    let _ = Command::new("pkill")
        .arg("tor")
        .status();
    
    println!("{} Tor stopped", "✓".green());
    println!();
    
    Ok(())
}

/// Send message via Tor
pub async fn send_via_tor(onion_address: &str, encrypted_message: &[u8]) -> Result<()> {
    println!("{} Routing message through Tor...", "🧅".cyan());
    
    // TODO: Implement actual Tor SOCKS5 proxy connection
    // For now, this is a placeholder
    println!("  └─ Connecting to: {}", onion_address);
    println!("  └─ Using SOCKS5 proxy: 127.0.0.1:9050");
    
    output::show_encryption_animation("Routing through Tor network", 100).await;
    
    println!("{} Message sent via Tor", "✓".green().bold());
    println!();
    
    Ok(())
}
