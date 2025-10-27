pub mod colors;
pub mod crawler;
pub mod highlighter;
pub mod search;

use colors::Color;
use crawler::get_files;
use search::search_files;
use std::path::PathBuf;

pub fn run(dir: &PathBuf, pattern: &str, color: &Color) {
    let files = get_files(&dir);
    search_files(&files, &pattern, &color);
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
