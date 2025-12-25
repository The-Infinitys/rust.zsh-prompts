use chrono::Utc;

pub fn get_execution_info(last_status: i32, last_command_executed: Option<f64>) -> String {
    let status_icon: &str;

    if last_status == 0 {
        status_icon = ""; // Success
    } else {
        status_icon = ""; // Failure
    }

    let mut duration_str = String::new();
    if let Some(timer_start_f64) = last_command_executed {
        // We need to use a consistent "now" for testing purposes.
        // For actual execution, `Utc::now()` is appropriate.
        // For tests, we might mock this or adjust `timer_start_f64` relative to a fixed `now`.
        let timer_now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let delta_f64 = timer_now_f64 - timer_start_f64;

        if delta_f64 >= 0.5 { // Only display if 0.5s or more
            let total_seconds = delta_f64.trunc() as i64;
            let days = total_seconds / 86400;
            let hours = (total_seconds % 86400) / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;

            if days > 0 {
                duration_str.push_str(&format!("{}d", days));
            }
            if hours > 0 {
                duration_str.push_str(&format!("{}h", hours));
            }
            if minutes > 0 {
                duration_str.push_str(&format!("{}m", minutes));
            }
            
            // If less than 1 second, show seconds with two decimal places (rounded)
            if delta_f64 < 1.0 {
                duration_str.push_str(&format!("{:.2}s", delta_f64));
            } else {
                duration_str.push_str(&format!("{}s", seconds));
            }
        }
    }

    let mut info: String;

    if !duration_str.is_empty() {
        info = format!("{} {}", status_icon, duration_str);
    } else {
        info = status_icon.to_string();
    }

    if last_status != 0 {
        info = format!("{} {}", info, last_status);
    }

    info
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_get_execution_info_success_no_time() {
        let last_status = 0;
        let last_command_executed = None;
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_execution_info_success_with_time() {
        let last_status = 0;
        // Simulate a command that started 0.5 seconds ago
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let last_command_executed = Some(now_f64 - 0.5); // 0.5 seconds ago
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, " 0.50s");
    }

    #[test]
    fn test_get_execution_info_failure_no_time() {
        let last_status = 1;
        let last_command_executed = None;
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, " 1");
    }

    #[test]
    fn test_get_execution_info_failure_with_time() {
        let last_status = 127;
        // Simulate a command that started 1.2 seconds ago
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let last_command_executed = Some(now_f64 - 1.2); // 1.2 seconds ago
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, " 1s 127");
    }

    #[test]
    fn test_get_execution_info_long_duration_minutes() {
        let last_status = 0;
        // Simulate a command that started 2 minutes and 30 seconds ago (150 seconds)
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let last_command_executed = Some(now_f64 - 150.5); // 2m 30.5s ago
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, " 2m30s");
    }

    #[test]
    fn test_get_execution_info_five_seconds_duration() {
        let last_status = 0;
        // Simulate a command that started 5 seconds ago
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let last_command_executed = Some(now_f64 - 5.000); // 5 seconds ago
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, " 5s");
    }

    #[test]
    fn test_get_execution_info_long_duration_days() {
        let last_status = 0;
        // Simulate a command that started 1 day, 2 hours, 3 minutes, 4 seconds ago
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let last_command_executed = Some(now_f64 - (86400.0 + 7200.0 + 180.0 + 4.0 + 0.9)); // 1d 2h 3m 4.9s ago
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, " 1d2h3m4s");
    }

    #[test]
    fn test_get_execution_info_less_than_0_1_sec() {
        let last_status = 0;
        // Simulate a command that started 0.05 seconds ago (less than 0.1s threshold)
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let last_command_executed = Some(now_f64 - 0.05);
        let result = get_execution_info(last_status, last_command_executed);
        assert_eq!(result, ""); // Should not display time
    }
}
