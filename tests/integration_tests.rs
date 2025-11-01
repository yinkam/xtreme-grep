use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempdir::TempDir;
use xerg::colors::Color;
use xerg::highlighter::TextHighlighter;

/// Helper function to run xerg command and capture output
fn run_xerg(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute xerg");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

/// Helper function to create test files in a temporary directory
fn create_test_files(temp_dir: &TempDir) -> std::path::PathBuf {
    let test_dir = temp_dir.path().join("test_files");
    fs::create_dir(&test_dir).unwrap();

    // Create various test files
    let mut file1 = File::create(test_dir.join("file1.txt")).unwrap();
    writeln!(file1, "Hello world").unwrap();
    writeln!(file1, "This is a test file").unwrap();
    writeln!(file1, "It contains some sample text").unwrap();

    let mut file2 = File::create(test_dir.join("file2.rs")).unwrap();
    writeln!(file2, "fn main() {{").unwrap();
    writeln!(file2, "    println!(\"Hello Rust!\");").unwrap();
    writeln!(file2, "}}").unwrap();

    let mut file3 = File::create(test_dir.join("empty.txt")).unwrap();
    file3.flush().unwrap(); // Empty file

    // Create subdirectory with more files
    let sub_dir = test_dir.join("subdir");
    fs::create_dir(&sub_dir).unwrap();
    let mut file4 = File::create(sub_dir.join("nested.py")).unwrap();
    writeln!(file4, "def main():").unwrap();
    writeln!(file4, "    print('Hello Python!')").unwrap();

    test_dir
}

#[test]
fn test_basic_search() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    let (stdout, stderr, exit_code) = run_xerg(&["Hello", test_dir.to_str().unwrap()]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());

    // Use our highlighter to generate the expected highlighted text
    let highlighter = TextHighlighter::new("Hello", &Color::Red);
    let expected_hello_world = highlighter.highlight("Hello world");
    let expected_hello_rust = highlighter.highlight("    println!(\"Hello Rust!\");");
    let expected_hello_python = highlighter.highlight("    print('Hello Python!')");

    assert!(stdout.contains(&expected_hello_world));
    assert!(stdout.contains(&expected_hello_rust));
    assert!(stdout.contains(&expected_hello_python));

    // Also check that file headers are shown
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("file2.rs"));
    assert!(stdout.contains("nested.py"));
}

#[test]
fn test_no_matches() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    let (stdout, stderr, exit_code) = run_xerg(&["NonexistentPattern", test_dir.to_str().unwrap()]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());
    // Should show file headers but no matches
    assert!(stdout.contains("---"));
    assert!(!stdout.contains("NonexistentPattern"));
}

#[test]
fn test_single_file_search() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);
    let file_path = test_dir.join("file1.txt");

    let (stdout, stderr, exit_code) = run_xerg(&["test", file_path.to_str().unwrap()]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());

    // Only "This is a test file" contains "test"
    let highlighter = TextHighlighter::new("test", &Color::Red);
    let expected_test_file = highlighter.highlight("This is a test file");

    assert!(stdout.contains(&expected_test_file));
    // Should NOT contain the "text" line since it doesn't match "test"
    assert!(!stdout.contains("sample text"));
}

#[test]
fn test_color_option() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    let (stdout, stderr, exit_code) =
        run_xerg(&["Hello", test_dir.to_str().unwrap(), "--color", "green"]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("Hello"));
    // Note: Testing ANSI color codes in integration tests is tricky
    // We're mainly testing that the option is accepted without error
}

#[test]
fn test_invalid_color_warning() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    let (stdout, stderr, exit_code) = run_xerg(&[
        "Hello",
        test_dir.to_str().unwrap(),
        "--color",
        "invalidcolor",
    ]);

    assert_eq!(exit_code, 0);
    assert!(stderr.contains("Warning: Invalid color name 'invalidcolor'"));

    // Should still highlight with default color (Red)
    let highlighter = TextHighlighter::new("Hello", &Color::Red);
    let expected_hello_world = highlighter.highlight("Hello world");
    assert!(stdout.contains(&expected_hello_world));
}

#[test]
fn test_nonexistent_directory() {
    let (stdout, stderr, exit_code) = run_xerg(&["pattern", "/nonexistent/directory"]);

    assert_eq!(exit_code, 1);
    assert!(stderr.contains("error: file or directory does not exist"));
    assert!(stdout.is_empty());
}

#[test]
fn test_help_option() {
    let (stdout, stderr, exit_code) = run_xerg(&["--help"]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("xerg"));
    assert!(stdout.contains("PATTERN"));
}

#[test]
fn test_version_option() {
    let (stdout, stderr, exit_code) = run_xerg(&["--version"]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());
    assert!(stdout.contains("xerg"));
    assert!(stdout.contains("0.1.1"));
}

#[test]
fn test_literal_patterns() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    // Test with a literal pattern that will match
    let (stdout, stderr, exit_code) = run_xerg(&["fn main", test_dir.to_str().unwrap()]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());

    // Use our highlighter to generate expected highlighted text
    let highlighter = TextHighlighter::new("fn main", &Color::Red);
    let expected_fn_main = highlighter.highlight("fn main() {");

    assert!(stdout.contains(&expected_fn_main));
}

#[test]
fn test_case_sensitivity() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    // Test lowercase search - should find no matches since we search for "hello" but files contain "Hello"
    let (stdout, stderr, exit_code) = run_xerg(&["hello", test_dir.to_str().unwrap()]);

    assert_eq!(exit_code, 0);
    assert!(stderr.is_empty());
    // Should not match "Hello" (case sensitive) - only file headers should be shown
    assert!(stdout.contains("---")); // File headers are shown
    assert!(!stdout.contains("Hello world")); // But no content matches
}

#[test]
fn test_missing_pattern_error() {
    let temp_dir = TempDir::new("integration_test").unwrap();
    let test_dir = create_test_files(&temp_dir);

    // Try to run with just a path (no pattern)
    let (stdout, stderr, exit_code) = run_xerg(&[test_dir.to_str().unwrap()]);

    assert_eq!(exit_code, 1);
    assert!(stderr.contains("Pattern missing"));
    assert!(stdout.is_empty());
}
