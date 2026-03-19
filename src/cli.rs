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
    /// List files and directories in a given path relative or absolute idk prob relative :pray:
    Ls {
        /// The path to list files from (defaults to current directory)
        path: Option<PathBuf>,
    },

    /// Display the current local time in a pretty format
    Time,

    /// Displays information about lcli and its creator
    About,

    /// Calc is slang for calculator :nerd:
    Calc {
        #[command(subcommand)]
        command: CalcCommands,
    },

    /// Fetch and print a random quote
    Quote {
        #[command(subcommand)]
        command: QuoteCommands,
    },

    /// Spam logs using nested subcommands
    Spam {
        #[command(subcommand)]
        command: SpamCommands,
    },

    File {
        #[command(subcommand)]
        command: FileCommands,
    }
}

#[derive(Debug, Subcommand)]
pub enum CalcCommands {
    Add { a: i32, b: i32 },
    Subtract { a: i32, b: i32 },
    Multiply { a: i32, b: i32 },
    Divide { a: i32, b: i32 },
    Sqrt { a: u64 }
}

#[derive(Debug, Subcommand)]
pub enum QuoteCommands {
    Random,
    Create { quote: String, author: String },
}

#[derive(Debug, Subcommand)]
pub enum SpamCommands {
    /// Spam print logs in terminal by a counter
    Counter {
        /// Content of what you want to spam print
        content: String,

        /// Counter for how many times you wanan print
        counter: i32,
    },

    /// Spam print logs in terminal by a duration (seconds)
    Duration {
        /// Content of what you want to spam print..
        content: String,

        /// SECONDS okay SECONDSSS
        duration: i32,
    },
}

#[derive(Debug, Subcommand)]
pub enum FileCommands {
    /// Create a new empty file
    New {
        /// Name of the new file
        name: String,
        /// Directory to create it in (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Delete a file
    Delete { path: PathBuf },
    /// Move a file to a new directory
    Move {
        old_path: PathBuf,
        new_path: PathBuf,
    },
    /// Rename a file
    Rename {
        old_path: PathBuf,
        new_name: PathBuf,
    },
}
