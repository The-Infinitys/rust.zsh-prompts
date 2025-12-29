use serde::{Deserialize, Serialize};

pub mod cmd;
pub mod git;
pub mod os;
pub mod pwd;
pub mod time;

// 色の選択肢を定義するenum
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy)] // Hash, Cloneを追加
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    Rgb(u8, u8, u8), // 新しいRgbバリアント
}

impl std::str::FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "black" => Ok(Color::Black),
            _ => {
                // Try parsing as hex color #RRGGBB
                if s.starts_with('#') && s.len() == 7 {
                    let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Invalid hex color")?;
                    let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Invalid hex color")?;
                    let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Invalid hex color")?;
                    Ok(Color::Rgb(r, g, b))
                } else {
                    Err("Invalid color name or hex format")
                }
            }
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Yellow => write!(f, "yellow"),
            Color::Blue => write!(f, "blue"),
            Color::Magenta => write!(f, "magenta"),
            Color::Cyan => write!(f, "cyan"),
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
            Color::Rgb(r, g, b) => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

impl Color {
    pub fn as_ansi_code(&self) -> String {
        // Stringを返すように変更
        match self {
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::Black => "30".to_string(),
            Color::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b), // 24-bit color
        }
    }
}

// 新しい構造体を追加
pub struct PromptSegment {
    pub content: String,
    pub color: Option<Color>,
}

impl PromptSegment {
    pub fn new(content: String) -> Self {
        Self {
            content,
            color: None,
        }
    }

    pub fn new_with_color(content: String, color_str: &str) -> Self {
        // 色文字列をパース
        let color = color_str.parse::<Color>().ok(); // パースに失敗した場合はNone
        Self { content, color }
    }

    pub fn format(&self) -> String {
        if let Some(color) = &self.color {
            // \x1b[39m を使うことで、文字色だけをデフォルトに戻す
            format!("\x1b[{}m{}\x1b[39m", color.as_ansi_code(), self.content)
        } else {
            self.content.clone()
        }
    }
}
