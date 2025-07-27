// src/main.rs
/*
 * Main executable for SuperEvent
 */

use clap::Parser;
use superevent::{Result, run};

#[derive(Parser)]
#[command(version, about = "SuperEvent - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
