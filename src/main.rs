use clap::Parser;
use std::process;
use text_scout::{Config, run};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    query: String,
    file_path: String,
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.query, args.file_path, args.ignore_case);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
