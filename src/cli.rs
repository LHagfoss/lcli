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

    /// Spam logs using nested subcommands
    Spam {
        #[command(subcommand)]
        command: SpamCommands,
    },
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
