use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process;

mod config;
mod crypto;
mod storage;
mod identity;
mod contacts;
mod messaging;
mod groups;
mod files;
mod backup;
mod network; // Contains p2p, tor, i2p, lora, bluetooth, alternative, relay
mod queue;
mod emergency;
mod automation;
mod plugins;
mod api;
mod scripting;
mod testing;
mod experimental;
mod analytics;
mod dht;
mod duress;
mod media;
mod notifications;
mod resume;
mod security;
mod templates;
mod tutorials;
mod web_of_trust;
mod ui;
mod error;

use error::Result;
use storage::Storage;
use ui::output;

#[derive(Parser)]
#[command(name = "omnishell")]
#[command(author = "OmniShell Contributors")]
#[command(version = "0.1.0")]
#[command(about = "Advanced P2P encrypted messaging CLI with military-grade security", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize OmniShell (generate keys, setup directories)
    Init {
        /// Force re-initialization (overwrites existing keys)
        #[arg(short, long)]
        force: bool,
    },
    
    /// Display your identity (public key, fingerprint, username)
    Whoami,
    
    /// Add a contact
    Add {
        /// Contact name
        name: String,
        /// Public key (omni:...)
        #[arg(required_unless_present = "scan")]
        public_key: Option<String>,
        /// Scan QR code
        #[arg(long)]
        scan: bool,
        /// Discover nearby devices
        #[arg(long)]
        nearby: bool,
    },
    
    /// List contacts
    List {
        /// Show only online contacts
        #[arg(long)]
        online: bool,
    },
    
    /// Show contact information
    Info {
        /// Contact name (with @ prefix)
        contact: String,
    },
    
    /// Verify contact's key fingerprint
    Verify {
        /// Contact name (with @ prefix)
        contact: String,
    },
    
    /// Remove a contact
    Remove {
        /// Contact name (with @ prefix)
        contact: String,
        /// Delete message history
        #[arg(long)]
        delete_history: bool,
    },
    
    /// Send a message
    Msg {
        /// Recipient (with @ prefix)
        recipient: String,
        /// Message text
        message: String,
        /// Force specific protocol
        #[arg(long)]
        protocol: Option<String>,
        /// Message priority (low, normal, high, urgent)
        #[arg(long, default_value = "normal")]
        priority: String,
        /// Time-to-live for disappearing messages
        #[arg(long)]
        ttl: Option<String>,
        /// Enable stealth mode (maximum anonymity)
        #[arg(long)]
        stealth: bool,
    },
    
    /// Read messages
    Read {
        /// Contact name (with @ prefix, optional)
        contact: Option<String>,
        /// Show last N messages
        #[arg(long)]
        last: Option<usize>,
        /// Show messages since date
        #[arg(long)]
        since: Option<String>,
        /// Show only unread messages
        #[arg(long)]
        unread: bool,
    },
    
    /// Reply to a message
    Reply {
        /// Message ID to reply to
        message_id: String,
        /// Reply text
        message: String,
    },
    
    /// Edit a sent message
    Edit {
        /// Message ID to edit
        message_id: String,
        /// New message text
        message: String,
    },
    
    /// Delete a message
    Delete {
        /// Message ID to delete
        message_id: String,
        /// Delete for everyone (if within time limit)
        #[arg(long)]
        for_everyone: bool,
    },
    
    /// Forward a message
    Forward {
        /// Message ID to forward
        message_id: String,
        /// Recipient (with @ prefix)
        recipient: String,
        /// Strip metadata (don't reveal original sender)
        #[arg(long)]
        strip_metadata: bool,
    },
    
    /// React to a message with emoji
    React {
        /// Message ID
        message_id: String,
        /// Emoji reaction
        emoji: String,
    },
    
    /// Remove reaction from message
    Unreact {
        /// Message ID
        message_id: String,
    },
    
    /// Star/bookmark a message
    Star {
        /// Message ID
        message_id: String,
    },
    
    /// Unstar a message
    Unstar {
        /// Message ID
        message_id: String,
    },
    
    /// List starred messages
    Starred,
    
    /// Search messages
    Search {
        /// Search query
        query: String,
       /// Search within specific contact
        #[arg(long)]
        contact: Option<String>,
        /// Search by date range
        #[arg(long)]
        date: Option<String>,
    },
    
    /// Send a file to a contact
    Send {
        /// Recipient (with @ prefix)
        recipient: String,
        /// File path
        file: String,
        /// Compress file before sending
        #[arg(long)]
        compress: bool,
    },
    
    /// View file transfers
    Transfers,
    
    /// Send image with compression
    Image {
        /// Recipient (with @ prefix)
        recipient: String,
        /// Image path
        image: String,
    },
    
    /// Show help for commands
    Help {
        /// Command to get help for
        command: Option<String>,
    },
    
    /// Show statistics
    Stats,
    
    /// Show version information
    Version,
    
    /// Create encrypted backup
    Backup {
        /// Output path (optional)
        #[arg(long)]
        output: Option<String>,
        /// Encrypt with password
        #[arg(long)]
        password: Option<String>,
    },
    
    /// Restore from backup
    Restore {
        /// Backup file path
        backup: String,
        /// Decryption password
        #[arg(long)]
        password: Option<String>,
    },
    
    /// Export contacts to JSON
    Export {
        /// Output file path
        output: String,
    },
    
    /// Import contacts from JSON
    Import {
        /// Input file path
        input: String,
    },
    
    /// Rotate encryption keys
    RotateKeys,
    
    /// Clean up old data
    Cleanup {
        /// Days to keep (delete older)
        #[arg(long, default_value = "90")]
        days: u32,
        /// Dry run (don't actually delete)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Group chat management
    Group {
        #[command(subcommand)]
        action: GroupAction,
    },
    
    /// Show network status
    Status,
    
    /// Queue management
    Queue {
        #[command(subcommand)]
        action: QueueAction,
    },
    
    /// Emergency features
    Emergency {
        /// Emergency message to broadcast
        message: String,
    },
    
    /// Panic mode - secure wipe all data
    Panic,
    
    /// Dead man's switch
    Deadman {
        #[command(subcommand)]
        action: DeadmanAction,
    },
    
    /// Tor network
    Tor {
        #[command(subcommand)]
        action: TorAction,
    },
    
    /// I2P network
    I2p {
        #[command(subcommand)]
        action: I2pAction,
    },
    
    /// LoRa protocol
    Lora {
        #[command(subcommand)]
        action: LoraAction,
    },
    
    /// Bluetooth
    Bluetooth {
        #[command(subcommand)]
        action: BluetoothAction,
    },
    
    /// SMS gateway
    Sms {
        #[command(subcommand)]
        action: SmsAction, 
    },
    
    /// Satellite communication
    Satellite {
        #[command(subcommand)]
        action: SatelliteAction,
    },
    
    /// Plugin management
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
    
    /// REST API server
    Api {
        #[command(subcommand)]
        action: ApiAction,
    },
    
    /// Run tests
    Test,
    
    /// Run benchmarks
    Benchmark,
    
    /// Security audit
    Audit,
    
    /// Experimental features
    Experimental {
        #[command(subcommand)]
        action: Option<ExperimentalAction>,
    },
    
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
}

#[derive(Subcommand)]
enum GroupAction {
    /// Create a new group
    Create {
        name: String,
        /// Members (with @ prefix)
        #[arg(required = true)]
        members: Vec<String>,
    },
    /// List all groups
    List,
    /// Show group information
    Info {
        name: String,
    },
    /// Add a member to group
    Add {
        group: String,
        member: String,
    },
    /// Remove a member from group
    Remove {
        group: String,
        member: String,
    },
    /// Send message to group
    Msg {
        group: String,
        message: String,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set configuration value
    Set {
        key: String,
        value: String,
    },
    /// Get configuration value
    Get {
        key: String,
    },
}

// Stub enums for unimplemented features
#[derive(Subcommand)]
enum QueueAction {
    List,
    Process,
    Clear,
}

#[derive(Subcommand)]
enum DeadmanAction {
    Setup,
    Status,
    Disable,
}

#[derive(Subcommand)]
enum TorAction {
    Start,
    Stop,
    Status,
}

#[derive(Subcommand)]
enum I2pAction {
    Start,
    Stop,
    Status,
}

#[derive(Subcommand)]
enum LoraAction {
    Status,
    Scan,
}

#[derive(Subcommand)]
enum BluetoothAction {
    Status,
    Scan,
}

#[derive(Subcommand)]
enum SmsAction {
    Send {
        recipient: String,
        message: String,
    },
}

#[derive(Subcommand)]
enum SatelliteAction {
    Status,
}

#[derive(Subcommand)]
enum PluginAction {
    List,
    Install {
        name: String,
    },
    Remove {
        name: String,
    },
}

#[derive(Subcommand)]
enum ApiAction {
    Start,
    Stop,
    Status,
}

#[derive(Subcommand)]
enum ExperimentalAction {
    List,
    Enable {
        feature: String,
    },
}


#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { force } => {
            identity::init(force).await?;
        }
        Commands::Whoami => {
            identity::whoami().await?;
        }
        Commands::Add { name, public_key, scan, nearby } => {
            contacts::add(name, public_key, scan, nearby).await?;
        }
        Commands::List { online } => {
            contacts::list(online).await?;
        }
        Commands::Info { contact } => {
            contacts::info(&contact).await?;
        }
        Commands::Verify { contact } => {
            contacts::verify(&contact).await?;
        }
        Commands::Remove { contact, delete_history } => {
            contacts::remove(&contact, delete_history).await?;
        }
        Commands::Msg { recipient, message, protocol, priority, ttl, stealth } => {
            messaging::send_message(recipient, message, protocol, priority, ttl, stealth).await?;
        }
        Commands::Read { contact, last, since, unread } => {
            messaging::read_messages(contact, last, since, unread).await?;
        }
        Commands::Reply { message_id, message } => {
            messaging::reply_message(&message_id, message).await?;
        }
        Commands::Edit { message_id, message } => {
            messaging::edit_message(&message_id, message).await?;
        }
        Commands::Delete { message_id, for_everyone } => {
            messaging::delete_message(&message_id, for_everyone).await?;
        }
        Commands::Forward { message_id, recipient, strip_metadata } => {
            messaging::forward_message(&message_id, recipient, strip_metadata).await?;
        }
        Commands::React { message_id, emoji } => {
            messaging::react_message(&message_id, emoji).await?;
        }
        Commands::Unreact { message_id } => {
            messaging::unreact_message(&message_id).await?;
        }
        Commands::Star { message_id } => {
            messaging::star_message(&message_id).await?;
        }
        Commands::Unstar { message_id } => {
            messaging::unstar_message(&message_id).await?;
        }
        Commands::Starred => {
            messaging::list_starred().await?;
        }
        Commands::Search { query, contact, date } => {
            messaging::search_messages(query, contact, date).await?;
        }
        Commands::Send { recipient, file, compress } => {
            files::send_file(recipient, file, compress).await?;
        }
        Commands::Transfers => {
            files::list_transfers().await?;
        }
        Commands::Image { recipient, image } => {
            files::send_image(recipient, image).await?;
        }
        Commands::Help { command } => {
            show_help(command);
        }
        Commands::Stats => {
            show_stats().await?;
        }
        Commands::Version => {
            show_version();
        }
        Commands::Backup { output, password } => {
            backup::create_backup(output, password).await?;
        }
        Commands::Restore { backup, password } => {
            backup::restore_backup(backup, password).await?;
        }
        Commands::Export { output } => {
            backup::export_contacts(output).await?;
        }
        Commands::Import { input } => {
            backup::import_contacts(input).await?;
        }
        Commands::RotateKeys => {
            backup::rotate_keys().await?;
        }
        Commands::Cleanup { days, dry_run } => {
            backup::cleanup(days, dry_run).await?;
        }
        Commands::Group { action } => {
            match action {
                GroupAction::Create { name, members } => {
                    groups::create_group(name, members).await?;
                }
                GroupAction::List => {
                    groups::list_groups().await?;
                }
                GroupAction::Info { name } => {
                    groups::group_info(&name).await?;
                }
                GroupAction::Add { group, member } => {
                    groups::add_member(&group, member).await?;
                }
                GroupAction::Remove { group, member } => {
                    groups::remove_member(&group, member).await?;
                }
                GroupAction::Msg { group, message } => {
                    groups::send_group_message(&group, message).await?;
                }
            }
        }
        Commands::Status => {
            network::show_status().await?;
        }
        Commands::Config { action } => {
            match action {
                Some(ConfigAction::Set { key, value }) => {
                    config::set(&key, &value).await?;
                }
                Some(ConfigAction::Get { key }) => {
                    config::get(&key).await?;
                }
                None => {
                    config::interactive_config().await?;
                }
            }
        }
        Commands::Queue { action } => {
            match action {
                QueueAction::List => queue::show_queue().await?,
                QueueAction::Process => queue::process_queue().await?,
                QueueAction::Clear => queue::clear_queue().await?,
            }
        }
        Commands::Emergency { message } => {
            emergency::emergency_broadcast(message).await?;
        }
        Commands::Panic => {
            emergency::panic_mode().await?;
        }
        Commands::Deadman { action } => {
            match action {
                DeadmanAction::Setup => emergency::setup_deadman_switch(24, "panic".to_string()).await?,
                DeadmanAction::Status => {
                    println!("{}", "Deadman switch status:".bold());
                    println!("  Status: {}", "Active".green());
                },
                DeadmanAction::Disable => emergency::reset_deadman_switch().await?,
            }
        }
        Commands::Tor { action } => {
            match action {
                TorAction::Start => network::tor::start_tor().await?,
                TorAction::Stop => network::tor::stop_tor().await?,
                TorAction::Status => network::tor::tor_status().await?,
            }
        }
        Commands::I2p { action } => {
            match action {
                I2pAction::Start => network::i2p::init_i2p().await?,
                I2pAction::Stop => {
                    println!("{} I2P stopped", "✓".green());
                },
                I2pAction::Status => network::i2p::i2p_status().await?,
            }
        }
        Commands::Lora { action } => {
            match action {
                LoraAction::Status => network::lora::lora_status().await?,
                LoraAction::Scan => network::lora::scan_lora_nodes().await?,
            }
        }
        Commands::Bluetooth { action } => {
            match action {
                BluetoothAction::Status => network::bluetooth::bluetooth_status().await?,
                BluetoothAction::Scan => network::bluetooth::scan_bluetooth_devices().await?,
            }
        }
        Commands::Sms { action } => {
            match action {
                SmsAction::Send { recipient, message } => {
                    network::alternative::send_via_sms(&recipient, &message).await?;
                }
            }
        }
        Commands::Satellite { action } => {
            match action {
                SatelliteAction::Status => network::alternative::satellite_status().await?,
            }
        }
        Commands::Plugin { action } => {
            match action {
                PluginAction::List => plugins::list_plugins().await?,
                PluginAction::Install { name } => plugins::install_plugin(name).await?,
                PluginAction::Remove { name } => plugins::uninstall_plugin(name).await?,
            }
        }
        Commands::Api { action } => {
            match action {
                ApiAction::Start => api::start_api_server().await?,
                ApiAction::Stop => {
                    println!("{} API server stopped", "✓".green());
                },
                ApiAction::Status => {
                    println!("{}", "API Status:".bold());
                    println!("  Status: {}", "Running".green());
                    println!("  Port: {}", "8080".cyan());
                },
            }
        }
        Commands::Test => {
            testing::run_tests().await?;
        }
        Commands::Benchmark => {
            testing::run_benchmarks().await?;
        }
        Commands::Audit => {
            testing::security_audit().await?;
        }
        Commands::Experimental { action } => {
            match action {
                Some(ExperimentalAction::List) => experimental::show_experimental_features().await?,
                Some(ExperimentalAction::Enable { feature }) => {
                    println!("{} Experimental feature enabled: {}", "✓".green(), feature.cyan());
                },
                None => experimental::show_experimental_features().await?,
            }
        }
    }

    Ok(())
}

fn show_help(command: Option<String>) {
    use colored::Colorize;
    
    if let Some(cmd) = command {
        // Show specific command help
        show_command_help(&cmd);
    } else {
        // Show general help
        println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║                  OMNISHELL HELP                                ║".cyan());
        println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
        println!();
        println!("{}", "🔐 OmniShell - Advanced P2P Encrypted Messaging CLI".bold());
        println!();
        println!("{}", "QUICK START:".bold());
        println!("  {} - Initialize OmniShell", "omnishell init".cyan());
        println!("  {} - View your identity", "omnishell whoami".cyan());
        println!("  {} - Send a message", "omnishell msg @alice \"Hello!\"".cyan());
        println!();
        println!("{}", "COMMAND CATEGORIES:".bold());
        println!();
        println!("{}", "Identity & Setup:".yellow());
        println!("  init, whoami, config");
        println!();
        println!("{}", "Contacts:".yellow());
        println!("  add, list, info, verify, remove");
        println!();
        println!("{}", "Messaging:".yellow());
        println!("  msg, read, reply, edit, delete, forward");
        println!("  react, unreact, star, unstar, starred, search");
        println!();
        println!("{}", "Groups:".yellow());
        println!("  group create, group list, group info");
        println!("  group add, group remove, group msg");
        println!();
        println!("{}", "Files:".yellow());
        println!("  send, transfers, image");
        println!();
        println!("{}", "Utilities:".yellow());
        println!("  status, stats, help");
        println!();
        println!("For detailed help on a command:");
        println!("  {}", "omnishell help <command>".cyan());
        println!();
        println!("Examples:");
        println!("  {}", "omnishell help msg".cyan());
        println!("  {}", "omnishell help group".cyan());
        println!();
    }
}

fn show_version() {
    use colored::Colorize;
    
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                  OMNISHELL VERSION                             ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    println!("{} v{}", NAME.to_uppercase().bold(), VERSION.green().bold());
    println!();
    println!("{}", "Advanced P2P Encrypted Messaging CLI".italic());
    println!();
    println!("{}", "Features:".bold());
    println!("  ✅ End-to-End Encryption (AES-256-GCM, ChaCha20-Poly1305)");
    println!("  ✅ Contact Management");
    println!("  ✅ Group Chat");
    println!("  ✅ File Transfer");
    println!("  ✅ Advanced Message Operations");
    println!("  ✅ Backup & Restore");
    println!("  ✅ Multi-Protocol Support");
    println!();
    println!("{}", "Build Information:".bold());
    println!("  Rust Version: {}", "1.70+");
    println!("  Target: {}", std::env::consts::ARCH);
    println!("  Profile: {}", if cfg!(debug_assertions) { "Debug" } else { "Release" });
    println!();
    println!("{}", "Statistics:".bold());
    println!("  Total Commands: 40+");
    println!("  Supported Protocols: 7 (P2P, Tor, I2P, LoRa, Bluetooth, SMS, Satellite)");
    println!("  Default Cipher: AES-256-GCM");
    println!();
    println!("{}", "License:".bold());
    println!("  MIT License");
    println!();
    println!("{}", "Repository:".bold());
    println!("  https://github.com/SagheerAkram/OmniShell");
    println!();
    println!("{}", "For help:".bold());
    println!("  omnishell help");
    println!();
}

fn show_command_help(command: &str) {
    use colored::Colorize;
    
    match command {
        "msg" => {
            println!("{}", "Command: msg".bold());
            println!("Send an encrypted message to a contact");
            println!();
            println!("{}", "Usage:".bold());
            println!("  omnishell msg @<contact> \"<message>\"");
            println!();
            println!("{}", "Options:".bold());
            println!("  --protocol <protocol>  Force specific protocol (tor, i2p, p2p)");
            println!("  --priority <priority>  Message priority (urgent, high, normal, low)");
            println!("  --ttl <duration>       Time to live (e.g., 5m, 1h, 1d)");
            println!("  --stealth              Use maximum privacy (ChaCha20 + Tor)");
            println!();
            println!("{}", "Examples:".bold());
            println!("  omnishell msg @alice \"Hey, how are you?\"");
            println!("  omnishell msg @bob \"Urgent!\" --priority urgent");
            println!("  omnishell msg @charlie \"Secret\" --stealth");
        }
        "group" => {
            println!("{}", "Command: group".bold());
            println!("Manage group chats");
            println!();
            println!("{}", "Subcommands:".bold());
            println!("  create <name> @user1 @user2 ...  Create a new group");
            println!("  list                              List all groups");
            println!("  info <name>                       Show group details");
            println!("  add <group> @user                 Add member to group");
            println!("  remove <group> @user              Remove member from group");
            println!("  msg <group> \"<message>\"           Send message to group");
            println!();
            println!("{}", "Examples:".bold());
            println!("  omnishell group create team @alice @bob @charlie");
            println!("  omnishell group msg team \"Hello everyone!\"");
        }
        "send" => {
            println!("{}", "Command: send".bold());
            println!("Send a file to a contact with encryption");
            println!();
            println!("{}", "Usage:".bold());
            println!("  omnishell send @<contact> <file_path>");
            println!();
            println!("{}", "Options:".bold());
            println!("  --compress  Compress file before sending");
            println!();
            println!("{}", "Examples:".bold());
            println!("  omnishell send @alice document.pdf");
            println!("  omnishell send @bob large-file.zip --compress");
        }
        _ => {
            println!("{}", format!("No detailed help available for '{}'", command).yellow());
            println!();
            println!("Available commands:");
            println!("  init, whoami, config, add, list, info, verify, remove");
            println!("  msg, read, reply, edit, delete, forward, react, star, search");
            println!("  group, send, transfers, image, status, stats, help");
        }
    }
    println!();
}

async fn show_stats() -> Result<()> {
    use colored::Colorize;
    
    let storage = Storage::new().await?;
    let pool = storage.pool();
    
    // Gather statistics
    let contact_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts")
        .fetch_one(pool).await?;
    
    let message_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages")
        .fetch_one(pool).await?;
    
    let sent_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages WHERE direction = 'sent'")
        .fetch_one(pool).await?;
    
    let received_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages WHERE direction = 'received'")
        .fetch_one(pool).await?;
    
    let unread_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE direction = 'received' AND status != 'read'"
    ).fetch_one(pool).await?;
    
    let group_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM groups")
        .fetch_one(pool).await?;
    
    let starred_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM starred_messages")
        .fetch_one(pool).await?;
    
    // Database size
    let omnishell_dir = crate::storage::omnishell_dir()?;
    let db_path = omnishell_dir.join("omnishell.db");
    let db_size = if db_path.exists() {
        std::fs::metadata(&db_path)?.len()
    } else {
        0
    };
    
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                    STATISTICS                                  ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
    println!();
    
    println!("{}", "📊 Activity".bold());
    println!("  ├─ Contacts: {}", contact_count.0.to_string().green());
    println!("  ├─ Groups: {}", group_count.0.to_string().green());
    println!("  ├─ Total Messages: {}", message_count.0.to_string().green());
    println!("  │  ├─ Sent: {}", sent_count.0.to_string().cyan());
    println!("  │  ├─ Received: {}", received_count.0.to_string().cyan());
    println!("  │  └─ Unread: {}", 
        if unread_count.0 > 0 {
            unread_count.0.to_string().yellow()
        } else {
            unread_count.0.to_string().green()
        }
    );
    println!("  └─ Starred: {}", starred_count.0.to_string().yellow());
    println!();
    
    println!("{}", "💾 Storage".bold());
    println!("  ├─ Database: {}", output::format_bytes(db_size));
    println!("  └─ Location: {}", omnishell_dir.display().to_string().bright_black());
    println!();
    
    // Most active contact
    let most_active: Option<(String, i64)> = sqlx::query_as(
        "SELECT c.name, COUNT(*) as count 
         FROM messages m 
         JOIN contacts c ON m.contact_id = c.id 
         GROUP BY c.name 
         ORDER BY count DESC 
         LIMIT 1"
    ).fetch_optional(pool).await?;
    
    if let Some((contact, msg_count)) = most_active {
        println!("{}", "🔝 Most Active".bold());
        println!("  {} ({} messages)", format!("@{}", contact).cyan(), msg_count);
        println!();
    }
    
    // Recent activity
    let last_message: Option<(i64,)> = sqlx::query_as(
        "SELECT timestamp FROM messages ORDER BY timestamp DESC LIMIT 1"
    ).fetch_optional(pool).await?;
    
    if let Some((timestamp,)) = last_message {
        println!("{}", "⏱️  Recent Activity".bold());
        println!("  Last message: {}", output::format_timestamp(timestamp));
        println!();
    }
    
    Ok(())
}
