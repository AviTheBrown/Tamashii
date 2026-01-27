use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// The top-level command-line interface structure.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for the Tamashii CLI.
#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
pub enum Commands {
    /// Initialize a new Tamashii database in the current directory
    Init,
    /// Add a file to be tracked for integrity
    Add {
        /// Path to the file to track
        path: PathBuf,
    },
    /// Verify the integrity of tracked files
    Verify {
        /// Optional path to a specific file to verify
        path: Option<PathBuf>,
        /// Verify all tracked files
        #[arg(long, short)]
        all: bool,
    },
    /// View the status of the database and tracked files
    Status,
}
