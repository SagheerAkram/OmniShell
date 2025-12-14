use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub async fn show_encryption_animation(message: &str, duration_ms: u64) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    
    let steps = duration_ms / 10;
    for _ in 0..steps {
        pb.tick();
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    pb.finish_and_clear();
}

pub fn show_progress(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(message.to_string());
    pb
}

pub fn print_box(title: &str, content: &[String]) {
    let max_width = content.iter().map(|s| s.len()).max().unwrap_or(0).max(title.len());
    let width = max_width + 4;
    
    println!("{}", format!("╔{}╗", "═".repeat(width)).cyan());
    println!("{}", format!("║ {:^width$} ║", title, width = width - 2).cyan());
    println!("{}", format!("╠{}╣", "═".repeat(width)).cyan());
    
    for line in content {
        println!("{}", format!("║ {:<width$} ║", line, width = width - 2).cyan());
    }
    
    println!("{}", format!("╚{}╝", "═".repeat(width)).cyan());
}

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    use prettytable::{Table, Row, Cell, format};
    
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    
    // Add headers
    let header_cells: Vec<Cell> = headers.iter()
        .map(|h| Cell::new(h).style_spec("Fyb"))
        .collect();
    table.add_row(Row::new(header_cells));
    
    // Add rows
    for row_data in rows {
        let cells: Vec<Cell> = row_data.iter()
            .map(|d| Cell::new(d))
            .collect();
        table.add_row(Row::new(cells));
    }
    
    table.printstd();
}

pub fn format_timestamp(timestamp: i64) -> String {
    use chrono::{DateTime, Local, Utc};
    
    let dt = DateTime::<Utc>::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| Utc::now());
    let local: DateTime<Local> = dt.into();
    
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_idx])
}

pub fn print_encryption_details(cipher: &str, key_fingerprint: &str) {
    println!("{} Encrypting message...", "[🔐]".cyan());
    println!("  └─ Algorithm: {}", cipher.green());
    println!("  └─ Key fingerprint: {}", key_fingerprint.yellow());
}

pub fn print_protocol_selection(protocol: &str, reason: &str) {
    println!("{} Selecting protocol...", "[📡]".cyan());
    println!("  └─ Selected: {}", protocol.green());
    println!("  └─ Reason: {}", reason);
}

pub fn print_routing_path(path: &[String]) {
    println!("{} Routing message...", "[🚀]".cyan());
    print!("  └─ Path: You");
    for hop in path {
        print!(" → {}", hop);
    }
    println!();
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_error(message: &str) {
    println!("{} {}", "✗".red().bold(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}
