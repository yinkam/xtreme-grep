pub mod colorizer;
pub mod colors;
pub mod crawler;
pub mod search;

use colors::Color;
use crawler::get_directory_files;
use search::{search_directory, search_file};
use std::path::PathBuf;

pub fn run(dir: &PathBuf, pattern: &str, color: &Color) {
    if dir.is_file() {
        search_file(&dir, &pattern, &color);
        return;
    }

    let files = get_directory_files(&dir);
    search_directory(&files, &pattern, &color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
