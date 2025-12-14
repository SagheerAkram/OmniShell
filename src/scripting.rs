// Scripting & Pipe Support Module
use colored::Colorize;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use serde_json::Value;

use crate::error::Result;

/// Handle piped input from stdin
pub fn read_from_pipe() -> Result<Option<String>> {
    // Check if stdin is a pipe
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(Some(buffer))
    } else {
        Ok(None)
    }
}

/// Output to stdout for piping
pub fn write_to_pipe(data: &str) -> Result<()> {
    io::stdout().write_all(data.as_bytes())?;
    io::stdout().flush()?;
    Ok(())
}

/// Execute external command with piping
pub async fn execute_with_pipe(command: String, input_data: Option<String>) -> Result<String> {
    println!("{} Executing command: {}", "→".cyan(), command);
    
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(crate::error::OmniShellError::InvalidInput("Empty command".to_string()));
    }
    
    let mut cmd = Command::new(parts[0]);
    
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }
    
    if input_data.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    
    let mut child = cmd.spawn()?;
    
    // Write input if provided
    if let Some(data) = input_data {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(data.as_bytes())?;
        }
    }
    
    let output = child.wait_with_output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        Err(crate::error::OmniShellError::Other(error))
    }
}

/// Convert messages to JSON for piping
pub async fn messages_to_json(contact: &str) -> Result<String> {
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let messages: Vec<(String, i64, String)> = sqlx::query_as(
        "SELECT content, timestamp, direction FROM messages WHERE contact_id = (SELECT id FROM contacts WHERE name = ?) ORDER BY timestamp DESC LIMIT 50"
    )
    .bind(contact)
    .fetch_all(pool)
    .await?;
    
    let json_data: Vec<Value> = messages.into_iter().map(|(content, timestamp, direction)| {
        serde_json::json!({
            "content": content,
            "timestamp": timestamp,
            "direction": direction
        })
    }).collect();
    
    Ok(serde_json::to_string_pretty(&json_data)?)
}

/// Export contacts to JSON
pub async fn contacts_to_json() -> Result<String> {
    let storage = crate::storage::Storage::new().await?;
    let pool = storage.pool();
    
    let contacts: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT name, fingerprint, trust_level FROM contacts"
    )
    .fetch_all(pool)
    .await?;
    
    let json_data: Vec<Value> = contacts.into_iter().map(|(name, fingerprint, trust)| {
        serde_json::json!({
            "name": name,
            "fingerprint": fingerprint,
            "trust_level": trust
        })
    }).collect();
    
    Ok(serde_json::to_string_pretty(&json_data)?)
}

/// Example automation script template
pub fn generate_script_template(script_type: &str) -> String {
    match script_type {
        "auto-reply" => {
r#"#!/bin/bash
# OmniShell Auto-Reply Script

# Monitor messages and auto-reply
omnishell read @all --json | jq -r '.[] | select(.unread) | .sender' | while read sender; do
    omnishell msg "@$sender" "Auto-reply: I'm currently away"
done
"#.to_string()
        }
        "backup" => {
r#"#!/bin/bash
# OmniShell Automated Backup Script

# Create daily backup
DATE=$(date +%Y%m%d)
omnishell backup --output "./backups/omnishell_$DATE.tar.gz.enc" --password "$BACKUP_PASSWORD"

# Keep only last 7 days
find ./backups -name "omnishell_*.tar.gz.enc" -mtime +7 -delete
"#.to_string()
        }
        "webhook" => {
r#"#!/bin/bash
# OmniShell Webhook Handler

# Forward new messages to webhook
omnishell read @all --json | jq -r '.[] | select(.unread)' | while read msg; do
    curl -X POST https://your-webhook.com/messages \
         -H "Content-Type: application/json" \
         -d "$msg"
done
"#.to_string()
        }
        _ => {
r#"#!/bin/bash
# OmniShell Custom Script

# Your automation here
omnishell status --json
"#.to_string()
        }
    }
}

/// Show scripting examples
pub fn show_scripting_examples() {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                 SCRIPTING EXAMPLES                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "📝 Pipe Support".bold());
    println!();
    println!("Export messages to JSON:");
    println!("  {}", "omnishell read @alice --json".cyan());
    println!();
    println!("Filter with jq:");
    println!("  {}", "omnishell read @all --json | jq '.[] | select(.unread)'".cyan());
    println!();
    println!("Send piped message:");
    println!("  {}", "echo \"Hello\" | omnishell msg @alice".cyan());
    println!();
    
    println!("{}", "🔄 Automation Examples".bold());
    println!();
    println!("Auto-backup every day:");
    println!("  {}", "0 0 * * * omnishell backup --output /backups/daily.tar.gz.enc".bright_black());
    println!();
    println!("Monitor and forward:");
    println!("  {}", "watch -n 60 'omnishell read --unread | omnishell webhook send'".bright_black());
    println!();
    println!("Bulk send:");
    println!("  {}", "cat contacts.txt | xargs -I {{}} omnishell msg @{{}} \"Broadcast message\"".bright_black());
    println!();
    
    println!("{}", "🎯 Integration Examples".bold());
    println!();
    println!("With cron:");
    println!("  {}", "0 * * * * omnishell queue process".bright_black());
    println!();
    println!("With systemd:");
    println!("  {}", "[Unit]".bright_black());
    println!("  {}", "Description=OmniShell Message Processor".bright_black());
    println!("  {}", "[Service]".bright_black());
    println!("  {}", "ExecStart=/usr/bin/omnishell queue process".bright_black());
    println!();
}
