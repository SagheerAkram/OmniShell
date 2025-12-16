use colored::Colorize;
use sqlx::SqlitePool;
use chrono::Utc;
use crate::crypto::keys::PublicKey;
use crate::error::{OmniShellError, Result};
use crate::storage::{Storage, omnishell_dir};
use crate::ui::output;
use dialoguer::Confirm;

#[derive(Debug, sqlx::FromRow)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub public_key: Vec<u8>,
    pub fingerprint: String,
    pub trust_level: String,
    pub last_seen: Option<i64>,
    pub created_at: i64,
    pub notes: Option<String>,
}

/// Add a new contact
pub async fn add(name: String, public_key: Option<String>, scan: bool, nearby: bool) -> Result<()> {
    if scan {
        println!("{}", "QR code scanning not yet implemented".yellow());
        return Ok(());
    }
    
    if nearby {
        println!("{}", "Nearby device discovery not yet implemented".yellow());
        return Ok(());
    }
    
    let public_key_str = public_key.ok_or_else(|| {
        OmniShellError::InvalidInput("Public key required (use --scan or --nearby for other methods)".to_string())
    })?;
    
    // Parse and validate public key
    let pub_key = PublicKey::from_string(&public_key_str)?;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    ADDING CONTACT                              ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    // Display contact information
    println!("{}", "Contact Name:".bold());
    println!("  {}", name.green());
    println!();
    
    println!("{}", "Public Key:".bold());
    println!("  {}", public_key_str.bright_blue());
    println!();
    
    println!("{}", "Fingerprint:".bold());
    println!("  {}", pub_key.fingerprint().yellow().bold());
    println!();
    
    println!("{}", "Visual Hash:".bold());
    println!("  {}", pub_key.visual_hash());
    println!();
    
    // Connect to database
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Check if contact already exists
    if contact_exists(pool, &name).await? {
        return Err(OmniShellError::Other(format!("Contact '{}' already exists", name)));
    }
    
    // Save to database
    let fingerprint = pub_key.fingerprint();
    let pub_key_bytes = pub_key.to_bytes();
    let now = Utc::now().timestamp();
    
    sqlx::query(
        "INSERT INTO contacts (name, public_key, fingerprint, trust_level, created_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&name)
    .bind(&pub_key_bytes)
    .bind(&fingerprint)
    .bind("medium")
    .bind(now)
    .execute(pool)
    .await?;
    
    // Save public key to file
    let contacts_dir = omnishell_dir()?.join("contacts");
    let contact_file = contacts_dir.join(format!("{}.key", name));
    std::fs::write(contact_file, public_key_str)?;
    
    // Create message directory
    let messages_dir = omnishell_dir()?.join("messages").join(&name);
    std::fs::create_dir_all(messages_dir)?;
    
    println!("{} Contact '{}' added successfully!", "✓".green().bold(), name.cyan());
    println!();
    println!("You can now send messages to {}:", format!("@{}", name).cyan());
    println!("  {}", format!("omnishell msg @{} \"Hello!\"", name).bright_blue());
    println!();
    
    Ok(())
}

/// List all contacts
pub async fn list(online_only: bool) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contacts = get_all_contacts(pool).await?;
    
    if contacts.is_empty() {
        println!("{}", "No contacts found.".yellow());
        println!();
        println!("Add a contact:");
        println!("  {}", "omnishell add <name> <public_key>".cyan());
        return Ok(());
    }
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                       CONTACTS                                 ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    for contact in &contacts {
        if online_only && contact.last_seen.is_none() {
            continue;
        }
        
        let status = if let Some(last_seen) = contact.last_seen {
            let now = Utc::now().timestamp();
            let diff = now - last_seen;
            
            if diff < 300 { // 5 minutes
                "● Online".green()
            } else if diff < 3600 { // 1 hour
                format!("○ Active {}m ago", diff / 60).yellow()
            } else {
                format!("○ Last seen {}", format_last_seen(last_seen)).bright_black()
            }
        } else {
            "○ Never seen".bright_black()
        };
        
        println!("{} {} {}", 
            format!("@{}", contact.name).cyan().bold(),
            status,
            format!("[{}]", contact.trust_level).bright_black()
        );
        println!("  Fingerprint: {}", contact.fingerprint.yellow());
    }
    
    println!();
    println!("{} total contacts", contacts.len());
    println!();
    
    Ok(())
}

/// Show detailed contact information
pub async fn info(contact: &str) -> Result<()> {
    let name = contact.trim_start_matches('@');
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contact = get_contact_by_name(pool, name).await?;
    
    let pub_key = PublicKey::from_string(&format!("omni:{}", hex::encode(&contact.public_key)))?;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", format!("║  Contact: @{:<50}║", name).cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "Public Key:".bold());
    println!("  omni:{}", hex::encode(&contact.public_key).bright_blue());
    println!();
    
    println!("{}", "Fingerprint:".bold());
    println!("  {}", contact.fingerprint.yellow().bold());
    println!();
    
    println!("{}", "Visual Hash:".bold());
    println!("  {}", pub_key.visual_hash());
    println!();
    
    println!("{}", "Trust Level:".bold());
    let trust_color = match contact.trust_level.as_str() {
        "high" => contact.trust_level.green(),
        "medium" => contact.trust_level.yellow(),
        "low" => contact.trust_level.red(),
        _ => contact.trust_level.normal(),
    };
    println!("  {}", trust_color);
    println!();
    
    println!("{}", "Status:".bold());
    if let Some(last_seen) = contact.last_seen {
        println!("  Last seen: {}", output::format_timestamp(last_seen));
    } else {
        println!("  Never seen");
    }
    println!();
    
    println!("{}", "Added:".bold());
    println!("  {}", output::format_timestamp(contact.created_at));
    println!();
    
    if let Some(notes) = contact.notes {
        println!("{}", "Notes:".bold());
        println!("  {}", notes);
        println!();
    }
    
    // Count messages
    let message_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE contact_id = ?"
    )
    .bind(contact.id)
    .fetch_one(pool)
    .await
    .unwrap_or((0,));
    
    println!("{}", "Statistics:".bold());
    println!("  Messages: {}", message_count.0);
    println!();
    
    Ok(())
}

/// Verify contact's key fingerprint
pub async fn verify(contact: &str) -> Result<()> {
    let name = contact.trim_start_matches('@');
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contact = get_contact_by_name(pool, name).await?;
    
    let pub_key = PublicKey::from_string(&format!("omni:{}", hex::encode(&contact.public_key)))?;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║               KEY FINGERPRINT VERIFICATION                     ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", format!("Verifying key for @{}", name).bold());
    println!();
    
    println!("{}", "Fingerprint (verify this with the contact):".bold());
    println!();
    println!("  {}", contact.fingerprint.yellow().bold().underline());
    println!();
    
    println!("{}", "Visual Hash (for quick comparison):".bold());
    println!();
    println!("  {}", pub_key.visual_hash());
    println!();
    
    println!("{}", "How to verify:".italic());
    println!("  1. Contact {} through a trusted channel", format!("@{}", name).cyan());
    println!("  2. Ask them to run: {}", "omnishell whoami".bright_blue());
    println!("  3. Compare the fingerprints - they must match exactly");
    println!("  4. If they match, you can trust this is the correct key");
    println!();
    
    println!("{}", "⚠️  Warning:".yellow().bold());
    println!("  Never trust a key fingerprint received over an insecure channel!");
    println!("  Always verify in person or via video call.");
    println!();
    
    Ok(())
}

/// Remove a contact
pub async fn remove(contact: &str, delete_history: bool) -> Result<()> {
    let name = contact.trim_start_matches('@');
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Check if contact exists
    if !contact_exists(pool, name).await? {
        return Err(OmniShellError::ContactNotFound(name.to_string()));
    }
    
    // Get contact info
    let contact_info = get_contact_by_name(pool, name).await?;
    
    // Confirm deletion
    println!("{}", "⚠️  Warning:".yellow().bold());
    println!("  You are about to remove contact: {}", format!("@{}", name).cyan());
    
    if delete_history {
        let message_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE contact_id = ?"
        )
        .bind(contact_info.id)
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
        
        println!("  This will also delete {} messages", message_count.0);
    }
    
    println!();
    
    let confirmed = Confirm::new()
        .with_prompt("Are you sure?")
        .default(false)
        .interact()?;
    
    if !confirmed {
        println!("{}", "Cancelled.".yellow());
        return Ok(());
    }
    
    // Delete from database
    if delete_history {
        // Delete messages first (due to foreign key)
        sqlx::query("DELETE FROM messages WHERE contact_id = ?")
            .bind(contact_info.id)
            .execute(pool)
            .await?;
    }
    
    sqlx::query("DELETE FROM contacts WHERE name = ?")
        .bind(name)
        .execute(pool)
        .await?;
    
    // Delete contact file
    let contacts_dir = omnishell_dir()?.join("contacts");
    let contact_file = contacts_dir.join(format!("{}.key", name));
    if contact_file.exists() {
        std::fs::remove_file(contact_file)?;
    }
    
    // Optionally delete message directory
    if delete_history {
        let messages_dir = omnishell_dir()?.join("messages").join(name);
        if messages_dir.exists() {
            std::fs::remove_dir_all(messages_dir)?;
        }
    }
    
    println!();
    println!("{} Contact '{}' removed successfully", "✓".green().bold(), name.cyan());
    println!();
    
    Ok(())
}

// Helper functions

async fn contact_exists(pool: &SqlitePool, name: &str) -> Result<bool> {
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM contacts WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    
    Ok(result.is_some())
}

async fn get_all_contacts(pool: &SqlitePool) -> Result<Vec<Contact>> {
    let contacts = sqlx::query_as::<_, Contact>(
        r#"
        SELECT id, name, public_key, fingerprint, trust_level, last_seen, created_at, notes
        FROM contacts
        ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(contacts)
}

async fn get_contact_by_name(pool: &SqlitePool, name: &str) -> Result<Contact> {
    let contact = sqlx::query_as::<_, Contact>(
        r#"
        SELECT id, name, public_key, fingerprint, trust_level, last_seen, created_at, notes
        FROM contacts
        WHERE name = ?
        "#
    )
    .bind(name)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| OmniShellError::ContactNotFound(name.to_string()))?;
    
    Ok(contact)
}

pub async fn get_contact_public_key(name: &str) -> Result<PublicKey> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contact = get_contact_by_name(pool, name).await?;
    let pub_key_str = format!("omni:{}", hex::encode(&contact.public_key));
    PublicKey::from_string(&pub_key_str)
}

fn format_last_seen(timestamp: i64) -> String {
    let now = Utc::now().timestamp();
    let diff = now - timestamp;
    
    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else if diff < 604800 {
        format!("{}d ago", diff / 86400)
    } else {
        output::format_timestamp(timestamp)
    }
}
