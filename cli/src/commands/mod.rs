use clap::Subcommand;

pub mod ingest;

#[derive(Subcommand)]
pub enum Commands {
    Ingest(ingest::IngestArgs),
}
