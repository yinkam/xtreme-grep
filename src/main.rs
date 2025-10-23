use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    filepath: PathBuf,
}

enum Color {
    Red,
    Green,
    Blue,
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

fn main() {
    let cli = Cli::parse();
    let file = File::open(&cli.filepath);
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
        if line.contains(&cli.pattern) {
            println!(
                "{}: {}",
                index + 1,
                colorize_pattern(&cli.pattern, &line, Color::Red)
            );
        }
    }
}
