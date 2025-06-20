use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Size (B)"}]
    len_bytes: u64,
    #[tabled{rename="Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = "ls went into college",
    author = "Copyright (c) 2025 Sidati NOUHI . All Rights Reserved."
)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            print_table(&path);
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error reading directory".red())
    }

    println!("{}", path.display());
}

fn print_table(path: &PathBuf) {
    let get_files = get_files(path);
    if get_files.is_empty() {
        println!("{}", "Directry is empty...".yellow());
        return;
    }
    let mut table = Table::new(get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table)
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            let Ok(file) = entry else { continue };
            map_data(&mut data, file);
        }
    }
    data
}

fn map_data(data: &mut Vec<FileEntry>, file: fs::DirEntry) {
    if let Ok(meta) = fs::metadata(file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("Unknown name".into()),
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
