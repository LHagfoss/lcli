mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List { path }) => {
            let path = path.unwrap_or_default();
            commands::list::handle_list_command(&path, cli.json);
        }
        Some(Commands::Time) => {
            commands::time::handle_time_command(cli.json);
        }
        Some(Commands::About) => {
            commands::about::handle_about_command();
        }
        None => {
            println!("Please run 'lcli help' for more information.");
        }
    }
}