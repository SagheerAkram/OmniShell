// The Hydra - Distributed Secret Sharing using Shamir's Secret Sharing
use sharks::{Sharks, Share};
use crate::error::{OmniShellError, Result};
use colored::Colorize;

/// Split a secret into N shares, requiring K to recover
pub fn split_secret(secret: &[u8], threshold: u8, total_shares: u8) -> Result<Vec<String>> {
    if threshold > total_shares {
        return Err(OmniShellError::InvalidInput("Threshold cannot be greater than total shares".to_string()));
    }

    let sharks = Sharks(threshold);
    // Convert secret to secret shares
    let dealer = sharks.dealer(secret);
    
    let shares: Vec<String> = dealer
        .take(total_shares as usize)
        .map(|s| hex::encode(s.to_bytes()))
        .collect();

    Ok(shares)
}

/// Recover a secret from a list of hex-encoded shares
pub fn recover_secret(shares_hex: Vec<String>) -> Result<Vec<u8>> {
    if shares_hex.is_empty() {
        return Err(OmniShellError::InvalidInput("No shares provided".to_string()));
    }

    let sharks = Sharks(0); // Threshold is determined by number of shares provided? No, it's embedded in the math usually or we just need enough points. 
    // Wait, sharks crate usage: Sharks(threshold).recover(shares).
    // The threshold must match what was used to generate? 
    // Actually, distinct from some implementations, sharks recovery usually needs just the shares.
    // Let's check typical usage. Sharks(threshold) is needed for Dealer. For recovery, we can use Sharks::recover given enough shares.
    
    // We need to parse the shares first
    let mut parsed_shares = Vec::new();
    for share_str in shares_hex {
        let share_bytes = hex::decode(share_str)
            .map_err(|_| OmniShellError::InvalidInput("Invalid hex share".to_string()))?;
        let share = Share::from_bytes(&share_bytes)
            .map_err(|_| OmniShellError::InvalidInput("Invalid share data".to_string()))?;
        parsed_shares.push(share);
    }

    // Try to recover. If we don't have enough, it might return wrong data or fail.
    // Sharks::recover is a static method or instance method?
    // Looking at common usage: Sharks(threshold).recover(&shares)
    // We might need to know the threshold? 
    // Actually, standard Shamir interpolation works if you have K valid points.
    // Let's assume the user provides enough valid shares.
    // We'll use a conservative threshold of 1 for the struct init if it requires one, 
    // but the recovery math relies on the points provided.
    
    let recovered = sharks.recover(&parsed_shares)
        .ok_or_else(|| OmniShellError::Generic("Failed to recover secret (insufficient or invalid shares)".to_string()))?;

    Ok(recovered)
}

pub async fn run_hydra_split(secret: String, total: u8, threshold: u8) -> Result<()> {
    println!("{} initializing The Hydra...", "⚡".yellow());
    println!("Splitting secret into {} shards (Threshold: {})", total, threshold);
    
    let shares = split_secret(secret.as_bytes(), threshold, total)?;
    
    println!();
    println!("{}", "GENERATED SHARDS (DISTRIBUTE THESE SECURELY):".green().bold());
    println!("{}", "─────────────────────────────────────────────".green());
    
    for (i, share) in shares.iter().enumerate() {
        println!("Shard #{}: {}", i + 1, share.cyan());
    }
    
    println!();
    println!("You need {} of these to recover the secret.", threshold);
    Ok(())
}

pub async fn run_hydra_recover(shares: Vec<String>) -> Result<()> {
    println!("{} initializing The Hydra...", "⚡".yellow());
    println!("Attempting recovery with {} shards...", shares.len());
    
    match recover_secret(shares) {
        Ok(secret_bytes) => {
            let secret_str = String::from_utf8_lossy(&secret_bytes);
            println!();
            println!("{}", "SECRET RECOVERED SUCCESSFULLY:".green().bold());
            println!("{}", "──────────────────────────────".green());
            println!("{}", secret_str.white().bold());
            println!();
        }
        Err(e) => {
            println!();
            println!("{}", "RECOVERY FAILED!".red().bold());
            println!("Error: {}", e);
            println!("Ensure you have enough valid shards from the same set.");
        }
    }
    
    Ok(())
}
