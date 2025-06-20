use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "A legendary command-line tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List files and directories
    List {
        /// The path to list files from. Defaults to the current directory.
        path: Option<PathBuf>,
    },
    /// Display the current local time
    Time,
    /// Displays information about the program and its author
    About,
    // We remove Install and Uninstall from the CLI tool itself.
}