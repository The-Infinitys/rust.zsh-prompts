use crate::modules::{Color, PromptSegment};
use os_info::Type;

pub fn get_os_icon(color: Option<Color>) -> PromptSegment {
    let info = os_info::get();
    let icon = match info.os_type() {
        Type::Macos => "".to_string(),                 // Apple icon
        Type::Ubuntu => "".to_string(),                // Ubuntu
        Type::Fedora => "".to_string(),                // Fedora
        Type::CentOS | Type::Redhat => "".to_string(), // CentOS/RHEL
        Type::Arch => "󰣇".to_string(),                  // Arch
        Type::Debian => "".to_string(),                // Debian
        Type::Windows => "".to_string(),               // Windows
        Type::Linux => "".to_string(), // WSL (Linux kernel, but Windows Subsystem) or Generic Linux
        _ => "󰀵".to_string(),           // Default icon
    };
    PromptSegment::new_with_color(icon, &color.unwrap_or(Color::White).to_string())
}
