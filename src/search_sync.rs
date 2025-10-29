use crate::colors::Color;
use crate::highlighter::TextHighlighter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn _print_line(index: usize, line: &str, highlighter: &TextHighlighter) {
    println!(
        "  \x1b[1;38;5;245m{:>3}:\x1b[0m  {}",
        index + 1,
        highlighter.highlight(line)
    );
}

fn _print_header(filepath: &Path) {
    println!("\x1b[1;38;5;245m--- {}\x1b[0m ---", filepath.display());
}

fn _process_file_sync(filepath: &PathBuf, pattern: &str, highlighter: &TextHighlighter) {
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

pub fn search_files_sync(files: &[PathBuf], pattern: &str, color: &Color) {
    let highlighter = TextHighlighter::new(pattern, color);

    for file in files {
        _process_file_sync(file, pattern, &highlighter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_search_files_sync_finds_pattern() {
        let temp_dir = TempDir::new("search_sync_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Hello world").unwrap();
        writeln!(file, "This is a test").unwrap();
        writeln!(file, "Hello again").unwrap();

        let files = vec![test_file];
        let pattern = "Hello";
        let color = Color::Red;

        // Test that sync version completes without panicking
        search_files_sync(&files, pattern, &color);
    }

    #[test]
    fn test_search_files_sync_multiple_files() {
        let temp_dir = TempDir::new("search_sync_multi_test").unwrap();

        // Create first file
        let file1 = temp_dir.path().join("file1.txt");
        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Pattern in file 1").unwrap();
        writeln!(f1, "Some other text").unwrap();

        // Create second file
        let file2 = temp_dir.path().join("file2.txt");
        let mut f2 = File::create(&file2).unwrap();
        writeln!(f2, "Different content").unwrap();
        writeln!(f2, "Pattern in file 2").unwrap();

        let files = vec![file1, file2];
        let pattern = "Pattern";
        let color = Color::Blue;

        // Test that sync version processes files sequentially
        search_files_sync(&files, pattern, &color);
    }

    #[test]
    fn test_search_files_sync_no_matches() {
        let temp_dir = TempDir::new("search_sync_no_match_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Hello world").unwrap();
        writeln!(file, "This is a test").unwrap();

        let files = vec![test_file];
        let pattern = "NotFound";
        let color = Color::Green;

        // Should handle no matches gracefully
        search_files_sync(&files, pattern, &color);
    }

    #[test]
    fn test_search_files_sync_nonexistent_file() {
        let temp_dir = TempDir::new("search_sync_nonexistent_test").unwrap();
        let nonexistent_file = temp_dir.path().join("does_not_exist.txt");

        let files = vec![nonexistent_file];
        let pattern = "anything";
        let color = Color::Red;

        // Should print error message to stderr and continue (not panic)
        search_files_sync(&files, pattern, &color);
    }
}
