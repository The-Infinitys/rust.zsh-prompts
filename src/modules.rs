pub mod os;
pub mod pwd;
pub mod time;
pub mod git;
pub mod cmd;

// 色の選択肢を定義するenum
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    // 必要に応じて他の色を追加
}

impl Color {
    pub fn as_ansi_code(&self) -> &'static str {
        match self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::Black => "30",
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
        Self { content, color: None }
    }

    pub fn new_with_color(content: String, color: Color) -> Self {
        Self { content, color: Some(color) }
    }

    // 色を適用した文字列を生成するヘルパーメソッド
    pub fn format(&self) -> String {
        if let Some(color) = &self.color {
            format!("\x1b[{}m{}\x1b[0m", color.as_ansi_code(), self.content)
        } else {
            self.content.clone()
        }
    }
}
