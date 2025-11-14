//! # Xtreme Search Mode
//!
//! This module provides ultra-fast search functionality with raw, unformatted output.
//! Perfect for users who need maximum speed and can handle direct file:line:content format.
//!
//! ## Features
//!
//! - **Raw Output**: Direct `file:line:content` format for speed
//! - **No Formatting**: Minimal processing overhead
//! - **Immediate Printing**: Results printed as soon as found
//! - **Shared Reader**: Uses same FileReader as default mode
//! - **Statistics Compatible**: Works with `--stats` flag
//!
//! ## Performance
//!
//! Xtreme mode eliminates messaging overhead by outputting matches immediately
//! in the standard `grep` format. This provides maximum throughput for large
//! codebases or when piping results to other tools.

use crate::colors::Color;
use crate::highlighter::TextHighlighter;
use crate::search::reader::FileReader;
use memmap2::MmapOptions;
use rayon::scope;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

fn _print_match(filepath: &Path, line_number: usize, highlighted_content: &str) {
    println!(
        "{}:{}: {}",
        filepath.display(),
        line_number,
        highlighted_content
    );
}

/// Process a single line and print if it matches, returning match count
fn _process_line(
    filepath: &Path,
    line_index: usize,
    line: &str,
    highlighter: &TextHighlighter,
    show_stats: bool,
) -> usize {
    if highlighter.regex.is_match(line) {
        let match_count = if show_stats {
            highlighter.regex.find_iter(line).count()
        } else {
            0
        };

        let highlighted = highlighter.highlight(line);
        _print_match(filepath, line_index + 1, &highlighted);
        match_count
    } else {
        0
    }
}

/// Process a single file with immediate printing using the specified reader
fn _process_file(
    filepath: &Path,
    highlighter: &TextHighlighter,
    show_stats: bool,
    reader: FileReader,
) -> Result<(usize, usize, usize)> {
    let skipped_lines = 0;

    let (lines_read, matches_found) = match reader {
        FileReader::Streaming => {
            let file = File::open(filepath)?;
            let reader = BufReader::new(file);
            let mut lines_read = 0;
            let mut matches_found = 0;

            for (line_index, line_result) in reader.lines().enumerate() {
                if show_stats {
                    lines_read += 1;
                }

                if let Ok(line) = line_result {
                    matches_found +=
                        _process_line(filepath, line_index, &line, highlighter, show_stats);
                }
                // Skip invalid UTF-8 lines silently
            }

            (lines_read, matches_found)
        }
        FileReader::BulkRead => {
            let content = std::fs::read_to_string(filepath)?;
            let mut lines_read = 0;
            let mut matches_found = 0;

            for (line_index, line) in content.lines().enumerate() {
                if show_stats {
                    lines_read += 1;
                }

                matches_found += _process_line(filepath, line_index, line, highlighter, show_stats);
            }

            (lines_read, matches_found)
        }
        FileReader::MemoryMap => {
            let file = File::open(filepath)?;
            let mmap = unsafe { MmapOptions::new().map(&file)? };
            let content = std::str::from_utf8(&mmap)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let mut lines_read = 0;
            let mut matches_found = 0;

            for (line_index, line) in content.lines().enumerate() {
                if show_stats {
                    lines_read += 1;
                }

                matches_found += _process_line(filepath, line_index, line, highlighter, show_stats);
            }

            (lines_read, matches_found)
        }
    };

    Ok((lines_read, matches_found, skipped_lines))
}

/// Search files in xtreme mode with raw output for maximum speed
pub fn search_files(
    files: &[PathBuf],
    pattern: &str,
    color: &Color,
    show_stats: bool,
) -> (usize, usize, usize, usize) {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let highlighter = TextHighlighter::new(pattern, color);
    let is_single_file = files.len() == 1;

    // Single-file optimization: bypass thread pool overhead
    if is_single_file {
        let file = &files[0];
        let reader = FileReader::select(file, true);

        match _process_file(file, &highlighter, show_stats, reader) {
            Ok((lines, matches, skipped)) => {
                return (1, lines, matches, skipped);
            }
            Err(err) => {
                eprintln!("Error reading {}: {}", file.display(), err);
                return (0, 0, 0, 0);
            }
        }
    }

    // Multi-file processing: use thread pool with streaming reader
    let total_files = AtomicUsize::new(0);
    let total_lines = AtomicUsize::new(0);
    let total_matches = AtomicUsize::new(0);
    let total_skipped = AtomicUsize::new(0);

    scope(|s| {
        for file in files {
            let _pattern = pattern;
            let _file = file.clone();
            let _highlighter = &highlighter;
            let _total_files = &total_files;
            let _total_lines = &total_lines;
            let _total_matches = &total_matches;
            let _total_skipped = &total_skipped;

            s.spawn(move |_| {
                let reader = FileReader::select(&_file, false);
                match _process_file(&_file, _highlighter, show_stats, reader) {
                    Ok((lines, matches, skipped)) => {
                        _total_files.fetch_add(1, Ordering::Relaxed);
                        _total_lines.fetch_add(lines, Ordering::Relaxed);
                        _total_matches.fetch_add(matches, Ordering::Relaxed);
                        _total_skipped.fetch_add(skipped, Ordering::Relaxed);
                    }
                    Err(err) => {
                        eprintln!("Error reading {}: {}", _file.display(), err);
                    }
                }
            });
        }
    });

    (
        total_files.load(Ordering::Relaxed),
        total_lines.load(Ordering::Relaxed),
        total_matches.load(Ordering::Relaxed),
        total_skipped.load(Ordering::Relaxed),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_search_files_finds_pattern() {
        let temp_dir = TempDir::new("xtreme_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "no match").unwrap();
        writeln!(file, "This is a test pattern").unwrap();
        writeln!(file, "another line").unwrap();

        let files = vec![test_file.clone()];
        let (files_processed, lines, matches, skipped) =
            search_files(&files, "pattern", &Color::Blue, true);

        // Should have processed 1 file, 3 lines, 1 match, 0 skipped
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 3);
        assert_eq!(matches, 1);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn test_search_files_with_stats() {
        let temp_dir = TempDir::new("xtreme_stats_test").unwrap();
        let test_file = temp_dir.path().join("stats.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "match this").unwrap();
        writeln!(file, "no pattern here").unwrap();
        writeln!(file, "match this too").unwrap();

        let files = vec![test_file.clone()];
        let (files_processed, lines, matches, skipped) =
            search_files(&files, "match", &Color::Blue, true);

        // Should have processed 1 file, 3 lines, 2 matches, 0 skipped
        // Note: stats are not printed in the new direct approach, just returned
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 3);
        assert_eq!(matches, 2);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn test_search_files_no_match() {
        let temp_dir = TempDir::new("xtreme_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "no match").unwrap();
        writeln!(file, "another line").unwrap();

        let files = vec![test_file.clone()];
        let (files_processed, lines, matches, skipped) =
            search_files(&files, "pattern", &Color::Blue, true);

        // Should have processed 1 file, 2 lines, no matches, 0 skipped
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 2);
        assert_eq!(matches, 0);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn test_search_files_regex_patterns() {
        let temp_dir = TempDir::new("xtreme_regex_test").unwrap();
        let test_file = temp_dir.path().join("emails.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Contact us at support@example.com").unwrap();
        writeln!(file, "No email on this line").unwrap();
        writeln!(file, "Admin: admin@test.org").unwrap();

        let files = vec![test_file.clone()];

        // Test email regex pattern
        let (files_processed, lines, matches, skipped) =
            search_files(&files, r"\w+@\w+\.\w+", &Color::Blue, true);

        // Should have 2 matches (both email lines)
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 3);
        assert_eq!(matches, 2);
        assert_eq!(skipped, 0);

        // Test word boundary regex
        let files2 = vec![test_file];
        let (files_processed2, lines2, matches2, skipped2) =
            search_files(&files2, r"\bAdmin\b", &Color::Red, true);

        // Should match only the "Admin:" line, not "admin@test.org"
        assert_eq!(files_processed2, 1);
        assert_eq!(lines2, 3);
        assert_eq!(matches2, 1);
        assert_eq!(skipped2, 0);
    }
}
