//! # Text Highlighting
//!
//! This module provides regex-based text highlighting functionality for pattern matching.
//! It applies ANSI color codes to highlight matched patterns in terminal output.
//!
//! ## Features
//!
//! - **Regex Pattern Matching**: Uses compiled regex for efficient pattern detection
//! - **ANSI Color Formatting**: Applies color codes around matched text
//! - **Performance Optimized**: Compiles regex once and reuses for multiple matches
//!
//! ## Example
//!
//! ```no_run
//! use xerg::highlighter::TextHighlighter;
//! use xerg::colors::Color;
//!
//! let highlighter = TextHighlighter::new("use", &Color::Blue);
//! let highlighted = highlighter.highlight("use std::path::Path;");
//! // Returns: "\x1b[34muse\x1b[0m std::path::Path;"
//! ```

use crate::colors::Color;
use regex::Regex;

pub struct TextHighlighter {
    pub regex: Regex,
    pub highlighted_pattern: String,
}

impl TextHighlighter {
    pub fn new(pattern: &str, color: &Color) -> Self {
        let regex = Regex::new(pattern).unwrap();
        let color_code = color.to_code();

        Self {
            regex,
            highlighted_pattern: format!("\x1b[{}m$0\x1b[0m", color_code),
        }
    }

    pub fn highlight(&self, text: &str) -> String {
        self.regex
            .replace_all(text, &self.highlighted_pattern)
            .to_string()
    }
}
