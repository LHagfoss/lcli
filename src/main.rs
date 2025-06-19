use chrono::{DateTime, Local, Utc};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    settings::{
        object::{Columns, Rows},
        Color, Style,
    },
    Table, Tabled,
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Type B")]
    len_bytes: u64,
    #[tabled(rename = "Modified")]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best Ls command ever")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, global = true)]
    json: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List files and directories
    List {
        /// The path to list files from. Defaults to the current directory.
        path: Option<PathBuf>,
    },
    /// Display the current local time
    Time,
    /// "Installs" the program by adding it to the user's PATH (Windows only)
    Install,
    /// Displays information about the program and its auther
    About,
}

#[derive(Serialize)]
struct TimeOutput {
    local_time: String,
    local_timezone: String,
}

fn main() {
    if let Ok(should_install) = is_running_in_temp_location() {
        if should_install {
            if let Err(e) = installation_flow() {
                eprintln!("{} {}", "Installation failed:".red(), e);
                pause_for_user();
            }
            return;
        }
    }

    run_cli_app();
}

fn run_cli_app() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List { path }) => {
            let path = path.unwrap_or(PathBuf::from("."));
            handle_list_command(&path, cli.json);
        }
        Some(Commands::Time) => {
            handle_time_command(cli.json);
        }
        Some(Commands::Install) => {
            if let Err(e) = handle_install_command() {
                eprintln!("{} {}", "Installation failed:".red(), e);
            } else {
                println!("{}", "Installation command ran successfully!".green());
                println!("Note: This command should only be needed for troubleshooting.");
            }
        }
        None => {
            let path = PathBuf::from(".");
            handle_list_command(&path, cli.json);
        }
    }
}

fn get_install_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|path| path.join("Programs").join("lcli"))
}

fn is_running_in_temp_location() -> Result<bool, io::Error> {
    let current_exe = env::current_exe()?;
    if let Some(install_dir) = get_install_dir() {
        let is_installed = current_exe.starts_with(&install_dir);
        Ok(!is_installed)
    } else {
        Ok(false)
    }
}

fn installation_flow() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the installer for 'lcli'!");
    println!("This will install the program to a local directory and add it to your PATH.");
    print!("Do you want to continue? (Y/n) ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") && !input.trim().is_empty() {
        println!("Installation cancelled.");
        pause_for_user();
        return Ok(());
    }

    let install_dir = get_install_dir().ok_or("Could not determine local data directory.")?;
    let target_exe_path = install_dir.join("lcli.exe");

    println!("\nInstalling to: {}", install_dir.display());

    fs::create_dir_all(&install_dir)?;
    let current_exe = env::current_exe()?;
    fs::copy(&current_exe, &target_exe_path)?;

    println!("{}", "[OK] Program copied successfully.".green());

    add_to_path(&install_dir)?;

    println!("\n{}", "Installation complete!".bright_green());
    println!("Please open a NEW terminal to start using the 'lcli' command.");

    pause_for_user();
    Ok(())
}

fn pause_for_user() {
    print!("\nPress Enter to exit...");
    io::stdout().flush().unwrap_or_default();
    io::stdin().read_line(&mut String::new()).unwrap_or_default();
}

#[cfg(windows)]
fn add_to_path(path_to_add: &Path) -> Result<(), io::Error> {
    use winreg::enums::*;
    use winreg::RegKey;

    let path_str = path_to_add.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Path is not valid UTF-8")
    })?;

    println!("Adding to user PATH...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    let mut current_path: String = env.get_value("Path").unwrap_or_default();

    if current_path
        .split(';')
        .any(|p| p.eq_ignore_ascii_case(path_str))
    {
        println!("{}", "Directory is already in PATH. Nothing to do.".yellow());
        return Ok(());
    }

    if !current_path.is_empty() && !current_path.ends_with(';') {
        current_path.push(';');
    }
    current_path.push_str(path_str);

    env.set_value("Path", &current_path)?;
    println!("{}", "[OK] PATH updated successfully.".green());
    Ok(())
}

#[cfg(not(windows))]
fn add_to_path(path_to_add: &Path) -> Result<(), io::Error> {
    println!(
        "Automatic PATH setup is not supported on this OS. Please add this to your PATH manually:"
    );
    println!("{}", path_to_add.display());
    Ok(())
}

#[cfg(windows)]
fn handle_install_command() -> Result<(), io::Error> {
    println!("Running manual installation...");
    let exe_path = env::current_exe()?;
    if let Some(dir) = exe_path.parent() {
        add_to_path(dir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find parent directory of executable.",
        ))
    }
}

#[cfg(not(windows))]
fn handle_install_command() -> Result<(), io::Error> {
    println!("The install command is only supported on Windows.");
    Ok(())
}

fn handle_list_command(path: &Path, is_json: bool) {
    match get_files(path) {
        Ok(files) => {
            if is_json {
                match serde_json::to_string(&files) {
                    Ok(json) => println!("{}", json),
                    Err(_) => eprintln!("{}", "Error: Could not serialize data to JSON.".red()),
                }
            } else {
                print_table(files);
            }
        }
        Err(e) => {
            eprintln!(
                "{} {}: {}",
                "Error reading directory".red(),
                path.display().red(),
                e.red()
            );
        }
    }
}

fn handle_time_command(is_json: bool) {
    let now = Local::now();
    if is_json {
        let output = TimeOutput {
            local_time: now.to_rfc3339(),
            local_timezone: now.format("%Z").to_string(),
        };
        println!("{}", serde_json::to_string(&output).unwrap());
    } else {
        println!(
            "Current Time: {}",
            now.format("%Y-%m-%d %H:%M:%S %Z").bright_blue()
        );
    }
}

fn print_table(get_files: Vec<FileEntry>) {
    let mut table = Table::new(get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
}

fn get_files(path: &Path) -> std::io::Result<Vec<FileEntry>> {
    let mut data = Vec::default();
    for entry in fs::read_dir(path)? {
        if let Ok(file) = entry {
            map_data(file, &mut data);
        }
    }
    Ok(data)
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file.file_name().to_string_lossy().into_owned(),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}