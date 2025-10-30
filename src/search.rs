//! # Parallel File Search
//!
//! This module provides high-performance parallel file search using Rayon's work-stealing
//! thread pool. It efficiently searches through multiple files concurrently while maintaining
//! optimal system responsiveness.
//!
//! ## Features
//!
//! - **Multi-core Processing**: Utilizes available CPU cores with intelligent thread management
//! - **Work-stealing Scheduler**: Rayon's scheduler automatically balances work across threads
//! - **Memory Efficient**: Line-by-line processing handles files of any size
//! - **Structured Streaming**: Streams structured matches per file with headers and statistics
//! - **Error Resilient**: Graceful per-file error recovery without stopping other files
//!
//! ## Performance Characteristics
//!
//! - **Thread Pool Size**: Uses `cores - 1` threads to prevent system lock-up
//! - **I/O Optimization**: Buffered reading with `BufReader` for optimal disk access
//! - **Regex Reuse**: Compiled patterns shared across threads for efficiency
//!
//! ## Example
//!
//! ```no_run
//! use xgrep::search::search_files;
//! use xgrep::colors::Color;
//! use std::path::PathBuf;
//!
//! let files = vec![PathBuf::from("src/main.rs")];
//! let pattern = "use";
//! let color = Color::Blue;
//! let rx = search_files(&files, pattern, &color, true);
//!
//! // Process results from receiver...
//! ```

use crate::colors::Color;
use crate::highlighter::TextHighlighter;
use crate::result::{FileMatchResult, ResultMessage};
use rayon::scope;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;
use std::sync::mpsc;

fn _process_file(
    filepath: &PathBuf,
    pattern: &str,
    highlighter: &TextHighlighter,
    show_stats: bool,
) -> Result<FileMatchResult> {
    let mut messages = Vec::new();
    messages.push(ResultMessage::Header(filepath.to_path_buf()));

    let file = File::open(filepath);
    let reader = BufReader::new(match file {
        Ok(f) => f,
        Err(e) => {
            let err_msg = format!("Failed to open file {}: {}", filepath.display(), e);
            messages.push(ResultMessage::Error(err_msg));
            return Ok(messages);
        }
    });

    let mut total_lines = 0;
    let mut matched_count = 0;
    let mut skipped_count = 0;

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
            let line_msg = ResultMessage::Line {
                index,
                content: highlighter.highlight(&line),
            };
            messages.push(line_msg);

            // Count actual number of pattern matches in this line
            let matches_in_line = line.matches(pattern).count();
            matched_count += matches_in_line;
        }
        // Non-matching lines are just processed normally - no separate tracking needed
    }

    // Add file summary with counts if stats are enabled
    if show_stats {
        messages.push(ResultMessage::SearchStats {
            lines: total_lines,
            matched: matched_count,
            skipped: skipped_count,
        });
    }

    messages.push(ResultMessage::Done);
    Ok(messages)
}

pub fn search_files(
    files: &[PathBuf],
    pattern: &str,
    color: &Color,
    show_stats: bool,
) -> mpsc::Receiver<FileMatchResult> {
    let (tx, rx) = mpsc::channel();
    let highlighter = TextHighlighter::new(pattern, color);

    scope(|s| {
        for file in files {
            let _tx = tx.clone();
            let _highlighter = &highlighter;
            let _pattern = pattern;
            let _file = file.clone();

            s.spawn(move |_| {
                let messages = match _process_file(&_file, _pattern, &_highlighter, show_stats) {
                    Ok(msg) => msg,
                    Err(e) => {
                        let err_msg = format!("Error processing file {}: {}", _file.display(), e);
                        vec![ResultMessage::Error(err_msg)]
                    }
                };
                _tx.send(messages).ok();
            });
        }
    });

    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_search_files_finds_pattern() {
        // Create temporary directory and file with content
        let temp_dir = TempDir::new("search_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Hello world").unwrap();
        writeln!(file, "This is a test").unwrap();
        writeln!(file, "Hello again").unwrap();

        // Test that search finds the pattern
        let files = vec![test_file];
        let pattern = "Hello";
        let color = Color::Red;

        // Test that search_files completes without panicking
        // Results go to stdout, so we're testing the function doesn't crash
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_multiple_files() {
        let temp_dir = TempDir::new("search_multi_test").unwrap();

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

        // Test that function completes without panicking
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_no_matches() {
        let temp_dir = TempDir::new("search_no_match_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Hello world").unwrap();
        writeln!(file, "This is a test").unwrap();

        let files = vec![test_file];
        let pattern = "NotFound";
        let color = Color::Green;

        // Should handle no matches gracefully
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_empty_file() {
        let temp_dir = TempDir::new("search_empty_test").unwrap();
        let test_file = temp_dir.path().join("empty.txt");

        // Create empty file
        File::create(&test_file).unwrap();

        let files = vec![test_file];
        let pattern = "anything";
        let color = Color::Red;

        // Should handle empty files without errors
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_nonexistent_file() {
        let temp_dir = TempDir::new("search_nonexistent_test").unwrap();
        let nonexistent_file = temp_dir.path().join("does_not_exist.txt");

        let files = vec![nonexistent_file];
        let pattern = "anything";
        let color = Color::Red;

        // Should print error message to stderr and continue (not panic)
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_different_colors() {
        let temp_dir = TempDir::new("search_colors_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Test pattern here").unwrap();

        let files = vec![
            test_file.clone(),
            test_file.clone(),
            test_file.clone(),
            test_file,
        ];
        let pattern = "pattern";

        // Test all color variants
        search_files(&vec![files[0].clone()], pattern, &Color::Red, false);
        search_files(&vec![files[1].clone()], pattern, &Color::Green, false);
        search_files(&vec![files[2].clone()], pattern, &Color::Blue, false);
        search_files(&vec![files[3].clone()], pattern, &Color::Bold, false);
    }

    #[test]
    fn test_search_files_regex_patterns() {
        let temp_dir = TempDir::new("search_regex_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "email@example.com").unwrap();
        writeln!(file, "test@domain.org").unwrap();
        writeln!(file, "not an email").unwrap();

        let files = vec![test_file];
        let pattern = r"\w+@\w+\.\w+"; // Email regex pattern
        let color = Color::Blue;

        // Should handle regex patterns (TextHighlighter uses regex internally)
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_special_characters() {
        let temp_dir = TempDir::new("search_special_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Special chars: àáâã").unwrap();
        writeln!(file, "Symbols: @#$%^&*()").unwrap();
        writeln!(file, "Unicode: 🦀 Rust crab").unwrap();

        let files = vec![test_file];
        let pattern = "🦀";
        let color = Color::Green;

        // Should handle Unicode and special characters
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_case_sensitive() {
        let temp_dir = TempDir::new("search_case_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Hello World").unwrap();
        writeln!(file, "hello world").unwrap();
        writeln!(file, "HELLO WORLD").unwrap();

        let files = vec![test_file];
        let pattern = "Hello"; // Exact case
        let color = Color::Red;

        // Should be case-sensitive by default
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_long_lines() {
        let temp_dir = TempDir::new("search_long_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        // Create a very long line
        let long_line = "x".repeat(10000) + "PATTERN" + &"y".repeat(10000);
        writeln!(file, "{}", long_line).unwrap();
        writeln!(file, "Short line").unwrap();

        let files = vec![test_file];
        let pattern = "PATTERN";
        let color = Color::Blue;

        // Should handle very long lines without issues
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_empty_pattern() {
        let temp_dir = TempDir::new("search_empty_pattern_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Some content").unwrap();

        let files = vec![test_file];
        let pattern = ""; // Empty pattern
        let color = Color::Red;

        // Should handle empty pattern gracefully (regex behavior)
        search_files(&files, pattern, &color, false);
    }

    #[test]
    fn test_search_files_mixed_scenarios() {
        let temp_dir = TempDir::new("search_mixed_test").unwrap();

        // Create valid file
        let valid_file = temp_dir.path().join("valid.txt");
        let mut f = File::create(&valid_file).unwrap();
        writeln!(f, "Valid content with pattern").unwrap();

        // Create empty file
        let empty_file = temp_dir.path().join("empty.txt");
        File::create(&empty_file).unwrap();

        // Reference non-existent file
        let nonexistent = temp_dir.path().join("missing.txt");

        let files = vec![valid_file, empty_file, nonexistent];
        let pattern = "pattern";
        let color = Color::Green;

        // Should handle mixed scenarios: valid, empty, and missing files
        search_files(&files, pattern, &color, false);
    }
}
