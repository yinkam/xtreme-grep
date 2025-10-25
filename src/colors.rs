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
