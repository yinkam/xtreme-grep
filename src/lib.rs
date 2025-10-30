//! # XGrep - A High-Performance Rust Grep Implementation
//!
//! XGrep is a fast, parallel grep implementation written in Rust that searches for patterns
//! in files and directories with syntax highlighting and detailed search statistics.
//!
//! ## Features
//!
//! - **Parallel Processing**: Multi-core file processing with intelligent thread pool management
//! - **Pattern Matching**: Regular expression engine with optimized performance
//! - **Structured Streaming**: Organized results with comprehensive statistics and timing
//! - **Directory Traversal**: Recursive scanning with symlink support
//! - **Colorized Output**: Customizable syntax highlighting (red, green, blue, bold)
//! - **Search Statistics**: Structured result format with timing metrics using `--stats`
//!
//! ## Usage
//!
//! ```no_run
//! use xerg::{run, colors::Color};
//! use std::path::PathBuf;
//!
//! let dir = PathBuf::from(".");
//! let pattern = "use";
//! let color = Color::Blue;
//! let show_stats = true;
//!
//! run(&dir, pattern, &color, show_stats);
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several focused modules:
//!
//! - [`colors`]: ANSI color management and formatting
//! - [`crawler`]: Directory traversal with symlink support
//! - [`highlighter`]: Regex-based text highlighting
//! - [`result`]: Message handling and statistics result formatting
//! - [`search`]: Parallel file processing with Rayon
//! - [`search_sync`]: Synchronous file processing implementation

pub mod colors;
pub mod crawler;
pub mod highlighter;
pub mod result;
pub mod search;
pub mod search_sync;

use colors::Color;
use crawler::get_files;
use result::print_result;
use search::search_files;
use std::path::PathBuf;
use std::time::Instant;

pub fn run(dir: &PathBuf, pattern: &str, color: &Color, show_stats: bool) {
    let start_time = Instant::now();
    let files = get_files(dir);
    let rx = search_files(&files, pattern, color, show_stats);

    print_result(rx, show_stats, start_time);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_run_integration() {
        // Test the main run function integrates all modules correctly
        let temp_dir = TempDir::new("lib_test").unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Hello world").unwrap();
        writeln!(file, "This is a test").unwrap();

        let pattern = "Hello";
        let color = Color::Red;

        // Test that run function completes without panicking
        // This tests integration of crawler::get_files and search::search_files
        run(&temp_dir.path().to_path_buf(), pattern, &color, false);
    }

    #[test]
    fn test_run_with_single_file() {
        // Test run function with a single file instead of directory
        let temp_dir = TempDir::new("lib_single_test").unwrap();
        let test_file = temp_dir.path().join("single.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Pattern match here").unwrap();

        let pattern = "Pattern";
        let color = Color::Blue;

        // Test run with single file path
        run(&test_file, pattern, &color, false);
    }

    #[test]
    fn test_run_with_no_matches() {
        // Test run function when no matches are found
        let temp_dir = TempDir::new("lib_no_match_test").unwrap();
        let test_file = temp_dir.path().join("nomatch.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "This file has no pattern").unwrap();

        let pattern = "NonExistentPattern";
        let color = Color::Green;

        // Should handle no matches gracefully
        run(&temp_dir.path().to_path_buf(), pattern, &color, false);
    }

    #[test]
    fn test_run_different_colors() {
        // Test run function with all color variants
        let temp_dir = TempDir::new("lib_colors_test").unwrap();
        let test_file = temp_dir.path().join("colors.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Test pattern").unwrap();

        let pattern = "pattern";

        // Test all color variants
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Red, false);
        run(
            &temp_dir.path().to_path_buf(),
            pattern,
            &Color::Green,
            false,
        );
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Blue, false);
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Bold, false);
    }
}
