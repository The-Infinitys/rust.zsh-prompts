pub mod os;
pub mod pwd;
pub mod time;
pub mod git;
pub mod cmd;

// 新しい構造体を追加
pub struct PromptSegment {
    pub content: String,
    pub color: Option<String>, // 例: "red", "green", "blue" などの文字列
}

impl PromptSegment {
    pub fn new(content: String) -> Self {
        Self { content, color: None }
    }

    pub fn new_with_color(content: String, color: String) -> Self {
        Self { content, color: Some(color) }
    }

    // 色を適用した文字列を生成するヘルパーメソッド
    pub fn format(&self) -> String {
        if let Some(color) = &self.color {
            // ここでANSIエスケープコードを使用して色を適用
            // 例: \x1b[31m (赤色)
            match color.as_str() {
                "red" => format!("\x1b[31m{}\x1b[0m", self.content),
                "green" => format!("\x1b[32m{}\x1b[0m", self.content),
                "yellow" => format!("\x1b[33m{}\x1b[0m", self.content),
                "blue" => format!("\x1b[34m{}\x1b[0m", self.content),
                "magenta" => format!("\x1b[35m{}\x1b[0m", self.content),
                "cyan" => format!("\x1b[36m{}\x1b[0m", self.content),
                "white" => format!("\x1b[37m{}\x1b[0m", self.content),
                // 他の色やデフォルトの色なしの場合
                _ => self.content.clone(),
            }
        } else {
            self.content.clone()
        }
    }
}
