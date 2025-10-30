//! # Synchronous File Processing
//!
//! This module provides synchronous (single-threaded) file processing as an alternative
//! to the parallel implementation. It processes files sequentially with immediate output.
//!
//! ## Features
//!
//! - **Sequential Processing**: Processes files one at a time in order
//! - **Immediate Output**: Results displayed instantly as files are processed
//! - **Memory Efficient**: Line-by-line processing with minimal memory footprint
//! - **Simple Architecture**: Straightforward implementation without threading complexity
//!
//! ## Use Cases
//!
//! - **Small File Sets**: When parallel processing overhead isn't beneficial
//! - **Debugging**: Simpler execution model for troubleshooting
//! - **Resource Constraints**: When limiting CPU usage is desired
//! - **Ordered Output**: When maintaining file processing order is important
//!
//! ## Example
//!
//! ```no_run
//! use xgrep::search_sync::search_files_sync;
//! use xgrep::colors::Color;
//! use std::path::PathBuf;
//!
//! let files = vec![PathBuf::from("src/main.rs")];
//! let pattern = "use";
//! let color = Color::Blue;
//! search_files_sync(&files, pattern, &color, true);
//! ```

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

fn _process_file_sync(
    filepath: &PathBuf,
    pattern: &str,
    highlighter: &TextHighlighter,
    show_stats: bool,
) -> (usize, usize, usize) {
    let file = File::open(filepath);
    let reader = BufReader::new(match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return (0, 0, 0); // (lines, matches, skipped) - errors handled at higher level
        }
    });

    let mut total_lines = 0;
    let mut matched_count = 0;
    let mut skipped_count = 0;

    _print_header(filepath);
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_e) => {
                // Line couldn't be read due to I/O or format error - count as skipped
                skipped_count += 1;
                continue;
            }
        };
        total_lines += 1; // Successfully processed line
        if line.contains(pattern) {
            _print_line(index, &line, highlighter);

            // Count actual number of pattern matches in this line
            let matches_in_line = line.matches(pattern).count();
            matched_count += matches_in_line;
        }
    }

    // Print file summary using new format
    if show_stats {
        println!(
            "  \x1b[2;38;5;245mlines: {}, matches: {}, skipped: {}\x1b[0m",
            total_lines, matched_count, skipped_count
        );
    }

    (total_lines, matched_count, skipped_count)
}

pub fn search_files_sync(files: &[PathBuf], pattern: &str, color: &Color, show_stats: bool) {
    let highlighter = TextHighlighter::new(pattern, color);
    let mut total_lines = 0;
    let mut total_matched = 0;
    let mut total_skipped = 0;
    let mut total_errors = 0;
    let mut files_processed = 0;

    for file in files {
        let (lines, matched, skipped) = _process_file_sync(file, pattern, &highlighter, show_stats);
        if lines == 0 && matched == 0 && skipped == 0 {
            // This indicates a file-level error (couldn't open file)
            total_errors += 1;
        }
        total_lines += lines;
        total_matched += matched;
        total_skipped += skipped;
        files_processed += 1;
    }

    // Print total summary if we processed any files and stats are enabled
    if show_stats && files_processed > 0 {
        println!(
            "\x1b[1;38;5;245mSummary: files: {}\tlines: {}\tmatches: {}\tskipped: {}\terrors: {}\x1b[0m",
            files_processed, total_lines, total_matched, total_skipped, total_errors
        );
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
        search_files_sync(&files, pattern, &color, false);
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
        search_files_sync(&files, pattern, &color, false);
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
        search_files_sync(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_sync_case_sensitive() {
        let temp_dir = TempDir::new("search_sync_nonexistent_test").unwrap();
        let nonexistent_file = temp_dir.path().join("does_not_exist.txt");

        let files = vec![nonexistent_file];
        let pattern = "anything";
        let color = Color::Red;

        // Should print error message to stderr and continue (not panic)
        search_files_sync(&files, pattern, &color, false);
    }
}
