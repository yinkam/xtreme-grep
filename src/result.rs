//! # Search Results Management
//!
//! This module handles message formatting and statistics reporting for parallel file search.
//! It manages the display of search results and provides detailed search statistics.
//!
//! ## Features
//!
//! - **Message Formatting**: Structures result messages for consistent display
//! - **Statistics Tracking**: Collects and displays detailed search metrics with timing
//! - **Parallel Communication**: Handles messages from multiple worker threads
//! - **Structured Results**: Provides machine-readable result format
//! - **Real-time Display**: Streams results as they become available
//!
//! ## Result Format
//!
//! When statistics are enabled (`--stats`), the module generates a structured summary:
//!
//! ```text
//! result: files:8; lines:1699; matches:85; skipped:0; errors:0; time:0.002s;
//! ```
//!
//! ## Search Statistics
//!
//! The module tracks comprehensive metrics:
//! - **Files**: Total number of files processed
//! - **Lines**: Total lines read across all files
//! - **Matches**: Total pattern occurrences found
//! - **Skipped**: Lines that couldn't be read due to errors
//! - **Errors**: File-level access failures
//! - **Time**: Total execution time with millisecond precision (3 decimal places)
//!
//! ## Example
//!
//! ```no_run
//! use xgrep::result::{print_result, ResultMessage};
//! use std::sync::mpsc;
//!
//! let (tx, rx) = mpsc::channel();
//! let start_time = std::time::Instant::now();
//! // Send messages from worker threads...
//! print_result(rx, true, start_time); // Print with statistics
//! ```

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Instant;

pub type FileMatchResult = Vec<ResultMessage>;

pub enum ResultMessage {
    Header(PathBuf),
    Line {
        index: usize,
        content: String,
    },
    SearchStats {
        lines: usize,
        matched: usize,
        skipped: usize,
    },
    Error(String),
    Done,
}

fn _print_line(index: usize, content: &str) {
    println!("  \x1b[1;38;5;245m{:>3}:\x1b[0m  {}", index + 1, content);
}

fn _print_header(filepath: &Path) {
    println!("\x1b[1;38;5;245m--- {}\x1b[0m ---", filepath.display());
}

fn _print_line_stats(lines: usize, matched: usize, skipped: usize) {
    println!(
        "  \x1b[2;38;5;245mlines: {}, matches: {}, skipped: {}\x1b[0m",
        lines, matched, skipped
    );
}

fn _print_result_stats(
    files: usize,
    lines: usize,
    matched: usize,
    skipped: usize,
    errors: usize,
    elapsed_secs: f64,
) {
    println!(
        "\x1b[1;38;5;245mresult: files:{}; lines:{}; matches:{}; skipped:{}; errors:{}; time:{:.3}s;\x1b[0m",
        files, lines, matched, skipped, errors, elapsed_secs
    );
}

pub fn print_result(rx: mpsc::Receiver<FileMatchResult>, show_stats: bool, start_time: Instant) {
    let mut total_lines = 0;
    let mut total_matched = 0;
    let mut total_skipped = 0;
    let mut total_errors = 0;
    let mut files_processed = 0;

    for message in rx {
        for msg in message {
            match msg {
                ResultMessage::Header(path) => {
                    _print_header(&path);
                }
                ResultMessage::Line { index, content } => {
                    _print_line(index, &content);
                }
                ResultMessage::SearchStats {
                    lines,
                    matched,
                    skipped,
                } => {
                    if show_stats {
                        _print_line_stats(lines, matched, skipped);
                    }
                    total_lines += lines;
                    total_matched += matched;
                    total_skipped += skipped;
                    files_processed += 1;
                }
                ResultMessage::Error(err) => {
                    eprintln!("Error: {}", err);
                    total_errors += 1;
                }
                ResultMessage::Done => break,
            }
        }
    }

    // Print total summary if we processed any files and stats are enabled
    if show_stats && files_processed > 0 {
        let elapsed_secs = start_time.elapsed().as_secs_f64();
        _print_result_stats(
            files_processed,
            total_lines,
            total_matched,
            total_skipped,
            total_errors,
            elapsed_secs,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::mpsc;

    #[test]
    fn test_result_message_variants() {
        // Test that all ResultMessage variants can be created
        let header = ResultMessage::Header(PathBuf::from("test.txt"));
        let line = ResultMessage::Line {
            index: 0,
            content: "test content".to_string(),
        };
        let stats = ResultMessage::SearchStats {
            lines: 10,
            matched: 5,
            skipped: 2,
        };
        let error = ResultMessage::Error("test error".to_string());
        let done = ResultMessage::Done;

        // Just test that they compile and can be matched
        match header {
            ResultMessage::Header(_) => {}
            _ => panic!("Header variant failed"),
        }
        match line {
            ResultMessage::Line { .. } => {}
            _ => panic!("Line variant failed"),
        }
        match stats {
            ResultMessage::SearchStats { .. } => {}
            _ => panic!("SearchStats variant failed"),
        }
        match error {
            ResultMessage::Error(_) => {}
            _ => panic!("Error variant failed"),
        }
        match done {
            ResultMessage::Done => {}
            _ => panic!("Done variant failed"),
        }
    }

    #[test]
    fn test_print_result_with_stats() {
        let (tx, rx) = mpsc::channel();

        // Create a test file result with stats
        let messages = vec![
            ResultMessage::Header(PathBuf::from("test.txt")),
            ResultMessage::Line {
                index: 0,
                content: "found match".to_string(),
            },
            ResultMessage::SearchStats {
                lines: 5,
                matched: 1,
                skipped: 0,
            },
            ResultMessage::Done,
        ];

        tx.send(messages).unwrap();
        drop(tx);

        // This test mainly ensures the function doesn't panic
        // Results go to stdout so we can't easily capture it in tests
        print_result(rx, true, Instant::now());
    }

    #[test]
    fn test_print_result_without_stats() {
        let (tx, rx) = mpsc::channel();

        // Create a test file result without stats display
        let messages = vec![
            ResultMessage::Header(PathBuf::from("test.txt")),
            ResultMessage::Line {
                index: 0,
                content: "found match".to_string(),
            },
            ResultMessage::SearchStats {
                lines: 5,
                matched: 1,
                skipped: 0,
            },
            ResultMessage::Done,
        ];

        tx.send(messages).unwrap();
        drop(tx);

        // This should not display stats
        print_result(rx, false, Instant::now());
    }

    #[test]
    fn test_print_result_with_errors() {
        let (tx, rx) = mpsc::channel();

        // Create a test with errors
        let messages = vec![
            ResultMessage::Header(PathBuf::from("test.txt")),
            ResultMessage::Error("Failed to read file".to_string()),
            ResultMessage::SearchStats {
                lines: 0,
                matched: 0,
                skipped: 5,
            },
            ResultMessage::Done,
        ];

        tx.send(messages).unwrap();
        drop(tx);

        // This test ensures error handling works
        print_result(rx, true, Instant::now());
    }

    #[test]
    fn test_print_result_multiple_files() {
        let (tx, rx) = mpsc::channel();

        // First file
        let messages1 = vec![
            ResultMessage::Header(PathBuf::from("file1.txt")),
            ResultMessage::Line {
                index: 0,
                content: "match in file 1".to_string(),
            },
            ResultMessage::SearchStats {
                lines: 10,
                matched: 2,
                skipped: 0,
            },
            ResultMessage::Done,
        ];

        // Second file
        let messages2 = vec![
            ResultMessage::Header(PathBuf::from("file2.txt")),
            ResultMessage::Line {
                index: 5,
                content: "match in file 2".to_string(),
            },
            ResultMessage::SearchStats {
                lines: 8,
                matched: 1,
                skipped: 1,
            },
            ResultMessage::Done,
        ];

        tx.send(messages1).unwrap();
        tx.send(messages2).unwrap();
        drop(tx);

        // Test multiple files with summary
        print_result(rx, true, Instant::now());
    }

    #[test]
    fn test_print_result_empty_results() {
        let (tx, rx) = mpsc::channel();
        drop(tx); // No messages sent

        // Should handle empty results gracefully
        print_result(rx, true, Instant::now());
    }

    #[test]
    fn test_file_match_result_type() {
        // Test the type alias works correctly
        let result: FileMatchResult = vec![
            ResultMessage::Header(PathBuf::from("test.txt")),
            ResultMessage::Done,
        ];

        assert_eq!(result.len(), 2);
        match &result[0] {
            ResultMessage::Header(path) => {
                assert_eq!(path, &PathBuf::from("test.txt"));
            }
            _ => panic!("Expected Header message"),
        }
    }

    #[test]
    fn test_search_stats_fields() {
        // Test SearchStats field access
        let stats = ResultMessage::SearchStats {
            lines: 100,
            matched: 25,
            skipped: 3,
        };

        if let ResultMessage::SearchStats {
            lines,
            matched,
            skipped,
        } = stats
        {
            assert_eq!(lines, 100);
            assert_eq!(matched, 25);
            assert_eq!(skipped, 3);
        } else {
            panic!("Expected SearchStats variant");
        }
    }
}
