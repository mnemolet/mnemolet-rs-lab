use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct IngestArgs {
    /// Recreate Qdrant collection and reindex all files.
    #[arg(long)]
    pub force: bool,

    /// Number of chunks per batch.
    #[arg(long, default_value_t = 100)]
    pub batch_size: u32,
}

pub fn handle(args: &IngestArgs) -> Result<()> {
    println!("Force recreate: {}", args.force);
    println!("Batch size:     {}", args.batch_size);

    Ok(())
}
