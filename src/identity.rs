use colored::Colorize;
use crate::crypto::{generate_keypair, KeyPair};
use crate::crypto::keys::{PublicKey, generate_device_id};
use crate::storage::{ensure_directories, omnishell_dir};
use crate::config::Config;
use crate::ui::output;
use crate::error::{OmniShellError, Result};

pub async fn init(force: bool) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║              OMNISHELL INITIALIZATION                          ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();

    // Check if already initialized
    let keys_dir = omnishell_dir()?.join("keys");
    let private_key_path = keys_dir.join("private.key");
    
    if private_key_path.exists() && !force {
        return Err(OmniShellError::AlreadyInitialized);
    }

    if force && private_key_path.exists() {
        println!("{}", "⚠️  Warning: Re-initializing will overwrite existing keys!".yellow());
        println!("{}", "   Make sure you have backed up your keys if needed.".yellow());
        println!();
    }

    // Create directory structure
    println!("{} Creating directory structure...", "→".cyan());
    ensure_directories()?;
    println!("{} Directories created", "✓".green());
    println!();

    // Generate Ed25519 key pair
    println!("{} Generating Ed25519 key pair...", "→".cyan());
    output::show_encryption_animation("Generating cryptographic keys", 50).await;
    
    let keypair = generate_keypair();
    let public_key = keypair.public_key();
    
    println!("{} Key pair generated", "✓".green());
    println!("  └─ Algorithm: Ed25519");
    println!("  └─ Key size: 256 bits");
    println!();

    // Generate device ID
    println!("{} Generating device identifier...", "→".cyan());
    let device_id = generate_device_id();
    println!("{} Device ID: {}", "✓".green(), device_id);
    println!();

    // Save keys
    println!("{} Saving keys...", "→".cyan());
    save_keypair(&keypair, &public_key)?;
    save_device_id(&device_id)?;
    println!("{} Keys saved securely", "✓".green());
    println!();

    // Initialize configuration
    println!("{} Initializing configuration...", "→".cyan());
    let config = Config::default();
    config.save()?;
    println!("{} Configuration saved", "✓".green());
    println!();

    // Initialize database
    println!("{} Initializing database...", "→".cyan());
    let _storage = crate::storage::Storage::new().await?;
    println!("{} Database initialized", "✓".green());
    println!();

    // Display identity
    println!("{}", "═══════════════════════════════════════════════════════════════".cyan());
    println!("{}", "YOUR IDENTITY".bold());
    println!("{}", "═══════════════════════════════════════════════════════════════".cyan());
    println!();
    
    println!("{}", "Public Key:".bold());
    println!("  {}", public_key.to_string().green());
    println!();
    
    println!("{}", "Fingerprint:".bold());
    println!("  {}", public_key.fingerprint().yellow().bold());
    println!();
    
    println!("{}", "Visual Hash:".bold());
    println!("  {}", public_key.visual_hash());
    println!();

    println!("{}", "Device ID:".bold());
    println!("  {}", device_id.bright_blue());
    println!();

    // Generate QR code
    println!("{} Generating QR code...", "→".cyan());
    generate_qr_code(&public_key)?;
    println!("{} QR code saved to: {}", "✓".green(), 
        omnishell_dir()?.join("keys").join("public_key_qr.png").display());
    println!();

    println!("{}", "═══════════════════════════════════════════════════════════════".cyan());
    println!();
    println!("{}", "✓ OmniShell initialized successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. Share your public key with contacts: {}", "omnishell whoami".cyan());
    println!("  2. Add a contact: {}", "omnishell add <name> <public_key>".cyan());
    println!("  3. Send a message: {}", "omnishell msg @<name> \"Hello!\"".cyan());
    println!();

    Ok(())
}

pub async fn whoami() -> Result<()> {
    let keys_dir = omnishell_dir()?.join("keys");
    let private_key_path = keys_dir.join("private.key");
    
    if !private_key_path.exists() {
        return Err(OmniShellError::NotInitialized);
    }

    // Load keys
    let keypair = load_keypair()?;
    let public_key = keypair.public_key();
    let device_id = load_device_id()?;
    let config = Config::load()?;

    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    YOUR IDENTITY                               ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();

    println!("{}", "Public Key:".bold());
    println!("  {}", public_key.to_string().green());
    println!();

    println!("{}", "Fingerprint (for verification):".bold());
    println!("  {}", public_key.fingerprint().yellow().bold());
    println!();

    println!("{}", "Visual Hash:".bold());
    println!("  {}", public_key.visual_hash());
    println!();

    println!("{}", "Device:".bold());
    println!("  Name: {}", config.identity.device_name);
    println!("  ID: {}", device_id.bright_blue());
    println!();

    if let Some(username) = config.identity.username {
        println!("{}", "Username (DHT):".bold());
        println!("  @{}", username.cyan());
        println!();
    }

    println!("{}", "QR Code:".bold());
    let qr_path = keys_dir.join("public_key_qr.png");
    if qr_path.exists() {
        println!("  Saved at: {}", qr_path.display());
    } else {
        println!("  Generating...");
        generate_qr_code(&public_key)?;
        println!("  Saved at: {}", qr_path.display());
    }
    println!();

    println!("{}", "Share this information with contacts to communicate securely.".italic());
    println!();

    Ok(())
}

fn save_keypair(keypair: &KeyPair, public_key: &PublicKey) -> Result<()> {
    let keys_dir = omnishell_dir()?.join("keys");
    
    // Save private key (with restrictive permissions)
    let private_key_path = keys_dir.join("private.key");
    std::fs::write(&private_key_path, keypair.to_bytes())?;
    
    // Set restrictive permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&private_key_path)?.permissions();
        perms.set_mode(0o600); // Read/write for owner only
        std::fs::set_permissions(&private_key_path, perms)?;
    }
    
    // Save public key
    let public_key_path = keys_dir.join("public.key");
    std::fs::write(&public_key_path, public_key.to_string())?;
    
    Ok(())
}

fn load_keypair() -> Result<KeyPair> {
    let keys_dir = omnishell_dir()?.join("keys");
    let private_key_path = keys_dir.join("private.key");
    
    let bytes = std::fs::read(&private_key_path)?;
    KeyPair::from_bytes(&bytes)
}

fn save_device_id(device_id: &str) -> Result<()> {
    let keys_dir = omnishell_dir()?.join("keys");
    let device_id_path = keys_dir.join("device.id");
    std::fs::write(&device_id_path, device_id)?;
    Ok(())
}

fn load_device_id() -> Result<String> {
    let keys_dir = omnishell_dir()?.join("keys");
    let device_id_path = keys_dir.join("device.id");
    Ok(std::fs::read_to_string(&device_id_path)?.trim().to_string())
}

fn generate_qr_code(public_key: &PublicKey) -> Result<()> {
    use qrcode::QrCode;
    use image::Luma;
    
    let code = QrCode::new(public_key.to_string().as_bytes())
        .map_err(|e| OmniShellError::Other(format!("QR code generation failed: {}", e)))?;
    
    let image = code.render::<Luma<u8>>().build();
    
    let qr_path = omnishell_dir()?.join("keys").join("public_key_qr.png");
    image.save(&qr_path)
        .map_err(|e| OmniShellError::Other(format!("Failed to save QR code: {}", e)))?;
    
    Ok(())
}

pub fn get_keypair() -> Result<KeyPair> {
    load_keypair()
}

#[allow(dead_code)]
pub fn get_public_key() -> Result<PublicKey> {
    let keypair = load_keypair()?;
    Ok(keypair.public_key())
}
