use std::env;
use std::process;

use text_scout::Config;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║   TextScout - Lightweight Rust File Search               ║");
    println!("║──────────────────────────────────────────────────────────║");
    println!("║   Usage   : text_scout <query> <file>                    ║");
    println!("║   Example : text_scout duct poem.txt                     ║");
    println!("║   Hint    : Set IGNORE_CASE=1 for case-insensitive mode  ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = text_scout::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}