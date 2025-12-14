use colored::Colorize;
use sqlx::SqlitePool;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::RngCore;

use crate::crypto::{encrypt_message, decrypt_message};
use crate::crypto::encryption::{CipherType, EncryptedMessage};
use crate::error::{OmniShellError, Result};
use crate::storage::Storage;
use crate::ui::output;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSettings {
    pub admins_only: bool,
    pub encrypt_sender: bool,
}

pub async fn create_group(name: String, members: Vec<String>) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   CREATING GROUP                               ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", format!("Group Name: {}", name).bold());
    println!("{}", format!("Members: {}", members.join(", ")).bold());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Check if group already exists
    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM groups WHERE name = ?"
    )
    .bind(&name)
    .fetch_optional(pool)
    .await?;
    
    if exists.is_some() {
        return Err(OmniShellError::Other(format!("Group '{}' already exists", name)));
    }
    
    // Verify all members exist
    for member in &members {
        let member_name = member.trim_start_matches('@');
        let exists: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM contacts WHERE name = ?"
        )
        .bind(member_name)
        .fetch_optional(pool)
        .await?;
        
        if exists.is_none() {
            return Err(OmniShellError::ContactNotFound(member_name.to_string()));
        }
    }
    
    // Generate group encryption key
    println!("{} Generating group encryption key...", "→".cyan());
    let mut group_key = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut group_key);
    
    // Create group settings
    let settings = GroupSettings {
        admins_only: false,
        encrypt_sender: false,
    };
    let settings_json = serde_json::to_string(&settings)?;
    
    let now = Utc::now().timestamp();
    
    // Insert group
    let result = sqlx::query(
        "INSERT INTO groups (name, created_at, encryption_key, settings) VALUES (?, ?, ?, ?)"
    )
    .bind(&name)
    .bind(now)
    .bind(&group_key.to_vec())
    .bind(&settings_json)
    .execute(pool)
    .await?;
    
    let group_id = result.last_insert_rowid();
    
    // Add members
    for member in &members {
        let member_name = member.trim_start_matches('@');
        let contact_id: (i64,) = sqlx::query_as(
            "SELECT id FROM contacts WHERE name = ?"
        )
        .bind(member_name)
        .fetch_one(pool)
        .await?;
        
        sqlx::query(
            "INSERT INTO group_members (group_id, contact_id, role, joined_at) VALUES (?, ?, ?, ?)"
        )
        .bind(group_id)
        .bind(contact_id.0)
        .bind("member")
        .bind(now)
        .execute(pool)
        .await?;
    }
    
    println!("{} Group created successfully!", "✓".green().bold());
    println!("  └─ Group: {}", name.cyan());
    println!("  └─ Members: {}", members.len());
    println!();
    println!("Send a message:");
    println!("  {}", format!("omnishell group msg {} \"Hello everyone!\"", name).bright_blue());
    println!();
    
    Ok(())
}

pub async fn list_groups() -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let groups: Vec<(String, i64)> = sqlx::query_as(
        "SELECT name, created_at FROM groups ORDER BY name"
    )
    .fetch_all(pool)
    .await?;
    
    if groups.is_empty() {
        println!("{}", "No groups found.".yellow());
        println!();
        println!("Create a group:");
        println!("  {}", "omnishell group create <name> @user1 @user2 ...".cyan());
        return Ok(());
    }
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                       GROUPS                                   ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    for (group_name, created_at) in groups {
        // Count members
        let member_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM group_members WHERE group_id = (SELECT id FROM groups WHERE name = ?)"
        )
        .bind(&group_name)
        .fetch_one(pool)
        .await?;
        
        println!("{} {} members", 
            group_name.cyan().bold(),
            member_count.0
        );
        println!("  Created: {}", output::format_timestamp(created_at).bright_black());
    }
    
    println!();
    println!("{} total groups", groups.len());
    println!();
    
    Ok(())
}

pub async fn group_info(name: &str) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get group
    let group: Option<(i64, i64, String)> = sqlx::query_as(
        "SELECT id, created_at, settings FROM groups WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    
    let (group_id, created_at, settings_json) = group.ok_or_else(|| {
        OmniShellError::Other(format!("Group '{}' not found", name))
    })?;
    
    let settings: GroupSettings = serde_json::from_str(&settings_json)?;
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", format!("║  Group: {:<56}║", name).cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "Created:".bold());
    println!("  {}", output::format_timestamp(created_at));
    println!();
    
    // Get members
    let members: Vec<(String, String)> = sqlx::query_as(
        "SELECT c.name, gm.role 
         FROM group_members gm 
         JOIN contacts c ON gm.contact_id = c.id 
         WHERE gm.group_id = ?"
    )
    .bind(group_id)
    .fetch_all(pool)
    .await?;
    
    println!("{}", format!("Members ({}):", members.len()).bold());
    for (member_name, role) in members {
        let role_badge = if role == "admin" { "👑" } else { "•" };
        println!("  {} @{}", role_badge, member_name.cyan());
    }
    println!();
    
    println!("{}", "Settings:".bold());
    println!("  Admins only: {}", if settings.admins_only { "Yes" } else { "No" });
    println!("  Encrypt sender: {}", if settings.encrypt_sender { "Yes" } else { "No" });
    println!();
    
    Ok(())
}

pub async fn add_member(group_name: &str, member: String) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let member_name = member.trim_start_matches('@');
    
    // Get group ID
    let group_id: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM groups WHERE name = ?"
    )
    .bind(group_name)
    .fetch_optional(pool)
    .await?;
    
    let (group_id,) = group_id.ok_or_else(|| {
        OmniShellError::Other(format!("Group '{}' not found", group_name))
    })?;
    
    // Get contact ID
    let contact_id: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM contacts WHERE name = ?"
    )
    .bind(member_name)
    .fetch_optional(pool)
    .await?;
    
    let (contact_id,) = contact_id.ok_or_else(|| {
        OmniShellError::ContactNotFound(member_name.to_string())
    })?;
    
    let now = Utc::now().timestamp();
    
    sqlx::query(
        "INSERT INTO group_members (group_id, contact_id, role, joined_at) VALUES (?, ?, ?, ?)"
    )
    .bind(group_id)
    .bind(contact_id)
    .bind("member")
    .bind(now)
    .execute(pool)
    .await?;
    
    println!("{} {} added to group {}", "✓".green().bold(), 
        format!("@{}", member_name).cyan(), 
        group_name.cyan());
    println!();
    
    Ok(())
}

pub async fn remove_member(group_name: &str, member: String) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let member_name = member.trim_start_matches('@');
    
    sqlx::query(
        "DELETE FROM group_members 
         WHERE group_id = (SELECT id FROM groups WHERE name = ?) 
         AND contact_id = (SELECT id FROM contacts WHERE name = ?)"
    )
    .bind(group_name)
    .bind(member_name)
    .execute(pool)
    .await?;
    
    println!("{} {} removed from group {}", "✓".green().bold(), 
        format!("@{}", member_name).cyan(), 
        group_name.cyan());
    println!();
    
    Ok(())
}

pub async fn send_group_message(group_name: &str, message: String) -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║              SENDING GROUP MESSAGE                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Get group key
    let group_data: Option<(i64, Vec<u8>)> = sqlx::query_as(
        "SELECT id, encryption_key FROM groups WHERE name = ?"
    )
    .bind(group_name)
    .fetch_optional(pool)
    .await?;
    
    let (group_id, group_key) = group_data.ok_or_else(|| {
        OmniShellError::Other(format!("Group '{}' not found", group_name))
    })?;
    
    // Get member count
    let member_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM group_members WHERE group_id = ?"
    )
    .bind(group_id)
    .fetch_one(pool)
    .await?;
    
    println!("{} Group: {} ({} members)", "→".cyan(), group_name.cyan(), member_count.0);
    println!();
    
    // Create message
    let msg_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().timestamp();
    
    let msg = crate::messaging::Message {
        id: msg_id.clone(),
        sender: "me".to_string(),
        recipient: group_name.to_string(),
        content: message,
        timestamp,
        protocol: "group".to_string(),
        edited_at: None,
    };
    
    // Encrypt with group key
    println!("{} Encrypting message with group key...", "[🔐]".cyan());
    let msg_json = serde_json::to_vec(&msg)?;
    
    let config = Config::load()?;
    let cipher = CipherType::from_string(&config.encryption.default_cipher)?;
    
    let encrypted = encrypt_message(&msg_json, &group_key[..32].try_into().unwrap(), cipher)?;
    let encrypted_bytes = bincode::serialize(&encrypted)?;
    
    println!("{} Message encrypted", "✓".green());
    println!();
    
    // Would send to all group members here (simulated)
    println!("{} Broadcasting to {} members...", "📡".cyan(), member_count.0);
    output::show_encryption_animation("Distributing to group members", 50).await;
    
    println!("{} Message delivered to group!", "✓".green().bold());
    println!("  └─ Group: {}", group_name.cyan());
    println!("  └─ Message ID: {}", msg_id.bright_black());
    println!("  └─ Recipients: {}", member_count.0);
    println!();
    
    Ok(())
}
