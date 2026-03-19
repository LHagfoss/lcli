#[path = "../cli.rs"]
mod cli;

use clap::CommandFactory;
use clap_complete::{generate, shells::Zsh};
use std::io;

fn main() {
    let mut cmd = cli::Cli::command();
    generate(Zsh, &mut cmd, "lcli", &mut io::stdout());
}
