mod commands;

use anyhow::Result;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "ingest_cli")]
#[command(about="TODO: Placeholder", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ingest(args) => commands::ingest::handle(args)?,
    }

    Ok(())
}
