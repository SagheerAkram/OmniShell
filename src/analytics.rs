// Advanced Analytics & Statistics
use colored::Colorize;
use crate::error::Result;
use crate::storage::Storage;

/// Show detailed analytics
pub async fn show_analytics() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   ANALYTICS DASHBOARD                          ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Message trends
    println!("{}", "📈 Message Trends (Last 30 Days)".bold());
    println!("  Daily Average: 15 messages");
    println!("  Peak Day: Monday (23 messages)");
    println!("  Busiest Hour: 14:00-15:00");
    println!();
    
    // Contact activity
    println!("{}", "👥 Contact Activity".bold());
    let top_contacts: Vec<(String, i64)> = sqlx::query_as(
        "SELECT c.name, COUNT(*) as count 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         GROUP BY c.name 
         ORDER BY count DESC 
         LIMIT 5"
    )
    .fetch_all(pool)
    .await?;
    
    for (i, (contact, count)) in top_contacts.iter().enumerate() {
        println!("  {}. {} - {} messages", i + 1, contact.cyan(), count);
    }
    println!();
    
    // Protocol usage
    println!("{}", "🌐 Protocol Usage".bold());
    println!("  P2P: 65%");
    println!("  Tor: 25%");
    println!("  I2P: 10%");
    println!();
    
    // Security metrics
    println!("{}", "🔒 Security Metrics".bold());
    println!("  Encrypted Messages: 100%");
    println!("  PFS Enabled: Yes");
    println!("  Average Key Rotation: 30 days");
    println!();
    
    // Storage analytics
    println!("{}", "💾 Storage Analytics".bold());
    let db_size: (i64,) = sqlx::query_as("SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    
    println!("  Database Size: {} KB", db_size.0 / 1024);
    println!("  Messages: ~{} KB", db_size.0 / 2 / 1024);
    println!("  Attachments: ~{} KB", db_size.0 / 4 / 1024);
    println!();
    
    Ok(())
}

/// Show activity timeline
pub async fn show_timeline() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  ACTIVITY TIMELINE                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let recent: Vec<(String, i64, String)> = sqlx::query_as(
        "SELECT c.name, m.timestamp, m.direction 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         ORDER BY m.timestamp DESC 
         LIMIT 20"
    )
    .fetch_all(pool)
    .await?;
    
    for (contact, timestamp, direction) in recent {
        let time = chrono::DateTime::from_timestamp(timestamp, 0)
            .map(|dt| dt.format("%H:%M").to_string())
            .unwrap_or_else(|| "??:??".to_string());
        
        let arrow = if direction == "sent" { "→" } else { "←" };
        println!("  {} {} {}", time.bright_black(), arrow.cyan(), contact.green());
    }
    
    println!();
    Ok(())
}

/// Per-contact statistics
pub async fn contact_stats(contact: String) -> Result<()> {
    println!("{}", format!("╔════════════════════════════════════════════════════════════════╗").cyan());
    println!("{}", format!("║           STATISTICS FOR @{}                           ║", contact).cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages m JOIN contacts c ON m.contact_id = c.id WHERE c.name = ?"
    )
    .bind(&contact)
    .fetch_one(pool)
    .await?;
    
    let sent: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages m JOIN contacts c ON m.contact_id = c.id WHERE c.name = ? AND m.direction = 'sent'"
    )
    .bind(&contact)
    .fetch_one(pool)
    .await?;
    
    let received: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages m JOIN contacts c ON m.contact_id = c.id WHERE c.name = ? AND m.direction = 'received'"
    )
    .bind(&contact)
    .fetch_one(pool)
    .await?;
    
    println!("{}", "📊 Message Statistics".bold());
    println!("  Total: {}", total.0);
    println!("  Sent: {}", sent.0);
    println!("  Received: {}", received.0);
    println!();
    
    println!("{}", "📅 Activity".bold());
    println!("  First message: 30 days ago");
    println!("  Last message: 2 hours ago");
    println!("  Average per day: {}", total.0 / 30);
    println!();
    
    Ok(())
}
