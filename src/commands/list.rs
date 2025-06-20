use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs,
    path::{Path},
};
use strum::Display;
use tabled::{
    settings::{
        object::{Columns, Rows},
        Color,
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

pub fn handle_list_command(path: &Path, is_json: bool) {
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
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::new(2..3), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::new(3..4), Color::FG_BRIGHT_YELLOW);
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
    if let Ok(meta) = fs::metadata(file.path()) {
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