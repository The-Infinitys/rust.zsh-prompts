use crate::modules::{Color, PromptSegment};
use chrono::Local;

pub fn get_time(color: Option<Color>) -> PromptSegment {
    let now = Local::now();
    PromptSegment::new_with_color(
        format!(" {}", now.format("%H:%M:%S")),
        &color.unwrap_or(Color::Magenta).to_string(),
    )
}
