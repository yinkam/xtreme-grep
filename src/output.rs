use std::path::{Path, PathBuf};
use std::sync::mpsc;

pub type FileMatchResult = Vec<OutputMessage>;

pub enum OutputMessage {
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

fn _print_summary_stats(files: usize, lines: usize, matched: usize, skipped: usize, errors: usize) {
    println!(
        "\x1b[1;38;5;245mSummary: files: {}\tlines: {}\tmatches: {}\tskipped: {}\terrors: {}\x1b[0m",
        files, lines, matched, skipped, errors
    );
}

pub fn print_output(rx: mpsc::Receiver<FileMatchResult>, show_stats: bool) {
    let mut total_lines = 0;
    let mut total_matched = 0;
    let mut total_skipped = 0;
    let mut total_errors = 0;
    let mut files_processed = 0;

    for message in rx {
        for msg in message {
            match msg {
                OutputMessage::Header(path) => {
                    _print_header(&path);
                }
                OutputMessage::Line { index, content } => {
                    _print_line(index, &content);
                }
                OutputMessage::SearchStats {
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
                OutputMessage::Error(err) => {
                    eprintln!("Error: {}", err);
                    total_errors += 1;
                }
                OutputMessage::Done => break,
            }
        }
    }

    // Print total summary if we processed any files and stats are enabled
    if show_stats && files_processed > 0 {
        _print_summary_stats(
            files_processed,
            total_lines,
            total_matched,
            total_skipped,
            total_errors,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::path::PathBuf;

    #[test]
    fn test_output_message_variants() {
        // Test that all OutputMessage variants can be created
        let header = OutputMessage::Header(PathBuf::from("test.txt"));
        let line = OutputMessage::Line {
            index: 0,
            content: "test content".to_string(),
        };
        let stats = OutputMessage::SearchStats {
            lines: 10,
            matched: 5,
            skipped: 2,
        };
        let error = OutputMessage::Error("test error".to_string());
        let done = OutputMessage::Done;

        // Just test that they compile and can be matched
        match header {
            OutputMessage::Header(_) => {},
            _ => panic!("Header variant failed"),
        }
        match line {
            OutputMessage::Line { .. } => {},
            _ => panic!("Line variant failed"),
        }
        match stats {
            OutputMessage::SearchStats { .. } => {},
            _ => panic!("SearchStats variant failed"),
        }
        match error {
            OutputMessage::Error(_) => {},
            _ => panic!("Error variant failed"),
        }
        match done {
            OutputMessage::Done => {},
            _ => panic!("Done variant failed"),
        }
    }

    #[test]
    fn test_print_output_with_stats() {
        let (tx, rx) = mpsc::channel();
        
        // Create a test file result with stats
        let messages = vec![
            OutputMessage::Header(PathBuf::from("test.txt")),
            OutputMessage::Line {
                index: 0,
                content: "found match".to_string(),
            },
            OutputMessage::SearchStats {
                lines: 5,
                matched: 1,
                skipped: 0,
            },
            OutputMessage::Done,
        ];
        
        tx.send(messages).unwrap();
        drop(tx);
        
        // This test mainly ensures the function doesn't panic
        // Output goes to stdout so we can't easily capture it in tests
        print_output(rx, true);
    }

    #[test]
    fn test_print_output_without_stats() {
        let (tx, rx) = mpsc::channel();
        
        // Create a test file result without stats display
        let messages = vec![
            OutputMessage::Header(PathBuf::from("test.txt")),
            OutputMessage::Line {
                index: 0,
                content: "found match".to_string(),
            },
            OutputMessage::SearchStats {
                lines: 5,
                matched: 1,
                skipped: 0,
            },
            OutputMessage::Done,
        ];
        
        tx.send(messages).unwrap();
        drop(tx);
        
        // This should not display stats
        print_output(rx, false);
    }

    #[test]
    fn test_print_output_with_errors() {
        let (tx, rx) = mpsc::channel();
        
        // Create a test with errors
        let messages = vec![
            OutputMessage::Header(PathBuf::from("test.txt")),
            OutputMessage::Error("Failed to read file".to_string()),
            OutputMessage::SearchStats {
                lines: 0,
                matched: 0,
                skipped: 5,
            },
            OutputMessage::Done,
        ];
        
        tx.send(messages).unwrap();
        drop(tx);
        
        // This test ensures error handling works
        print_output(rx, true);
    }

    #[test]
    fn test_print_output_multiple_files() {
        let (tx, rx) = mpsc::channel();
        
        // First file
        let messages1 = vec![
            OutputMessage::Header(PathBuf::from("file1.txt")),
            OutputMessage::Line {
                index: 0,
                content: "match in file 1".to_string(),
            },
            OutputMessage::SearchStats {
                lines: 10,
                matched: 2,
                skipped: 0,
            },
            OutputMessage::Done,
        ];
        
        // Second file
        let messages2 = vec![
            OutputMessage::Header(PathBuf::from("file2.txt")),
            OutputMessage::Line {
                index: 5,
                content: "match in file 2".to_string(),
            },
            OutputMessage::SearchStats {
                lines: 8,
                matched: 1,
                skipped: 1,
            },
            OutputMessage::Done,
        ];
        
        tx.send(messages1).unwrap();
        tx.send(messages2).unwrap();
        drop(tx);
        
        // Test multiple files with summary
        print_output(rx, true);
    }

    #[test]
    fn test_print_output_empty_results() {
        let (tx, rx) = mpsc::channel();
        drop(tx); // No messages sent
        
        // Should handle empty results gracefully
        print_output(rx, true);
    }

    #[test] 
    fn test_file_match_result_type() {
        // Test the type alias works correctly
        let result: FileMatchResult = vec![
            OutputMessage::Header(PathBuf::from("test.txt")),
            OutputMessage::Done,
        ];
        
        assert_eq!(result.len(), 2);
        match &result[0] {
            OutputMessage::Header(path) => {
                assert_eq!(path, &PathBuf::from("test.txt"));
            },
            _ => panic!("Expected Header message"),
        }
    }

    #[test]
    fn test_search_stats_fields() {
        // Test SearchStats field access
        let stats = OutputMessage::SearchStats {
            lines: 100,
            matched: 25,
            skipped: 3,
        };
        
        if let OutputMessage::SearchStats { lines, matched, skipped } = stats {
            assert_eq!(lines, 100);
            assert_eq!(matched, 25);
            assert_eq!(skipped, 3);
        } else {
            panic!("Expected SearchStats variant");
        }
    }
}
