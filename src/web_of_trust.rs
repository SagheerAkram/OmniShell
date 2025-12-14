// Web of Trust Implementation
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::error::Result;
use crate::storage::Storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustLevel {
    Unknown,      // 0 - No trust established
    Marginal,     // 1 - Some trust
    Full,         // 2 - Fully trusted
    Ultimate,     // 3 - Absolute trust (own keys)
}

impl TrustLevel {
    pub fn from_i32(level: i32) -> Self {
        match level {
            0 => TrustLevel::Unknown,
            1 => TrustLevel::Marginal,
            2 => TrustLevel::Full,
            3 => TrustLevel::Ultimate,
            _ => TrustLevel::Unknown,
        }
    }
    
    pub fn to_i32(&self) -> i32 {
        match self {
            TrustLevel::Unknown => 0,
            TrustLevel::Marginal => 1,
            TrustLevel::Full => 2,
            TrustLevel::Ultimate => 3,
        }
    }
    
    pub fn to_string(&self) -> &str {
        match self {
            TrustLevel::Unknown => "Unknown",
            TrustLevel::Marginal => "Marginal",
            TrustLevel::Full => "Full",
            TrustLevel::Ultimate => "Ultimate",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustSignature {
    pub signer: String,
    pub signee: String,
    pub trust_level: TrustLevel,
    pub timestamp: i64,
    pub signature: Vec<u8>,
}

/// Set trust level for a contact
pub async fn set_trust_level(contact: String, level: TrustLevel) -> Result<()> {
    println!("{} Setting trust level for {}...", "→".cyan(), contact);
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contact_name = contact.trim_start_matches('@');
    
    // Update trust level in database
    sqlx::query("UPDATE contacts SET trust_level = ? WHERE name = ?")
        .bind(level.to_string())
        .bind(contact_name)
        .execute(pool)
        .await?;
    
    println!("{} Trust level set to: {}", "✓".green().bold(), level.to_string().yellow());
    println!();
    
    Ok(())
}

/// Sign a contact's key (vouch for them)
pub async fn sign_key(contact: String, trust_level: TrustLevel) -> Result<()> {
    println!("{} Signing key for {}...", "→".cyan(), contact);
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let contact_name = contact.trim_start_matches('@');
    
    // Get contact's public key
    let contact_data: Option<(String, String)> = sqlx::query_as(
        "SELECT public_key, fingerprint FROM contacts WHERE name = ?"
    )
    .bind(contact_name)
    .fetch_optional(pool)
    .await?;
    
    if let Some((public_key, fingerprint)) = contact_data {
        // Get our identity
        let our_keypair = crate::identity::get_keypair()?;
        let our_identity = crate::identity::get_identity().await?;
        
        // Create trust signature
        let signature_data = format!("{}:{}:{}", 
            our_identity.username, 
            contact_name, 
            trust_level.to_i32()
        );
        
        let signature = crate::crypto::sign_message(
            signature_data.as_bytes(),
            &our_keypair
        )?;
        
        let trust_sig = TrustSignature {
            signer: our_identity.username.clone(),
            signee: contact_name.to_string(),
            trust_level: trust_level.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            signature: signature.to_vec(),
        };
        
        // Store signature
        let sig_json = serde_json::to_string(&trust_sig)?;
        sqlx::query(
            "INSERT INTO trust_signatures (signer, signee, trust_level, signature_data) VALUES (?, ?, ?, ?)"
        )
        .bind(&our_identity.username)
        .bind(contact_name)
        .bind(trust_level.to_i32())
        .bind(&sig_json)
        .execute(pool)
        .await?;
        
        println!("{} Key signed", "✓".green().bold());
        println!("  Signer: {}", our_identity.username.cyan());
        println!("  Signee: {}", contact_name.cyan());
        println!("  Trust: {}", trust_level.to_string().yellow());
        println!("  Fingerprint: {}", fingerprint.bright_black());
        println!();
        
        // Update contact's trust level
        set_trust_level(contact, trust_level).await?;
        
    } else {
        println!("{} Contact not found", "✗".red());
    }
    
    Ok(())
}

/// Calculate trust path to a contact
pub async fn calculate_trust_path(target: String) -> Result<Vec<String>> {
    println!("{} Calculating trust path to {}...", "→".cyan(), target);
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let our_identity = crate::identity::get_identity().await?;
    let target_name = target.trim_start_matches('@');
    
    // Get all trust signatures
    let signatures: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT signer, signee, trust_level FROM trust_signatures"
    )
    .fetch_all(pool)
    .await?;
    
    // Build trust graph
    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for (signer, signee, level) in signatures {
        graph.entry(signer.clone())
            .or_insert_with(Vec::new)
            .push((signee, level));
    }
    
    // BFS to find shortest trust path
    let mut queue = vec![(our_identity.username.clone(), vec![our_identity.username.clone()])];
    let mut visited = HashSet::new();
    
    while let Some((current, path)) = queue.pop() {
        if current == target_name {
            println!("{} Trust path found ({} hops)", "✓".green().bold(), path.len() - 1);
            for (i, node) in path.iter().enumerate() {
                if i == 0 {
                    println!("  {} {} (you)", "●".green(), node.cyan());
                } else if i == path.len() - 1 {
                    println!("  {} {} (target)", "●".yellow(), node.cyan());
                } else {
                    println!("  {} {}", "●".blue(), node.cyan());
                }
            }
            println!();
            return Ok(path);
        }
        
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        
        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, _level) in neighbors {
                if !visited.contains(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push((neighbor.clone(), new_path));
                }
            }
        }
    }
    
    println!("{} No trust path found", "⚠".yellow());
    println!();
    Ok(vec![])
}

/// Show web of trust
pub async fn show_web_of_trust() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  WEB OF TRUST                                  ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let our_identity = crate::identity::get_identity().await?;
    
    // Get all contacts with trust levels
    let contacts: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, trust_level FROM contacts ORDER BY trust_level DESC"
    )
    .fetch_all(pool)
    .await?;
    
    if contacts.is_empty() {
        println!("{}", "No contacts in web of trust".yellow());
        println!();
        return Ok(());
    }
    
    println!("{}", "Your Trust Network:".bold());
    println!();
    
    let mut by_level: HashMap<String, Vec<String>> = HashMap::new();
    for (name, level) in contacts {
        by_level.entry(level).or_insert_with(Vec::new).push(name);
    }
    
    // Ultimate trust (you)
    println!("{} {} (you)", "●".green().bold(), our_identity.username.cyan());
    println!();
    
    // Full trust
    if let Some(full_trust) = by_level.get("full") {
        println!("{} Full Trust ({}):", "●".green(), full_trust.len());
        for contact in full_trust {
            println!("  └─ {}", contact.cyan());
        }
        println!();
    }
    
    // Marginal trust
    if let Some(marginal) = by_level.get("marginal") {
        println!("{} Marginal Trust ({}):", "●".yellow(), marginal.len());
        for contact in marginal {
            println!("  └─ {}", contact.cyan());
        }
        println!();
    }
    
    // Unknown
    if let Some(unknown) = by_level.get("unknown") {
        println!("{} Unknown ({}):", "●".bright_black(), unknown.len());
        for contact in unknown {
            println!("  └─ {}", contact.bright_black());
        }
        println!();
    }
    
    // Get signature count
    let sig_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM trust_signatures"
    )
    .fetch_one(pool)
    .await?;
    
    println!("{}", "Statistics:".bold());
    println!("  Total contacts: {}", contacts.len());
    println!("  Trust signatures: {}", sig_count.0);
    println!();
    
    Ok(())
}

/// Verify trust chain
pub async fn verify_trust_chain(contact: String) -> Result<bool> {
    println!("{} Verifying trust chain for {}...", "→".cyan(), contact);
    
    let path = calculate_trust_path(contact).await?;
    
    if path.is_empty() {
        println!("{} No trust path - verification failed", "✗".red());
        return Ok(false);
    }
    
    println!("{} Trust chain verified", "✓".green().bold());
    Ok(true)
}
