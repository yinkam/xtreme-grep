use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    path: Option<PathBuf>,
}

enum Color {
    Red,
    Green,
    Blue,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn find_files_in_directory(dir: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn colorize_pattern(pattern: &str, text: &str, color: Color) -> String {
    let color_code = match color {
        Color::Red => "31",
        Color::Green => "32",
        Color::Blue => "34",
    };

    let colorized_pattern = format!("\x1b[{}m{}\x1b[0m", color_code, pattern);
    let parts = text.split(pattern).collect::<Vec<&str>>();
    let mut result = String::new();
    for (i, part) in parts.iter().enumerate() {
        result.push_str(part);
        if i < parts.len() - 1 {
            result.push_str(&colorized_pattern);
        }
    }
    result
}

fn search_in_file(filepath: &PathBuf, pattern: &str) {
    let file = File::open(filepath);
    let reader = BufReader::new(match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    });

    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line {}: {}", index + 1, e);
                continue;
            }
        };
        if line.contains(pattern) {
            println!(
                "{}: {}",
                index + 1,
                colorize_pattern(pattern, &line, Color::Red)
            );
        }
    }
}

fn search_in_directory(dir: &PathBuf, pattern: &str) {
    let files = find_files_in_directory(dir);
    for file in files {
        search_in_file(&file, pattern);
    }
}

fn process_path(path: Option<PathBuf>) -> PathBuf {
    match path {
        Some(path) => match path.as_os_str().to_str() {
            Some(".") => std::env::current_dir().unwrap(),
            _ => path.clone(),
        },
        None => std::env::current_dir().unwrap(),
    }
}

fn main() {
    let cli = Cli::parse();

    let path = process_path(cli.path);

    if path.is_file() {
        search_in_file(&path, &cli.pattern);
        return;
    }
    search_in_directory(&path, &cli.pattern);
}
