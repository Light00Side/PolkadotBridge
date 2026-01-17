// src/main.rs
/*
 * PolkadotBridge - High-Performance Rust+ Implementation
 * Enhanced with async/await and parallel processing
 */

use clap::Parser;
use polkadotbridge::{Result, App, Config};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(version, about = "PolkadotBridge - Advanced Rust+ Implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Number of parallel workers
    #[arg(short, long, default_value = "4")]
    workers: usize,
    
    /// Input file or directory
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
    
    /// Enable parallel processing
    #[arg(long)]
    parallel: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    
    // Setup tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(if args.verbose { Level::DEBUG } else { Level::INFO })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    info!("Starting PolkadotBridge with {} workers", args.workers);
    
    let config = Config::builder()
        .workers(args.workers)
        .parallel(args.parallel)
        .build();
    
    let app = App::new(config);
    app.run(args.input, args.output).await
}
