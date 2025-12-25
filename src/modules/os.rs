use os_info::Type;

pub fn get_os_icon() -> String {
    let info = os_info::get(); // Corrected: use os_info::get()
    match info.os_type() {
        Type::Macos => "".to_string(), // Apple icon
        Type::Ubuntu => "".to_string(), // Ubuntu
        Type::Fedora => "".to_string(), // Fedora
        Type::CentOS | Type::Redhat => "".to_string(), // CentOS/RHEL (Corrected: Type::CentOS)
        Type::Arch => "󰣇".to_string(), // Arch
        Type::Debian => "".to_string(), // Debian
        Type::Windows => "".to_string(), // Windows
        Type::Linux => {
            // Check for WSL within Linux, similar to os.sh
            if is_wsl() {
                "".to_string() // WSL (Linux kernel, but Windows Subsystem)
            } else {
                "".to_string() // Generic Linux
            }
        }
        _ => "󰀵".to_string(), // Default icon
    }
}

fn is_wsl() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|s| s.to_lowercase().contains("microsoft") || s.to_lowercase().contains("wsl"))
        .unwrap_or(false)
}