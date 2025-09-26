mod runtime;
mod utils;

use chrono::Local;
use colored::*;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use runtime::Runtime;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (watch_mode, filename) = parse_args(&args);

    if watch_mode {
        run_with_watch(filename);
    } else {
        run_once(filename);
    }
}

fn parse_args(args: &[String]) -> (bool, String) {
    if args.len() < 2 {
        print_error("Invalid arguments");
        println!(
            "Usage: {} {} <javascript_file>",
            "rode".bold(),
            "[--watch, -w]".dimmed()
        );
        println!("  {} Run script once", "rode script.js".cyan());
        println!(
            "  {} Run script and watch for changes",
            "rode --watch script.js".cyan()
        );
        process::exit(1);
    }

    if args.len() == 3 && args[1] == "--watch" || args[1] == "-w" {
        (true, args[2].clone())
    } else if args.len() == 2 {
        (false, args[1].clone())
    } else {
        print_error("Invalid arguments");
        println!(
            "Usage: {} {} <javascript_file>",
            "rode".bold(),
            "[--watch, -w]".dimmed()
        );
        process::exit(1);
    }
}

fn run_once(filename: String) {
    print_header();

    let code = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(err) => {
            print_error(&format!("Cannot read file '{}'", filename));
            println!("  {}", err.to_string().red());
            process::exit(1);
        }
    };

    let mut runtime = Runtime::new();

    match runtime.execute(&code) {
        Ok(_) => {}
        Err(err) => {
            println!();
            print_error("Runtime error");
            println!("  {}", err.red());
            process::exit(1);
        }
    }
}

fn run_with_watch(filename: String) {
    let path = Path::new(&filename);
    if !path.exists() {
        print_error(&format!("File '{}' does not exist", filename));
        process::exit(1);
    }

    clear_screen();
    print_header();
    print_watch_banner(&filename);

    // Initial run
    run_script(&filename);

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let _ = tx.send(());
                }
            }
        },
        notify::Config::default(),
    )
    .unwrap();

    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    // Watch for changes
    while rx.recv().is_ok() {
        // Small delay to avoid multiple rapid triggers
        std::thread::sleep(Duration::from_millis(100));

        // Drain any additional events
        while rx.try_recv().is_ok() {}

        clear_screen();
        print_header();
        print_restart_banner(&filename);
        run_script(&filename);
    }
}

fn run_script(filename: &str) {
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            print_error(&format!("Cannot read file '{}'", filename));
            println!("  {}", err.to_string().red());
            return;
        }
    };

    let mut runtime = Runtime::new();

    match runtime.execute(&code) {
        Ok(_) => {
            println!();
            print_separator();
        }
        Err(err) => {
            println!();
            print_error("Runtime error");
            println!("  {}", err.red());
            print_separator();
        }
    }
}

// UI Helper Functions
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn print_header() {
    let version = env!("CARGO_PKG_VERSION");
    println!();
    println!("{}  {}", "🦀".bright_yellow(), "RODE".bright_blue().bold());
    println!(
        "   {} {}",
        "JavaScript Runtime".dimmed(),
        format!("v{}", version).dimmed()
    );
    println!();
}

fn print_watch_banner(filename: &str) {
    // Clear console
    clear_screen();
    let now = Local::now().format("%H:%M:%S");
    println!(
        "{} {} {} {}",
        "👀".bright_cyan(),
        "Watching".bright_cyan().bold(),
        filename.cyan().bold(),
        format!("({})", now).dimmed()
    );
    println!("   {} {}", "Press".dimmed(), "Ctrl+C".yellow().bold());
    println!();
    print_separator();
}

fn print_restart_banner(filename: &str) {
    clear_screen();
    let now = Local::now().format("%H:%M:%S");
    println!(
        "{} {} {}",
        "File changed, restarting".bright_yellow().bold(),
        filename.cyan().bold(),
        format!("({})", now).dimmed()
    );
    println!();
    print_separator();
}

fn print_error(message: &str) {
    println!("{} {}", "ERROR".red().bold(), message.red().bold());
}

fn print_separator() {
    println!("{}", "─".repeat(60).dimmed());
    println!();
}
