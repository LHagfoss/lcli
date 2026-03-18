use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "lcli",
    version,
    about = "A simple command-line tool by me lagos aka the name lcli = lucas command-lline interface",
    long_about = "A multi-purpose CLI built for speed and efficiency!... is what I wish I could say"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Output results in JSON format
    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List files and directories in a given path
    List {
        /// The path to list files from (defaults to current directory)
        path: Option<PathBuf>,
    },

    /// Display the current local time in a pretty format
    Time,

    /// Displays information about lcli and its creator
    About,
}
