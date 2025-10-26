use crate::highlighter::TextHighlighter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn search_file(filepath: &PathBuf, pattern: &str, highlighter: &TextHighlighter) {
    let file = File::open(filepath);
    let reader = BufReader::new(match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    });

    println!("{:?}", filepath);
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        if line.contains(pattern) {
            println!("Line {}:\t {}", index + 1, highlighter.highlight(&line));
        }
    }
}

pub fn search_directory(files: &Vec<PathBuf>, pattern: &str, highlighter: &TextHighlighter) {
    for file in files {
        search_file(&file, pattern, highlighter)
    }
}
