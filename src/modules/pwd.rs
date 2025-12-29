use crate::modules::{Color, PromptSegment};
use std::env;
use std::fs;

pub fn get_smart_pwd(color: Option<Color>) -> Vec<PromptSegment> {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            return vec![
                PromptSegment::new_with_color(" ".to_string(), &Color::Red.to_string()),
                PromptSegment::new_with_color("Error".to_string(), &Color::Red.to_string()),
            ];
        }
    };

    let home_dir = dirs::home_dir();

    let mut display_path = current_dir.to_string_lossy().to_string();

    if let Some(home) = home_dir {
        if current_dir == home {
            display_path = "~".to_string();
        } else if current_dir.starts_with(&home) {
            display_path = display_path.replacen(&home.to_string_lossy().to_string(), "~", 1);
        }
    }

    let mut icon = ""; // Default folder icon

    // Check write permissions for the current directory
    if let Ok(metadata) = fs::metadata(&current_dir) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt; // Corrected: PermissionsExt trait
            let permissions = metadata.permissions().mode(); // Corrected: use metadata.permissions().mode()
            // Check if owner, group, or others have write permission
            if permissions & 0o222 == 0 {
                icon = ""; // Lock icon if not writable
            }
        }
        #[cfg(windows)]
        {
            // Windows permissionsはより複雑です; 現時点では簡略化または省略します。
            // 堅牢なソリューションのためには、winapiを使用してACLをチェックする必要があります。
            // 現時点では、明示的に処理しない限りWindowsでは書き込み可能と仮定します。
        }
    }

    if display_path == "~" {
        icon = ""; // Home icon
    }

    // Basic path truncation: if too long, show start...end
    let max_len = 100;
    if display_path.len() > max_len {
        let path_parts: Vec<&str> = display_path.split('/').collect();
        if path_parts.len() > 1 {
            let first_part = path_parts[0];
            let last_part = path_parts.last().unwrap_or(&""); // Safe unwrap after check
            let truncated_len = max_len - first_part.len() - last_part.len() - 3; // 3 for "..."
            if truncated_len > 0 {
                display_path = format!("{}/.../{}", first_part, last_part);
            } else {
                // If not enough space for start/.../end, just truncate from end
                display_path = format!("...{}", &display_path[display_path.len() - max_len + 3..]);
            }
        } else {
            display_path = format!("...{}", &display_path[display_path.len() - max_len + 3..]);
        }
    }

    vec![
        PromptSegment::new_with_color(icon.to_string(), &color.unwrap_or(Color::Cyan).to_string()),
        PromptSegment::new_with_color(
            display_path.to_string(),
            &color.unwrap_or(Color::Cyan).to_string(),
        ),
    ]
}
