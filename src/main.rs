mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands, SpamCommands, QuoteCommands, CalcCommands};

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
        Some(Commands::Calc { command }) => {
            match command {
                CalcCommands::Add { a, b } => {
                    commands::calc::add(a, b);
                }
                CalcCommands::Subtract { a, b } => {
                    commands::calc::subtract(a, b);
                }
                CalcCommands::Multiply { a, b } => {
                    commands::calc::multiply(a, b);
                }
                CalcCommands::Divide { a, b } => {
                    commands::calc::divide(a, b);
                }
            }
        }
        Some(Commands::Quote { command }) => {
            match command {
                QuoteCommands::Random => {
                    commands::quote::handle_random_quote_command();
                }
                QuoteCommands::Create { quote, author } => {
                    commands::quote::handle_create_quote_command(&quote, &author);
                }
            }
        }
        Some(Commands::Spam { command }) => {
            match command {
                SpamCommands::Counter { content, counter } => {
                    commands::spam::handle_spam_counter_command(&content, counter);
                }
                SpamCommands::Duration { content, duration } => {
                    commands::spam::handle_spam_duration_command(&content, duration);
                }
            }
        }
        None => {
            println!("Welcome to lcli! Use --help for more information.");
        }
    }
}
