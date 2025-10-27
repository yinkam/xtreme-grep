use crate::colors::Color;
use crate::highlighter::TextHighlighter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn _print_line(index: usize, line: &str, highlighter: &TextHighlighter) {
    println!(
        "\x1b[1m{:>3}:\x1b[0m  {}",
        index + 1,
        highlighter.highlight(line)
    );
}

fn _print_header(filepath: &PathBuf) {
    println!("--- \x1b[1m{:?}\x1b[0m ---", filepath);
}

fn _process_file(filepath: &PathBuf, pattern: &str, highlighter: &TextHighlighter) {
    let file = File::open(filepath);
    let reader = BufReader::new(match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    });

    _print_header(filepath);
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_e) => {
                // suppress line read errors for cleaner output
                continue;
            }
        };
        if line.contains(pattern) {
            _print_line(index, &line, highlighter);
        }
    }
}

pub fn search_files(files: &Vec<PathBuf>, pattern: &str, color: &Color) {
    let highlighter = TextHighlighter::new(pattern, color);

    for file in files {
        _process_file(&file, pattern, &highlighter)
    }
}
