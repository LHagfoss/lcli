use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{fs, path::Path};
use strum::Display;
use tabled::{
    settings::{
        object::Columns,
        Color,
    }, Table,
    Tabled,
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "File Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Last Modified")]
    modified: String,
}

pub fn handle_ls_command(path: &Path, is_json: bool) {
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

fn print_table(get_files: Vec<FileEntry>) {
    let mut table = Table::new(get_files);

    table.with(tabled::settings::Style::rounded());
    table.modify(Columns::first(), Color::FG_WHITE);
    table.modify(Columns::new(0..4), Color::FG_BRIGHT_WHITE);

    println!("{}", table);
}

fn get_files(path: &Path) -> std::io::Result<Vec<FileEntry>> {
    let mut data = Vec::default();

    for file in fs::read_dir(path)?.flatten() {
        map_data(file, &mut data);
    }

    Ok(data)
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(file.path()) {
        data.push(FileEntry {
            name: file.file_name().to_string_lossy().into_owned(),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            size: format_size(meta.len()),
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}


/// monkey readable formatting for size
fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

