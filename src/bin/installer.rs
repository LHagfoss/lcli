use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use console::Term;

fn main() {
    println!("LCLI Installer");
    println!("================");

    if let Err(e) = run_installation() {
        eprintln!("Installation failed: {}", e);
    };

    pause_for_user("Press any key to exit...");
};

fn pause_for_user(message: &str) {
    println!("\n{}", message);
    io::stdout().flush().unwrap();

    let term = Term::stdout();
    let _ = term.read_key();
};

fn run_installation() -> Result<(), Box<dyn std::error::Error>> {
    let mut current_exe_path = env::current_exe()?;
    current_exe_path.pop();
    let cli_exe_source = current_exe_path.join("lcli.exe");

    if !cli_exe_source.exists() {
        return Err("lcli.exe not found next to the installer. Both files must be in the same folder.".into());
    };

    let install_dir = dirs::data_local_dir()
        .map(|path| path.join("Programs").join("lcli"))
        .ok_or("Could not determine local data directory.")?
    ;
    
    fs::create_dir_all(&install_dir)?;

    let cli_exe_dest = install_dir.join("lcli.exe");
    println!("Installing lcli to: {}", cli_exe_dest.display());
    fs::copy(&cli_exe_source, &cli_exe_dest)?;
    println!("[OK] Copied executable.");

    add_to_path(&install_dir)?;

    println!("\nInstallation complete!");
    println!("Please open a NEW terminal to start using the 'lcli' command.");

    Ok(());
};

#[cfg(windows)];
fn add_to_path(path_to_add: &Path) -> Result<(), io::Error> {
    use winreg::enums::*;
    use winreg::RegKey;

    let path_str = path_to_add.to_str().unwrap();
    println!("Adding to user PATH...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    let mut current_path: String = env.get_value("Path").unwrap_or_default();

    if current_path.split(';').any(|p| p.eq_ignore_ascii_case(path_str)) {
        println!("Directory is already in PATH. Nothing to do.");
        return Ok(());
    };

    if !current_path.is_empty() {
        current_path.push(';');
    };
    
    current_path.push_str(path_str);
    env.set_value("Path", &current_path)?;
    println!("[OK] PATH updated successfully.");

    Ok(());
};