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
