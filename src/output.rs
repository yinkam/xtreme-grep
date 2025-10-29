use std::path::{Path, PathBuf};
use std::sync::mpsc;

pub type FileMatchResult = Vec<OutputMessage>;

pub enum OutputMessage {
    Header(PathBuf),
    Line { index: usize, content: String },
    Error(String),
    Done,
}

fn _print_line(index: usize, content: &str) {
    println!("  \x1b[1;38;5;245m{:>3}:\x1b[0m  {}", index + 1, content);
}

fn _print_header(filepath: &Path) {
    println!("\x1b[1;38;5;245m--- {}\x1b[0m ---", filepath.display());
}

pub fn print_output(rx: mpsc::Receiver<FileMatchResult>) {
    for message in rx {
        for msg in message {
            match msg {
                OutputMessage::Header(path) => {
                    _print_header(&path);
                }
                OutputMessage::Line { index, content } => {
                    _print_line(index, &content);
                }
                OutputMessage::Error(err) => {
                    eprintln!("Error: {}", err);
                }
                OutputMessage::Done => break,
            }
        }
    }
}
