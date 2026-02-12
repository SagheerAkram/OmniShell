// Mirage - Panic Camouflage UI
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use colored::Colorize;
use rand::Rng;
use crate::error::Result;

pub enum MirageMode {
    Update,
    Logs,
    Code,
}

pub fn start_mirage(mode: MirageMode) -> Result<()> {
    // Clear screen immediately
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;

    match mode {
        MirageMode::Update => run_fake_update(),
        MirageMode::Logs => run_fake_logs(),
        MirageMode::Code => run_fake_code(),
    }
}

fn run_fake_update() -> Result<()> {
    println!("\n\n\n\n");
    println!("    Working on updates");
    
    for i in 0..=100 {
        print!("\r    {}% complete", i);
        print!("\n    Don't turn off your computer");
        print!("\x1B[1A"); // Move up one line to overwrite "Don't turn off..." less frequently or just keep it simple
        
        io::stdout().flush()?;
        
        // Random usage simulation
        let sleep_ms = rand::thread_rng().gen_range(500..3000);
        thread::sleep(Duration::from_millis(sleep_ms));
        
        if i == 100 {
             thread::sleep(Duration::from_secs(5));
             println!("\n\n    Restarting...");
             break;
        }
    }
    Ok(())
}

fn run_fake_logs() -> Result<()> {
    let logs = vec![
        "kern.info: [1234.56] wlan0: authenticate with 00:11:22:33:44:55",
        "kern.info: [1234.58] wlan0: send auth to 00:11:22:33:44:55 (try 1/3)",
        "kern.info: [1234.60] wlan0: authenticated",
        "kern.info: [1234.65] wlan0: associate with 00:11:22:33:44:55 (try 1/3)",
        "kern.info: [1234.70] wlan0: RX AssocResp from 00:11:22:33:44:55 (capab=0x411 status=0 aid=2)",
        "kern.info: [1234.75] wlan0: associated",
        "daemon.info: systemd[1]: Started Network Manager Script Dispatcher Service.",
        "auth.notice: sudo: pam_unix(sudo:session): session opened for user root by (uid=0)",
        "cron.info: CRON[1234]: (root) CMD (cd / && run-parts --report /etc/cron.hourly)",
    ];

    loop {
        let log = logs[rand::thread_rng().gen_range(0..logs.len())];
        let timestamp = chrono::Local::now().format("%b %d %H:%M:%S");
        println!("{} localhost {}", timestamp, log);
        
        io::stdout().flush()?;
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100..800)));
    }
}

fn run_fake_code() -> Result<()> {
    let code_snippets = vec![
        "def process_data(data):\n    return [x * 2 for x in data]",
        "impl Display for Mirage {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {",
        "const MAX_RETRIES = 5;\nlet mut retries = 0;",
        "if (user.isAdmin()) {\n    grantAccess();\n}",
        "SELECT * FROM access_logs WHERE successful = false;",
        "// TODO: Refactor this mess before production",
    ];

    loop {
        let snippet = code_snippets[rand::thread_rng().gen_range(0..code_snippets.len())];
        for char in snippet.chars() {
            print!("{}", char.to_string().green());
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20..100)));
        }
        println!();
        thread::sleep(Duration::from_millis(500));
    }
}
