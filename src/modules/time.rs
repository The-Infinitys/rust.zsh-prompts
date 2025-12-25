use chrono::Local;
use crate::modules::PromptSegment;

pub fn get_time() -> PromptSegment {
    let now = Local::now();
    PromptSegment::new(format!(" {}", now.format("%H:%M:%S")))
}
