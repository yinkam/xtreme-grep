//! # Xtreme Search Mode
//!
//! This module provides ultra-fast search functionality with raw, unformatted output.
//! Perfect for users who need maximum speed and can handle direct file:line:content format.
//!
//! ## Features
//!
//! - **Raw Output**: Direct `file:line:content` format for speed
//! - **No Formatting**: Minimal processing overhead
//! - **Pipe-Friendly**: Perfect for shell pipelines
//! - **Statistics Compatible**: Works with `--stats` flag
//!
//! ## Performance
//!
//! Xtreme mode eliminates formatting overhead by outputting matches immediately
//! in the standard `grep` format. This provides maximum throughput for large
//! codebases or when piping results to other tools.
//!
//! ## Example
//!
//! ```no_run
//! use xerg::search_xtreme::search_files_xtreme;
//! use xerg::colors::Color;
//!
//! let files = vec![std::path::PathBuf::from("src/main.rs")];
//! let pattern = "fn";
//! let color = Color::Red;
//! let rx = search_files_xtreme(&files, pattern, &color, true);
//!
//! // Process raw results from receiver...
//! ```

use crate::colors::Color;
use crate::highlighter::TextHighlighter;
use rayon::scope;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn _process_file_xtreme(
    filepath: &Path,
    pattern: &str,
    color: &Color,
    count_stats: bool,
) -> Result<(usize, usize, usize), std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let highlighter = TextHighlighter::new(pattern, color);

    let mut lines_read = 0;
    let mut matches_found = 0;
    let mut skipped_lines = 0;

    for (line_index, line_result) in reader.lines().enumerate() {
        if count_stats {
            lines_read += 1;
        }

        match line_result {
            Ok(line) => {
                if highlighter.regex.is_match(&line) {
                    // Only count matches if stats are needed
                    if count_stats {
                        let match_count = highlighter.regex.find_iter(&line).count();
                        matches_found += match_count;
                    }

                    let highlighted = highlighter.highlight(&line);

                    // Print immediately for maximum speed
                    println!(
                        "{}: {}:   {}",
                        filepath.display(),
                        line_index + 1,
                        highlighted
                    );
                }
            }
            Err(_) => {
                if count_stats {
                    skipped_lines += 1;
                }
            }
        }
    }

    // Individual file stats are now aggregated and printed only in summary

    Ok((lines_read, matches_found, skipped_lines))
}

/// Search files in xtreme mode with raw output for maximum speed
///
/// Returns a receiver that will contain raw formatted results as they become available.
/// Each result is a vector of strings in the format `filepath:line_number:content`.
///
/// # Arguments
///
/// * `files` - Vector of file paths to search
/// * `pattern` - Pattern to search for
/// * `color` - Color highlighting option
/// * `show_stats` - Whether to include statistics in output
///
/// # Example
///
/// ```no_run
/// use xerg::search_xtreme::search_files_xtreme;
/// use xerg::colors::Color;
///
/// let files = vec![std::path::PathBuf::from("src/main.rs")];
/// let rx = search_files_xtreme(&files, "fn", &Color::Blue, true);
/// ```
pub fn search_files_xtreme(
    files: &[PathBuf],
    pattern: &str,
    color: &Color,
    show_stats: bool,
) -> (usize, usize, usize, usize) {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let total_files = AtomicUsize::new(0);
    let total_lines = AtomicUsize::new(0);
    let total_matches = AtomicUsize::new(0);
    let total_skipped = AtomicUsize::new(0);

    scope(|s| {
        for file in files {
            let _pattern = pattern;
            let _file = file.clone();
            let _total_files = &total_files;
            let _total_lines = &total_lines;
            let _total_matches = &total_matches;
            let _total_skipped = &total_skipped;

            s.spawn(move |_| {
                match _process_file_xtreme(&_file, _pattern, color, show_stats) {
                    Ok((lines, matches, skipped)) => {
                        _total_files.fetch_add(1, Ordering::Relaxed);
                        _total_lines.fetch_add(lines, Ordering::Relaxed);
                        _total_matches.fetch_add(matches, Ordering::Relaxed);
                        _total_skipped.fetch_add(skipped, Ordering::Relaxed);
                    }
                    Err(err) => {
                        println!("# Error reading {}: {}", _file.display(), err);
                    }
                };
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
    fn test_search_files_xtreme_finds_pattern() {
        let temp_dir = TempDir::new("xtreme_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "no match").unwrap();
        writeln!(file, "This is a test pattern").unwrap();
        writeln!(file, "another line").unwrap();

        let files = vec![test_file.clone()];
        let (files_processed, lines, matches, skipped) =
            search_files_xtreme(&files, "pattern", &Color::Blue, true);

        // Should have processed 1 file, 3 lines, 1 match, 0 skipped
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 3);
        assert_eq!(matches, 1);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn test_search_files_xtreme_with_stats() {
        let temp_dir = TempDir::new("xtreme_stats_test").unwrap();
        let test_file = temp_dir.path().join("stats.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "match this").unwrap();
        writeln!(file, "no pattern here").unwrap();
        writeln!(file, "match this too").unwrap();

        let files = vec![test_file.clone()];
        let (files_processed, lines, matches, skipped) =
            search_files_xtreme(&files, "match", &Color::Blue, true);

        // Should have processed 1 file, 3 lines, 2 matches, 0 skipped
        // Note: stats are not printed in the new direct approach, just returned
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 3);
        assert_eq!(matches, 2);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn test_search_files_xtreme_no_match() {
        let temp_dir = TempDir::new("xtreme_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "no match").unwrap();
        writeln!(file, "another line").unwrap();

        let files = vec![test_file.clone()];
        let (files_processed, lines, matches, skipped) =
            search_files_xtreme(&files, "pattern", &Color::Blue, true);

        // Should have processed 1 file, 2 lines, no matches, 0 skipped
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 2);
        assert_eq!(matches, 0);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn test_search_files_xtreme_regex_patterns() {
        let temp_dir = TempDir::new("xtreme_regex_test").unwrap();
        let test_file = temp_dir.path().join("emails.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Contact us at support@example.com").unwrap();
        writeln!(file, "No email on this line").unwrap();
        writeln!(file, "Admin: admin@test.org").unwrap();

        let files = vec![test_file.clone()];

        // Test email regex pattern
        let (files_processed, lines, matches, skipped) =
            search_files_xtreme(&files, r"\w+@\w+\.\w+", &Color::Blue, true);

        // Should have 2 matches (both email lines)
        assert_eq!(files_processed, 1);
        assert_eq!(lines, 3);
        assert_eq!(matches, 2);
        assert_eq!(skipped, 0);

        // Test word boundary regex
        let files2 = vec![test_file];
        let (files_processed2, lines2, matches2, skipped2) =
            search_files_xtreme(&files2, r"\bAdmin\b", &Color::Red, true);

        // Should match only the "Admin:" line, not "admin@test.org"
        assert_eq!(files_processed2, 1);
        assert_eq!(lines2, 3);
        assert_eq!(matches2, 1);
        assert_eq!(skipped2, 0);
    }
}
