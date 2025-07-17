// src/main.rs
/*
 * Main executable for NftMarketplaceComposer
 */

use clap::Parser;
use nftmarketplacecomposer::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceComposer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
