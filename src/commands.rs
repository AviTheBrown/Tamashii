use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
pub enum Commands {
    Init,
    Add {
        path: PathBuf,
    },
    Verify {
        path: Option<PathBuf>,
        #[arg(long, short)]
        all: bool,
    },
    Status,
}
