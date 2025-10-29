pub mod colors;
pub mod crawler;
pub mod highlighter;
pub mod output;
pub mod search;
pub mod search_sync;

use colors::Color;
use crawler::get_files;
use output::print_output;
use search::search_files;
use std::path::PathBuf;

pub fn run(dir: &PathBuf, pattern: &str, color: &Color) {
    let files = get_files(dir);
    let rx = search_files(&files, pattern, color);

    print_output(rx);
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
        run(&temp_dir.path().to_path_buf(), pattern, &color);
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
        run(&test_file, pattern, &color);
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
        run(&temp_dir.path().to_path_buf(), pattern, &color);
    }

    #[test]
    fn test_run_with_different_colors() {
        // Test run function with all color variants
        let temp_dir = TempDir::new("lib_colors_test").unwrap();
        let test_file = temp_dir.path().join("colors.txt");

        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "Test pattern").unwrap();

        let pattern = "pattern";

        // Test all color variants
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Red);
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Green);
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Blue);
        run(&temp_dir.path().to_path_buf(), pattern, &Color::Bold);
    }
}
