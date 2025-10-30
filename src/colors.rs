//! # Color Management
//!
//! This module provides ANSI color code management for terminal text highlighting.
//! It supports customizable syntax highlighting with different color options.
//!
//! ## Supported Colors
//!
//! - **Red**: Standard red text highlighting
//! - **Green**: Standard green text highlighting  
//! - **Blue**: Standard blue text highlighting
//! - **Bold**: Bold text formatting
//!
//! ## Example
//!
//! ```no_run
//! use xgrep::colors::Color;
//!
//! let red = Color::Red;
//! let code = red.to_code(); // Returns "31"
//! ```

/// Represents available color options for text highlighting

#[derive(Debug, PartialEq)]
pub enum Color {
    /// Red text color (ANSI code 31)
    Red,
    /// Green text color (ANSI code 32)
    Green,
    /// Blue text color (ANSI code 34)
    Blue,
    /// Bold text formatting (ANSI code 1)
    Bold,
}

impl Color {
    /// Returns the ANSI escape code for this color
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xgrep::colors::Color;
    ///
    /// let code = Color::Red.to_code();    // Returns "31"
    /// let code = Color::Blue.to_code();   // Returns "34"
    /// let code = Color::Bold.to_code();   // Returns "1"
    /// ```
    pub fn to_code(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Blue => "34",
            Color::Bold => "1",
        }
    }

    /// Parses a color from a string representation
    ///
    /// Returns `Some(Color)` if the string matches a valid color name (case-insensitive),
    /// or `None` if the string doesn't match any known color.
    ///
    /// # Supported Values
    ///
    /// - `"red"` → `Color::Red`
    /// - `"green"` → `Color::Green`
    /// - `"blue"` → `Color::Blue`
    /// - `"bold"` → `Color::Bold`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xgrep::colors::Color;
    ///
    /// let color = Color::from_string("red");     // Returns Some(Color::Red)
    /// let color = Color::from_string("BLUE");    // Returns Some(Color::Blue)
    /// let color = Color::from_string("invalid"); // Returns None
    /// ```
    pub fn from_string(color_str: &str) -> Option<Color> {
        match color_str.to_lowercase().as_str() {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            "bold" => Some(Color::Bold),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_code_red() {
        let color = Color::Red;
        assert_eq!(color.to_code(), "31");
    }

    #[test]
    fn test_color_to_code_green() {
        let color: Color = Color::Green;
        assert_eq!(color.to_code(), "32");
    }

    #[test]
    fn test_color_to_code_blue() {
        let color: Color = Color::Blue;
        assert_eq!(color.to_code(), "34");
    }

    #[test]
    fn test_color_to_code_bold() {
        let color: Color = Color::Bold;
        assert_eq!(color.to_code(), "1");
    }

    #[test]
    fn test_from_str_valid_colors() {
        let colors_list = vec![
            ("red", Color::Red),
            ("green", Color::Green),
            ("blue", Color::Blue),
            ("bold", Color::Bold),
        ];
        for (color_str, _expected_color) in colors_list {
            let color = Color::from_string(color_str);
            assert!(color.is_some());
        }
    }

    #[test]
    fn test_from_str_invalid_color() {
        let color = Color::from_string("invalid");
        assert!(color.is_none());
    }

    #[test]
    fn test_from_str_case_insensitive() {
        let color_cases = vec![
            ("RED", Color::Red),
            ("Red", Color::Red),
            ("rEd", Color::Red),
        ];
        for (color_str, _expected_color) in color_cases {
            let color = Color::from_string(color_str);
            assert!(color.is_some());
        }
    }

    #[test]
    fn test_from_str_empty_string() {
        let color = Color::from_string("");
        assert!(color.is_none());
    }

    #[test]
    fn test_all_colors_have_codes() {
        let colors = vec![Color::Red, Color::Green, Color::Blue, Color::Bold];
        for color in colors {
            let code = color.to_code();
            assert!(!code.is_empty());
        }
    }
}
