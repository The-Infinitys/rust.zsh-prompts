use crate::modules::{Color, PromptSegment};
use chrono::Utc;
use std::env;

pub fn get_execution_info(
    last_status_var_name: &String,
    last_command_executed_var_name: &Option<String>,
    color: Option<Color>,
) -> PromptSegment {
    // 1. 環境変数名からステータスを取得
    let last_status: i32 = env::var(last_status_var_name)
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(0);
    let last_command_executed: Option<f64> = last_command_executed_var_name
        .as_ref()
        .and_then(|name| env::var(name).ok().and_then(|val| val.parse().ok()));
    let status_icon: &str;
    let segment_color: Color;

    if last_status == 0 {
        status_icon = "";
        segment_color = Color::Green;
    } else {
        status_icon = "";
        segment_color = Color::Red;
    }

    let mut duration_str = String::new();
    if let Some(timer_start_f64) = last_command_executed {
        let timer_now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        let delta_f64 = timer_now_f64 - timer_start_f64;

        if delta_f64 >= 0.5 {
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

            if delta_f64 < 1.0 {
                duration_str.push_str(&format!("{:.2}s", delta_f64));
            } else {
                duration_str.push_str(&format!("{}s", seconds));
            }
        }
    }

    let mut info = if !duration_str.is_empty() {
        format!("{} {}", status_icon, duration_str)
    } else {
        status_icon.to_string()
    };

    if last_status != 0 {
        info = format!("{} {}", info, last_status);
    }

    PromptSegment::new_with_color(info, &color.unwrap_or(segment_color).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::Color;
    use chrono::Utc;
    use std::env;

    fn set_test_env(key: &str, value: &str) {
        unsafe {
            env::set_var(key, value);
        }
    }

    #[test]
    fn test_get_execution_info_success_no_time() {
        let status_var = "TEST_STATUS_SUCCESS".to_string();
        set_test_env(&status_var, "0");

        // 引数を参照 (&) で渡すように修正
        let result = get_execution_info(&status_var, &None, None);
        assert_eq!(result.content, "");
        assert_eq!(result.color, Some(Color::Green));
    }

    #[test]
    fn test_get_execution_info_failure_with_time() {
        let status_var = "TEST_STATUS_FAIL".to_string();
        let time_var = "TEST_TIME_FAIL".to_string();

        set_test_env(&status_var, "127");
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        set_test_env(&time_var, (now_f64 - 1.2).to_string().as_str());

        // 引数を参照 (&) で渡すように修正
        let result = get_execution_info(&status_var, &Some(time_var), None);

        assert!(result.content.contains(""));
        assert!(result.content.contains("127"));
        assert!(result.content.contains("1s"));
        assert_eq!(result.color, Some(Color::Red));
    }

    #[test]
    fn test_get_execution_info_long_duration_minutes() {
        let status_var = "TEST_STATUS_LONG".to_string();
        let time_var = "TEST_TIME_LONG".to_string();

        set_test_env(&status_var, "0");
        let now_f64 = Utc::now().timestamp_nanos_opt().unwrap() as f64 / 1_000_000_000.0;
        set_test_env(&time_var, (now_f64 - 150.5).to_string().as_str());

        let result = get_execution_info(&status_var, &Some(time_var), None);
        assert!(result.content.contains("2m30s"));
    }

    #[test]
    fn test_get_execution_info_invalid_env_fallback() {
        let status_var = "NON_EXISTENT_VAR".to_string();
        let time_var = Some("INVALID_VAL_VAR".to_string());

        let result = get_execution_info(&status_var, &time_var, None);

        assert_eq!(result.content, "");
        assert_eq!(result.color, Some(Color::Green));
    }

    #[test]
    fn test_get_execution_info_with_custom_color() {
        let status_var = "TEST_STATUS_COLOR".to_string();
        set_test_env(&status_var, "0");

        let custom_color = Some(Color::Blue);
        let result = get_execution_info(&status_var, &None, custom_color);

        assert_eq!(result.content, "");
        assert_eq!(result.color, Some(Color::Blue));
    }
}
