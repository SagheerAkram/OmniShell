// Plugin System Foundation
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

use crate::error::{OmniShellError, Result};
use crate::storage::omnishell_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub entry_point: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin: Plugin,
    pub dependencies: Vec<String>,
    pub permissions: Vec<String>,
}

/// List installed plugins
pub async fn list_plugins() -> Result<()> {
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    INSTALLED PLUGINS                           ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    let plugins_dir = omnishell_dir()?.join("plugins");
    
    if !plugins_dir.exists() {
        fs::create_dir_all(&plugins_dir)?;
        println!("{}", "No plugins installed".yellow());
        println!();
        return Ok(());
    }
    
    let mut found_plugins = Vec::new();
    
    for entry in fs::read_dir(&plugins_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let manifest_path = path.join("plugin.toml");
            if manifest_path.exists() {
                let manifest_str = fs::read_to_string(&manifest_path)?;
                if let Ok(manifest) = toml::from_str::<PluginManifest>(&manifest_str) {
                    found_plugins.push(manifest);
                }
            }
        }
    }
    
    if found_plugins.is_empty() {
        println!("{}", "No plugins installed".yellow());
        println!();
        return Ok(());
    }
    
    for manifest in found_plugins {
        let status = if manifest.plugin.enabled { "ENABLED".green() } else { "DISABLED".red() };
        println!("{} v{} [{}]", 
            manifest.plugin.name.bold(), 
            manifest.plugin.version,
            status
        );
        println!("  Author: {}", manifest.plugin.author);
        println!("  {}", manifest.plugin.description);
        println!();
    }
    
    println!("{}", "Plugin directory:".bold());
    println!("  {}", plugins_dir.display());
    println!();
    
    Ok(())
}

/// Install a plugin
pub async fn install_plugin(plugin_path: String) -> Result<()> {
    println!("{} Installing plugin from {}...", "→".cyan(), plugin_path);
    
    let source = Path::new(&plugin_path);
    if !source.exists() {
        return Err(OmniShellError::InvalidInput(format!("Plugin not found: {}", plugin_path)));
    }
    
    // Read manifest
    let manifest_path = source.join("plugin.toml");
    if !manifest_path.exists() {
        return Err(OmniShellError::InvalidInput("plugin.toml not found".to_string()));
    }
    
    let manifest_str = fs::read_to_string(&manifest_path)?;
    let manifest: PluginManifest = toml::from_str(&manifest_str)?;
    
    println!("{} Plugin: {} v{}", "→".cyan(), manifest.plugin.name, manifest.plugin.version);
    println!("  Author: {}", manifest.plugin.author);
    println!("  Description: {}", manifest.plugin.description);
    println!();
    
    // Copy to plugins directory
    let plugins_dir = omnishell_dir()?.join("plugins");
    fs::create_dir_all(&plugins_dir)?;
    
    let target_dir = plugins_dir.join(&manifest.plugin.name);
    if target_dir.exists() {
        println!("{}", "⚠️  Plugin already installed, removing old version...".yellow());
        fs::remove_dir_all(&target_dir)?;
    }
    
    // Copy plugin files
    copy_dir_recursive(source, &target_dir)?;
    
    println!("{} Plugin installed successfully", "✓".green().bold());
    println!("  Location: {}", target_dir.display());
    println!();
    println!("Enable the plugin:");
    println!("  {}", format!("omnishell plugin enable {}", manifest.plugin.name).cyan());
    println!();
    
    Ok(())
}

/// Enable a plugin
pub async fn enable_plugin(name: String) -> Result<()> {
    println!("{} Enabling plugin '{}'...", "→".cyan(), name);
    
    let plugins_dir = omnishell_dir()?.join("plugins");
    let plugin_dir = plugins_dir.join(&name);
    
    if !plugin_dir.exists() {
        return Err(OmniShellError::InvalidInput(format!("Plugin '{}' not found", name)));
    }
    
    // Update manifest
    let manifest_path = plugin_dir.join("plugin.toml");
    let manifest_str = fs::read_to_string(&manifest_path)?;
    let mut manifest: PluginManifest = toml::from_str(&manifest_str)?;
    
    manifest.plugin.enabled = true;
    
    let updated_toml = toml::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, updated_toml)?;
    
    println!("{} Plugin '{}' enabled", "✓".green().bold(), name);
    println!();
    
    Ok(())
}

/// Disable a plugin
pub async fn disable_plugin(name: String) -> Result<()> {
    println!("{} Disabling plugin '{}'...", "→".cyan(), name);
    
    let plugins_dir = omnishell_dir()?.join("plugins");
    let plugin_dir = plugins_dir.join(&name);
    
    if !plugin_dir.exists() {
        return Err(OmniShellError::InvalidInput(format!("Plugin '{}' not found", name)));
    }
    
    // Update manifest
    let manifest_path = plugin_dir.join("plugin.toml");
    let manifest_str = fs::read_to_string(&manifest_path)?;
    let mut manifest: PluginManifest = toml::from_str(&manifest_str)?;
    
    manifest.plugin.enabled = false;
    
    let updated_toml = toml::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, updated_toml)?;
    
    println!("{} Plugin '{}' disabled", "✓".green(), name);
    println!();
    
    Ok(())
}

/// Uninstall a plugin
pub async fn uninstall_plugin(name: String) -> Result<()> {
    println!("{} Uninstalling plugin '{}'...", "→".cyan(), name);
    
    let plugins_dir = omnishell_dir()?.join("plugins");
    let plugin_dir = plugins_dir.join(&name);
    
    if !plugin_dir.exists() {
        return Err(OmniShellError::InvalidInput(format!("Plugin '{}' not found", name)));
    }
    
    fs::remove_dir_all(&plugin_dir)?;
    
    println!("{} Plugin '{}' uninstalled", "✓".green().bold(), name);
    println!();
    
    Ok(())
}

// Helper function to copy directory recursively
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let dest_path = dst.join(file_name);
        
        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    
    Ok(())
}
