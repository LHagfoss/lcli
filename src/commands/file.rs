use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

pub fn handle_new_file(file_name: &str, path: &Path) {
    let full_path = path.join(file_name);

    match fs::File::create(&full_path) {
        Ok(_) => println!(
            "{} {}",
            "Successfully created file:".green(),
            full_path.display().bright_blue()
        ),
        Err(e) => eprintln!("{} {}", "Error creating file:".red(), e.to_string().red()),
    }
}

pub fn handle_delete_file(path: &Path) {
    match fs::remove_file(path) {
        Ok(_) => println!(
            "{} {}",
            "Successfully deleted file:".green(),
            path.display().bright_blue()
        ),
        Err(e) => eprintln!("{} {}", "Error deleting file:".red(), e.to_string().red()),
    }
}

pub fn handle_rename_file(old_path: &Path, new_path: &Path) {
    match fs::rename(old_path, new_path) {
        Ok(_) => println!(
            "{} {} to {}",
            "Successfully renamed file:".green(),
            old_path.display().bright_blue(),
            new_path.display().bright_blue()
        ),
        Err(e) => eprintln!(
            "{} {}: {}",
            "Error renaming file:".red(),
            old_path.display().red(),
            e.to_string().red()
        ),
    }
}

pub fn handle_move_file(old_path: &Path, new_path: &Path) {
    match fs::rename(old_path, new_path) {
        Ok(_) => println!(
            "{} {} to {}",
            "Successfully moved file:".green(),
            old_path.display().bright_blue(),
            new_path.display().bright_blue()
        ),
        Err(e) => eprintln!(
            "{} {}: {}",
            "Error moving file:".red(),
            old_path.display().red(),
            e.to_string().red()
        ),
    }
}
