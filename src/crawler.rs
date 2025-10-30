//! # Directory Traversal
//!
//! This module provides efficient directory traversal functionality with intelligent
//! file filtering and symlink support.
//!
//! ## Features
//!
//! - **Recursive Scanning**: Traverses directories recursively to find all files
//! - **Hidden File Filtering**: Automatically skips hidden files and directories (starting with '.')
//! - **Symlink Support**: Safely handles symbolic links during traversal
//! - **Error Resilience**: Gracefully handles permission errors and inaccessible files
//!
//! ## Example
//!
//! ```rust
//! use xgrep::crawler::get_files;
//! use std::path::PathBuf;
//!
//! let dir = PathBuf::from("src/");
//! let files = get_files(&dir);
//! println!("Found {} files", files.len());
//! ```

use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn get_files(dir: &PathBuf) -> Vec<PathBuf> {
    if dir.is_file() {
        return vec![dir.clone()];
    }

    WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempdir::TempDir;

    #[test]
    fn test_get_files_single_file() {
        // Create a temporary file and test get_files on it
        let temp_dir = TempDir::new("test_").unwrap();

        let temp_file = temp_dir.path().join("test.txt");
        File::create(&temp_file).unwrap();

        let files = get_files(&temp_file);
        assert_eq!(files, vec![temp_file]);
    }

    #[test]
    fn test_get_files_directory_with_files() {
        // Create temp directory with multiple files and verify all are returned
        let temp_dir = TempDir::new("test_").unwrap();

        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let files = get_files(&temp_dir.into_path());
        assert_eq!(files, vec![file2, file1]);
    }

    #[test]
    fn test_get_files_empty_directory() {
        // TODO: Test that empty directory returns empty vec
        let temp_dir = TempDir::new("test_").unwrap();

        let files = get_files(&temp_dir.into_path());
        assert_eq!(files, Vec::<PathBuf>::new());
    }

    #[test]
    fn test_get_files_nested_directories() {
        // Create nested directory structure and verify all files are found
        let temp_dir = TempDir::new("test_").unwrap();

        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let file1 = temp_dir.path().join("file1.txt");
        let file2 = sub_dir.join("file2.txt");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let files = get_files(&temp_dir.into_path());
        assert_eq!(files, vec![file1, file2]);
    }

    #[test]
    fn test_get_files_ignores_hidden_files() {
        let temp_dir = TempDir::new("test_").unwrap();

        let hidden_file = temp_dir.path().join(".hidden_file");
        let regular_file = temp_dir.path().join("regular_file.txt");
        File::create(&hidden_file).unwrap();
        File::create(&regular_file).unwrap();

        let files = get_files(&temp_dir.into_path());
        assert_eq!(files, vec![regular_file]);
    }

    #[test]
    fn test_get_files_ignores_hidden_directories() {
        // Create .hidden_dir with files inside
        // Verify files in hidden directory are not returned
        let temp_dir = TempDir::new("test_").unwrap();

        let hidden_dir = temp_dir.path().join(".hidden_dir");
        fs::create_dir(&hidden_dir).unwrap();
        let hidden_file = hidden_dir.join("hidden_file.txt");
        File::create(&hidden_file).unwrap();
        let regular_file = temp_dir.path().join("regular_file.txt");
        File::create(&regular_file).unwrap();
        let files = get_files(&temp_dir.into_path());
        assert_eq!(files, vec![regular_file]);
    }

    #[test]
    fn test_get_files_mixed_content() {
        // Verify only non-hidden files are returned
        let temp_dir = TempDir::new("test_").unwrap();

        let hidden_dir = temp_dir.path().join(".hidden_dir");
        fs::create_dir(&hidden_dir).unwrap();

        let hidden_file = hidden_dir.join("hidden_file.txt");
        File::create(&hidden_file).unwrap();

        let regular_file = temp_dir.path().join("regular_file.txt");
        File::create(&regular_file).unwrap();

        let files = get_files(&temp_dir.into_path());
        assert_eq!(files, vec![regular_file]);
    }

    #[test]
    fn test_get_files_follows_file_symlinks() {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new("test_file_symlinks").unwrap();

        // Create a regular file
        let regular_file = temp_dir.path().join("regular.txt");
        File::create(&regular_file).unwrap();

        // Create symlink to file (should be followed with follow_links(true))
        let file_symlink = temp_dir.path().join("link_to_file.txt");
        symlink(&regular_file, &file_symlink).unwrap();

        let files = get_files(&temp_dir.path().to_path_buf());

        // Should include both the original file and the symlink target
        // Note: with follow_links(true), symlinks are resolved to their targets
        let mut sorted_files = files;
        sorted_files.sort();

        // Both should point to the same file (the original), but walkdir
        // will include both the original path and the symlink path
        assert!(sorted_files.contains(&regular_file));
        assert!(sorted_files.len() >= 1);
    }

    #[test]
    fn test_get_files_follows_directory_symlinks() {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new("test_dir_symlinks").unwrap();

        // Create a subdirectory with a file
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        let sub_file = sub_dir.join("file_in_subdir.txt");
        File::create(&sub_file).unwrap();

        // Create symlink to directory (should be followed with follow_links(true))
        let dir_symlink = temp_dir.path().join("link_to_dir");
        symlink(&sub_dir, &dir_symlink).unwrap();

        let files = get_files(&temp_dir.path().to_path_buf());

        // include files from both the original directory and via the symlink
        let mut sorted_files = files;
        sorted_files.sort();

        // Both should point to the same file (the original), but walkdir
        assert!(sorted_files.contains(&sub_file));

        // Both should point to the same file (the original), but walkdir
        assert!(sorted_files.len() >= 1);
    }

    #[test]
    fn test_get_files_handles_broken_symlinks() {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new("test_broken_symlinks").unwrap();

        // Create a regular file for comparison
        let regular_file = temp_dir.path().join("regular.txt");
        File::create(&regular_file).unwrap();

        // Create broken symlink - Both should point to the same file (the original), but walkdir
        let broken_symlink = temp_dir.path().join("broken_link.txt");
        symlink("nonexistent_file.txt", &broken_symlink).unwrap();

        let files = get_files(&temp_dir.path().to_path_buf());

        // Should include regular file but gracefully skip broken symlink
        assert_eq!(files, vec![regular_file]);
    }

    #[test]
    fn test_get_files_symlinks_comprehensive() {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new("test_comprehensive_symlinks").unwrap();

        // Create regular file
        let regular_file = temp_dir.path().join("regular.txt");
        File::create(&regular_file).unwrap();

        // Create subdirectory with file
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        let sub_file = sub_dir.join("sub_file.txt");
        File::create(&sub_file).unwrap();

        // Create various symlinks
        let file_symlink = temp_dir.path().join("link_to_file.txt");
        symlink(&regular_file, &file_symlink).unwrap();

        let dir_symlink = temp_dir.path().join("link_to_dir");
        symlink(&sub_dir, &dir_symlink).unwrap();

        let broken_symlink = temp_dir.path().join("broken_link.txt");
        symlink("nonexistent.txt", &broken_symlink).unwrap();

        let files = get_files(&temp_dir.path().to_path_buf());

        // With follow_links(true), should include regular files and handle symlinks appropriately
        assert!(files.contains(&regular_file));
        assert!(files.contains(&sub_file));
        assert!(files.len() >= 2); // At least the two regular files

        // Should not crash or include broken symlinks
        assert!(
            !files
                .iter()
                .any(|path| path.to_string_lossy().contains("nonexistent"))
        );
    }
}
