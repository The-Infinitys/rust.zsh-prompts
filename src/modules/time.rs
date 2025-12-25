use chrono::Local;

pub fn get_time() -> String {
    let now = Local::now();
    format!(" {}", now.format("%H:%M:%S"))
}
