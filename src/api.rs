// REST API Server Module
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::Result;
use crate::storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub auth_required: bool,
    pub api_keys: Vec<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "127.0.0.1".to_string(),
            port: 3000,
            auth_required: true,
            api_keys: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Initialize API server
pub async fn init_api() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                   REST API SERVER SETUP                        ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "🌐 REST API Server".bold());
    println!();
    println!("Enables external applications to interact with OmniShell");
    println!("Use cases: Web dashboard, mobile apps, integrations");
    println!();
    
    let config = ApiConfig::default();
    
    println!("{}", "Configuration:".bold());
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  Authentication: {}", if config.auth_required { "Required" } else { "Disabled" });
    println!();
    
    // Generate API key
    let api_key = generate_api_key();
    println!("{}", "Generated API Key:".bold());
    println!("  {}", api_key.green());
    println!();
    println!("{}", "⚠️  Save this key securely - it won't be shown again!".yellow());
    println!();
    
    // Save config
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let mut config_with_key = config;
    config_with_key.api_keys.push(api_key);
    
    let config_json = serde_json::to_string(&config_with_key)?;
    sqlx::query("INSERT OR REPLACE INTO config (key, value) VALUES ('api_config', ?)")
        .bind(&config_json)
        .execute(pool)
        .await?;
    
    println!("{}", "API Endpoints:".bold());
    println!("  POST   /api/v1/messages/send");
    println!("  GET    /api/v1/messages/list");
    println!("  GET    /api/v1/contacts/list");
    println!("  POST   /api/v1/contacts/add");
    println!("  GET    /api/v1/status");
    println!("  POST   /api/v1/webhooks/register");
    println!();
    
    println!("{} API server initialized", "✓".green().bold());
    println!();
    println!("Start server:");
    println!("  {}", "omnishell api start".cyan());
    println!();
    
    Ok(())
}

/// Start API server
pub async fn start_api_server() -> Result<()> {
    println!("{} Starting REST API server...", "→".cyan());
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let config_data: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM config WHERE key = 'api_config'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((config_json,)) = config_data {
        let config: ApiConfig = serde_json::from_str(&config_json)?;
        
        let addr = format!("{}:{}", config.host, config.port);
        
        println!("{} API server running on http://{}", "✓".green().bold(), addr);
        println!();
        println!("{}", "Available Endpoints:".bold());
        println!("  http://{}/api/v1/messages/send", addr);
        println!("  http://{}/api/v1/status", addr);
        println!();
        println!("{}", "Example Request:".bold());
        println!("  curl -H 'Authorization: Bearer YOUR_API_KEY' \\");
        println!("       -H 'Content-Type: application/json' \\");
        println!("       -d '{{\"recipient\": \"@alice\", \"message\": \"Hello\"}}' \\");
        println!("       http://{}/api/v1/messages/send", addr);
        println!();
        
        // In production, this would start actual HTTP server
        // For now, show simulation
        println!("{}", "Note: Running in simulation mode".yellow());
        println!("Press Ctrl+C to stop");
        println!();
        
        // Simulate server running
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    } else {
        println!("{} API not initialized", "✗".red());
        println!("Run: {}", "omnishell api init".cyan());
    }
    
    Ok(())
}

/// Register webhook
pub async fn register_webhook(url: String, events: Vec<String>) -> Result<()> {
    println!("{} Registering webhook...", "→".cyan());
    println!("  URL: {}", url);
    println!("  Events: {}", events.join(", "));
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let webhook_data = serde_json::json!({
        "url": url,
        "events": events,
        "created_at": chrono::Utc::now().timestamp()
    });
    
    sqlx::query(
        "INSERT INTO webhooks (url, events, active) VALUES (?, ?, 1)"
    )
    .bind(&url)
    .bind(serde_json::to_string(&events)?)
    .execute(pool)
    .await?;
    
    println!("{} Webhook registered", "✓".green().bold());
    println!();
    println!("Webhook will be called on these events:");
    for event in events {
        println!("  • {}", event);
    }
    println!();
    
    Ok(())
}

/// List registered webhooks
pub async fn list_webhooks() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  REGISTERED WEBHOOKS                           ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let webhooks: Vec<(i64, String, String, bool)> = sqlx::query_as(
        "SELECT id, url, events, active FROM webhooks"
    )
    .fetch_all(pool)
    .await?;
    
    if webhooks.is_empty() {
        println!("{}", "No webhooks registered".yellow());
        println!();
        return Ok(());
    }
    
    for (id, url, events_json, active) in webhooks {
        let events: Vec<String> = serde_json::from_str(&events_json).unwrap_or_default();
        let status = if active { "ACTIVE".green() } else { "INACTIVE".red() };
        
        println!("#{} [{}]", id, status);
        println!("  URL: {}", url.cyan());
        println!("  Events: {}", events.join(", "));
        println!();
    }
    
    Ok(())
}

/// Generate API key
fn generate_api_key() -> String {
    use rand::RngCore;
    let mut key = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut key);
    format!("omni_api_{}", hex::encode(&key))
}

/// Trigger webhook for event
pub async fn trigger_webhook(event: &str, payload: serde_json::Value) -> Result<()> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let webhooks: Vec<(String, String)> = sqlx::query_as(
        "SELECT url, events FROM webhooks WHERE active = 1"
    )
    .fetch_all(pool)
    .await?;
    
    for (url, events_json) in webhooks {
        let events: Vec<String> = serde_json::from_str(&events_json).unwrap_or_default();
        
        if events.contains(&event.to_string()) {
            // In production, make HTTP POST request
            println!("{} Triggering webhook: {}", "→".cyan(), url);
            println!("  Event: {}", event);
            println!("  Payload: {}", payload);
        }
    }
    
    Ok(())
}
