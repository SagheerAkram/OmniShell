// Message Templates
use colored::Colorize;
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTemplate {
    pub name: String,
    pub content: String,
    pub variables: Vec<String>,
}

/// Create message template
pub async fn create_template(name: String, content: String) -> Result<()> {
    println!("{} Creating template '{}'...", "→".cyan(), name);
    
    // Extract variables (e.g., {{name}}, {{time}})
    let variables: Vec<String> = content
        .split("{{")
        .skip(1)
        .filter_map(|s| s.split("}}").next())
        .map(|s| s.to_string())
        .collect();
    
    let template = MessageTemplate {
        name: name.clone(),
        content,
        variables: variables.clone(),
    };
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let template_json = serde_json::to_string(&template)?;
    sqlx::query("INSERT INTO templates (name, content) VALUES (?, ?)")
        .bind(&name)
        .bind(&template_json)
        .execute(pool)
        .await?;
    
    println!("{} Template created", "✓".green().bold());
    if !variables.is_empty() {
        println!("  Variables: {}", variables.join(", "));
    }
    println!();
    
    Ok(())
}

/// List templates
pub async fn list_templates() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  MESSAGE TEMPLATES                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let templates: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, content FROM templates"
    )
    .fetch_all(pool)
    .await?;
    
    if templates.is_empty() {
        println!("{}", "No templates created".yellow());
        println!();
        println!("Create a template:");
        println!("  {}", "omnishell template create greeting \"Hello {{name}}!\"".cyan());
        println!();
        return Ok(());
    }
    
    for (name, content_json) in templates {
        let template: MessageTemplate = serde_json::from_str(&content_json)?;
        println!("{} {}", "●".green(), name.bold());
        println!("  {}", template.content.bright_black());
        if !template.variables.is_empty() {
            println!("  Variables: {}", template.variables.join(", ").yellow());
        }
        println!();
    }
    
    Ok(())
}

/// Use template
pub async fn use_template(name: String, vars: Vec<(String, String)>) -> Result<String> {
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    let template_data: Option<(String,)> = sqlx::query_as(
        "SELECT content FROM templates WHERE name = ?"
    )
    .bind(&name)
    .fetch_optional(pool)
    .await?;
    
    if let Some((content_json,)) = template_data {
        let template: MessageTemplate = serde_json::from_str(&content_json)?;
        let mut message = template.content.clone();
        
        for (var, value) in vars {
            message = message.replace(&format!("{{{{{}}}}}", var), &value);
        }
        
        Ok(message)
    } else {
        Err(crate::error::OmniShellError::InvalidInput(format!("Template '{}' not found", name)))
    }
}
