use std::env;
use std::process;

use text_scout::Config;

fn main() {
    println!("╔═════════════════════════════════╗");
    println!("║       Welcome to TextScout!     ║");
    println!("║   Fast & minimal file search    ║");
    println!("╚═════════════════════════════════╝\n");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = text_scout::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
