pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
}

impl Color {
    pub fn to_code(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Blue => "34",
            Color::Bold => "1",
        }
    }

    pub fn from_str(color_str: &str) -> Option<Color> {
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
            let color = Color::from_str(color_str);
            assert!(color.is_some());
        }
    }

    #[test]
    fn test_from_str_invalid_color() {
        let color = Color::from_str("invalid");
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
            let color = Color::from_str(color_str);
            assert!(color.is_some());
        }
    }

    #[test]
    fn test_from_str_empty_string() {
        let color = Color::from_str("");
        assert!(color.is_none());
    }

    #[test]
    fn test_all_colors_have_codes() {
        let colors = vec![Color::Red, Color::Green, Color::Blue, Color::Bold];
        for _color in colors {
            let code = _color.to_code();
            assert!(matches!(!code.is_empty(), _color));
        }
    }
}
