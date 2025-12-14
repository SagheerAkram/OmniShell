use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;
use crate::error::{OmniShellError, Result};

pub mod schema;

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new() -> Result<Self> {
        let db_path = Self::database_path()?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite:{}", db_path.display()))
            .await?;

        // Run migrations
        sqlx::query(schema::CREATE_TABLES_SQL)
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    fn database_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| OmniShellError::Config("Could not find home directory".to_string()))?;
        Ok(home_dir.join(".omnishell").join("omnishell.db"))
    }
}

pub fn ensure_directories() -> Result<()> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| OmniShellError::Config("Could not find home directory".to_string()))?;
    
    let base = home_dir.join(".omnishell");
    
    let dirs = vec![
        base.join("keys"),
        base.join("contacts"),
        base.join("groups"),
        base.join("messages"),
        base.join("queue").join("outgoing"),
        base.join("queue").join("incoming"),
        base.join("queue").join("failed"),
        base.join("nodes"),
        base.join("cache"),
        base.join("backups"),
        base.join("plugins"),
        base.join("logs"),
    ];

    for dir in dirs {
        std::fs::create_dir_all(&dir)?;
    }

    Ok(())
}

pub fn omnishell_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| OmniShellError::Config("Could not find home directory".to_string()))?;
    Ok(home_dir.join(".omnishell"))
}
