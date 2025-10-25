use crate::colors::Color;
use regex::Regex;

pub struct TextColorizer {
    pub regex: Regex,
    pub colorized_pattern: String,
}

impl TextColorizer {
    pub fn new(pattern: &str, color: &Color) -> Self {
        let regex = Regex::new(pattern).unwrap();
        let color_code = color.to_code();

        Self {
            regex,
            colorized_pattern: format!("\x1b[{}m$0\x1b[0m", color_code),
        }
    }

    pub fn colorize(&self, text: &str) -> String {
        self.regex
            .replace_all(text, &self.colorized_pattern)
            .to_string()
    }
}
