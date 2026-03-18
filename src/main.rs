mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Ls { path }) => {
            let path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
            commands::ls::handle_ls_command(&path, cli.json);
        }
        Some(Commands::Time) => {
            commands::time::handle_time_command(cli.json);
        }
        Some(Commands::About) => {
            commands::about::handle_about_command();
        }
        None => {
            println!("Welcome to lcli! Use --help for more information.");
        }
    }
}
