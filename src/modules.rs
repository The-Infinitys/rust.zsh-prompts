use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;
use std::str::FromStr;

pub mod cmd;
pub mod git;
pub mod os;
pub mod pwd;
pub mod time;

// 色の選択肢を定義するenum
#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, rkyv::Serialize, rkyv::Deserialize, rkyv::Archive,
)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    Rgb(u8, u8, u8),
}

// --- 手動シリアライズの実装 ---
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Displayトレイトの実装を利用して文字列としてシリアライズ
        serializer.serialize_str(&self.to_string())
    }
}

// --- 手動デシリアライズの実装 ---
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_str(&s).map_err(de::Error::custom)
    }
}

impl FromStr for Color {
    type Err = String;

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
                if s.starts_with('#') && (s.len() == 7 || s.len() == 4) {
                    if s.len() == 7 {
                        let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Invalid hex R")?;
                        let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Invalid hex G")?;
                        let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Invalid hex B")?;
                        Ok(Color::Rgb(r, g, b))
                    } else {
                        // #FFF 形式のサポート
                        let r = u8::from_str_radix(&s[1..2].repeat(2), 16)
                            .map_err(|_| "Invalid hex R")?;
                        let g = u8::from_str_radix(&s[2..3].repeat(2), 16)
                            .map_err(|_| "Invalid hex G")?;
                        let b = u8::from_str_radix(&s[3..4].repeat(2), 16)
                            .map_err(|_| "Invalid hex B")?;
                        Ok(Color::Rgb(r, g, b))
                    }
                } else {
                    Err(format!("Invalid color name or hex format: {}", s))
                }
            }
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        match self {
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::Black => "30".to_string(),
            Color::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }
}
#[derive(Debug, Clone, rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
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
        let color = Color::from_str(color_str).ok();
        Self { content, color }
    }

    pub fn format(&self) -> String {
        if let Some(color) = &self.color {
            format!("\x1b[{}m{}\x1b[39m", color.as_ansi_code(), self.content)
        } else {
            self.content.clone()
        }
    }
}
