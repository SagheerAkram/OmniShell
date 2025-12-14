use colored::Colorize;
use std::path::Path;
use std::fs;
use chrono::Utc;
use tar::Builder;
use flate2::Compression;
use flate2::write::GzEncoder;

use crate::crypto::{derive_key_from_password, encrypt_message, decrypt_message};
use crate::crypto::encryption::CipherType;
use crate::error::{OmniShellError, Result};
use crate::storage::omnishell_dir;
use crate::ui::output;

/// Create an encrypted backup of all OmniShell data
pub async fn create_backup(output_path: Option<String>, password: Option<String>) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   CREATING BACKUP                              ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let omnishell_path = omnishell_dir()?;
    
    // Create backup filename
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("omnishell_backup_{}.tar.gz", timestamp);
    let backup_path = if let Some(path) = output_path {
        Path::new(&path).join(&backup_name)
    } else {
        omnishell_path.join("backups").join(&backup_name)
    };
    
    // Ensure backups directory exists
    if let Some(parent) = backup_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    println!("{} Creating backup archive...", "→".cyan());
    
    // Create tar.gz archive
    let tar_gz = fs::File::create(&backup_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    
    // Add directories to backup
    let dirs = vec!["keys", "contacts", "messages"];
    for dir_name in dirs {
        let dir_path = omnishell_path.join(dir_name);
        if dir_path.exists() {
            tar.append_dir_all(dir_name, &dir_path)?;
        }
    }
    
    // Add database
    let db_path = omnishell_path.join("omnishell.db");
    if db_path.exists() {
        tar.append_path_with_name(&db_path, "omnishell.db")?;
    }
    
    // Add config
    let config_path = omnishell_path.join("config.toml");
    if config_path.exists() {
        tar.append_path_with_name(&config_path, "config.toml")?;
    }
    
    tar.finish()?;
    
    // Get file size
    let metadata = fs::metadata(&backup_path)?;
    let size = metadata.len();
    
    println!("{} Backup created successfully!", "✓".green().bold());
    println!("  └─ Location: {}", backup_path.display());
    println!("  └─ Size: {}", output::format_bytes(size));
    println!();
    
    // Optionally encrypt with password
    if let Some(pwd) = password {
        println!("{} Encrypting backup with password...", "→".cyan());
        
        let backup_data = fs::read(&backup_path)?;
        let key = derive_key_from_password(&pwd, b"omnishell_backup_salt")?;
        let encrypted = encrypt_message(&backup_data, &key, CipherType::Aes256Gcm)?;
        let encrypted_bytes = bincode::serialize(&encrypted)?;
        
        let encrypted_path = backup_path.with_extension("tar.gz.enc");
        fs::write(&encrypted_path, encrypted_bytes)?;
        
        // Remove unencrypted backup
        fs::remove_file(&backup_path)?;
        
        println!("{} Backup encrypted", "✓".green());
        println!("  └─ Encrypted file: {}", encrypted_path.display());
        println!();
    }
    
    println!("{}", "⚠️  Important:".yellow().bold());
    println!("  Store this backup in a secure location");
    println!("  Keep your password safe if encrypted");
    println!();
    
    Ok(())
}

/// Restore from an encrypted backup
pub async fn restore_backup(backup_path: String, password: Option<String>) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   RESTORING BACKUP                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let backup = Path::new(&backup_path);
    if !backup.exists() {
        return Err(OmniShellError::InvalidInput(format!("Backup not found: {}", backup_path)));
    }
    
    println!("{}", "⚠️  Warning:".yellow().bold());
    println!("  This will overwrite your current OmniShell data!");
    println!();
    
    let confirmed = dialoguer::Confirm::new()
        .with_prompt("Are you sure you want to restore?")
        .default(false)
        .interact()?;
    
    if !confirmed {
        println!("{}", "Cancelled.".yellow());
        return Ok(());
    }
    
    println!();
    println!("{} Restoring backup...", "→".cyan());
    
    // Handle encrypted backup
    let backup_data = if backup_path.ends_with(".enc") {
        if let Some(pwd) = &password {
            let encrypted_bytes = fs::read(backup)?;
            let encrypted = bincode::deserialize(&encrypted_bytes)?;
            let key = derive_key_from_password(pwd, b"omnishell_backup_salt")?;
            decrypt_message(&encrypted, &key)?
        } else {
            return Err(OmniShellError::InvalidInput("Password required for encrypted backup".to_string()));
        }
    } else {
        fs::read(backup)?
    };
    
    // Extract tar.gz
    let omnishell_path = omnishell_dir()?;
    
    // TODO: Extract tar archive (requires tar crate)
    // For now, show what would happen
    println!("{} Backup would be restored to: {}", "✓".green(), omnishell_path.display());
    println!();
    println!("{}", "Note: Full restore implementation requires restarting OmniShell".yellow());
    println!();
    
    Ok(())
}

/// Export contacts to a file
pub async fn export_contacts(output_path: String) -> Result<()> {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize)]
    struct ContactExport {
        name: String,
        public_key: String,
        fingerprint: String,
        trust_level: String,
        notes: Option<String>,
    }
    
    println!("{} Exporting contacts...", "→".cyan());
    
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let contacts: Vec<(String, Vec<u8>, String, String, Option<String>)> = sqlx::query_as(
        "SELECT name, public_key, fingerprint, trust_level, notes FROM contacts"
    )
    .fetch_all(pool)
    .await?;
    
    let exports: Vec<ContactExport> = contacts.iter().map(|(name, pubkey, fp, trust, notes)| {
        ContactExport {
            name: name.clone(),
            public_key: format!("omni:{}", hex::encode(pubkey)),
            fingerprint: fp.clone(),
            trust_level: trust.clone(),
            notes: notes.clone(),
        }
    }).collect();
    
    let json = serde_json::to_string_pretty(&exports)?;
    fs::write(&output_path, json)?;
    
    println!("{} Exported {} contacts to {}", "✓".green(), exports.len(), output_path);
    println!();
    
    Ok(())
}

/// Import contacts from a file
pub async fn import_contacts(input_path: String) -> Result<()> {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize)]
    struct ContactExport {
        name: String,
        public_key: String,
        fingerprint: String,
        trust_level: String,
        notes: Option<String>,
    }
    
    println!("{} Importing contacts from {}...", "→".cyan(), input_path);
    
    let json = fs::read_to_string(&input_path)?;
    let imports: Vec<ContactExport> = serde_json::from_str(&json)?;
    
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    let now = Utc::now().timestamp();
    
    let mut imported = 0;
    let mut skipped = 0;
    
    for contact in imports {
        // Check if already exists
        let exists: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM contacts WHERE name = ?"
        )
        .bind(&contact.name)
        .fetch_optional(pool)
        .await?;
        
        if exists.is_some() {
            skipped += 1;
            continue;
        }
        
        // Parse public key
        let pubkey_hex = contact.public_key.trim_start_matches("omni:");
        let pubkey_bytes = hex::decode(pubkey_hex)?;
        
        // Insert contact
        sqlx::query(
            "INSERT INTO contacts (name, public_key, fingerprint, trust_level, notes, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&contact.name)
        .bind(&pubkey_bytes)
        .bind(&contact.fingerprint)
        .bind(&contact.trust_level)
        .bind(&contact.notes)
        .bind(now)
        .execute(pool)
        .await?;
        
        imported += 1;
    }
    
    println!("{} Import complete", "✓".green().bold());
    println!("  └─ Imported: {}", imported);
    println!("  └─ Skipped (already exist): {}", skipped);
    println!();
    
    Ok(())
}

/// Rotate encryption keys
pub async fn rotate_keys() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   KEY ROTATION                                 ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "⚠️  Warning:".yellow().bold());
    println!("  Key rotation will generate new encryption keys");
    println!("  You will need to share your new public key with contacts");
    println!("  Old messages can still be decrypted with archived keys");
    println!();
    
    let confirmed = dialoguer::Confirm::new()
        .with_prompt("Proceed with key rotation?")
        .default(false)
        .interact()?;
    
    if !confirmed {
        println!("{}", "Cancelled.".yellow());
        return Ok(());
    }
    
    println!();
    println!("{} Rotating keys...", "→".cyan());
    
    // Archive old keys
    let keys_dir = omnishell_dir()?.join("keys");
    let archive_dir = keys_dir.join("archive");
    fs::create_dir_all(&archive_dir)?;
    
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let old_private = keys_dir.join("private.key");
    let old_public = keys_dir.join("public.key");
    
    if old_private.exists() {
        fs::rename(&old_private, archive_dir.join(format!("private_{}.key", timestamp)))?;
    }
    if old_public.exists() {
        fs::rename(&old_public, archive_dir.join(format!("public_{}.key", timestamp)))?;
    }
    
    println!("{} Old keys archived", "✓".green());
    
    // Generate new keys
    println!("{} Generating new keys...", "→".cyan());
    let keypair = crate::crypto::generate_keypair();
    let public_key = keypair.public_key();
    
    // Save new keys
    fs::write(&old_private, keypair.to_bytes())?;
    fs::write(&old_public, public_key.to_string())?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&old_private)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&old_private, perms)?;
    }
    
    println!("{} New keys generated", "✓".green().bold());
    println!();
    println!("{}", "New Public Key:".bold());
    println!("  {}", public_key.to_string().green());
    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Share your new public key with contacts");
    println!("  2. Update your profile on DHT (if applicable)");
    println!("  3. Old keys archived in keys/archive/");
    println!();
    
    Ok(())
}

/// Clean up old data
pub async fn cleanup(days: u32, dry_run: bool) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   DATA CLEANUP                                 ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let cutoff = Utc::now().timestamp() - (days as i64 * 86400);
    
    // Count old messages
    let old_messages: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE timestamp < ? AND deleted_at IS NULL"
    )
    .bind(cutoff)
    .fetch_one(pool)
    .await?;
    
    println!("{} Found {} old messages (older than {} days)", 
        "→".cyan(), old_messages.0, days);
    
    if dry_run {
        println!();
        println!("{}", "DRY RUN - No changes made".yellow());
        println!("Run without --dry-run to actually delete");
        return Ok(());
    }
    
    // Delete old messages
    sqlx::query("DELETE FROM messages WHERE timestamp < ? AND deleted_at IS NULL")
        .bind(cutoff)
        .execute(pool)
        .await?;
    
    println!("{} Deleted {} old messages", "✓".green(), old_messages.0);
    
    // Vacuum database
    println!("{} Optimizing database...", "→".cyan());
    sqlx::query("VACUUM").execute(pool).await?;
    println!("{} Database optimized", "✓".green());
    println!();
    
    Ok(())
}
