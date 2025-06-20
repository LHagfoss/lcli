use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path};
use std::process::Command;
use console::Term;
use dirs;
use winreg;

fn main() {
    println!("LCLI Uninstaller");
    println!("================");

    println!("\nThis will remove lcli from your PATH and delete the application files.");
    print!("Are you sure you want to continue? (y/N) ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Uninstall cancelled.");
    } else {
        if let Err(e) = run_uninstallation() {
            eprintln!("\nUninstallation failed: {}", e);
        };
    };

    pause_for_user("Press any key to exit...");
}

fn pause_for_user(message: &str) {
    println!("\n{}", message);
    io::stdout().flush().unwrap();

    let term = Term::stdout();
    let _ = term.read_key();
}

fn run_uninstallation() -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = dirs::data_local_dir()
        .map(|path| path.join("Programs").join("lcli"))
        .ok_or("Could not determine local data directory.")?;

    if !install_dir.exists() {
        println!("Installation directory not found. Nothing to do.");
        return Ok(());
    }

    remove_from_path(&install_dir)?;

    create_and_run_deleter_script(&install_dir)?;
    
    println!("\n[OK] Uninstallation complete.");
    println!("The application files will be deleted after this window closes.");

    Ok(())
}

fn create_and_run_deleter_script(install_dir: &Path) -> io::Result<()> {
    let temp_dir = env::temp_dir();
    let bat_path = temp_dir.join("lcli_deleter.bat");

    let script_content = format!(
        "@echo off\n\
         timeout /t 1 /nobreak > nul\n\
         rd /s /q \"{}\"\n\
         del \"{}\"\n",
        install_dir.display(),
        bat_path.display()
    );

    fs::write(&bat_path, script_content)?;

    Command::new("cmd")
        .arg("/C")
        .arg(bat_path)
        .spawn()?;

    Ok(())
}


#[cfg(windows)]
fn remove_from_path(path_to_remove: &Path) -> io::Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let path_str_to_remove = path_to_remove.to_str().unwrap();
    println!("Removing from user PATH...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    let current_path: String = env.get_value("Path").unwrap_or_default();

    let new_path: String = current_path
        .split(';')
        .filter(|p| !p.eq_ignore_ascii_case(path_str_to_remove) && !p.is_empty())
        .collect::<Vec<&str>>()
        .join(";");

    if new_path == current_path {
        println!("Directory was not found in PATH. Nothing to do.");
    } else {
        env.set_value("Path", &new_path)?;
        println!("[OK] PATH updated successfully.");
    }

    Ok(())
}