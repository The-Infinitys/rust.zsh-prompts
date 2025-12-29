use crate::modules::{Color, PromptSegment};
use os_info::Type;

pub fn get_os_icon(color: Option<Color>) -> PromptSegment {
    let info = os_info::get();

    // 共通アイコンの定義
    let windows = "";
    let linux = "";
    let arch = "󰣇";
    let free_bsd = "";
    let gentoo = "";
    let apple = "";
    let azure = "";
    let cloud = "";
    let redhat = "";
    let fedora = "";
    let suse = "";
    let infinity = "";
    let icon = match info.os_type() {
        Type::AIX => linux,
        Type::AlmaLinux => "",
        Type::Alpaquita => linux,
        Type::Alpine => "",
        Type::ALTLinux => linux,
        Type::Amazon => "",
        Type::Android => "",
        Type::AOSC => "",
        Type::Arch => arch,
        Type::Artix => "",
        Type::Bluefin => linux,
        Type::CachyOS => arch,
        Type::CentOS => redhat,
        Type::Cygwin => windows,
        Type::Debian => "",
        Type::DragonFly => free_bsd,
        Type::Elementary => "",
        Type::Emscripten => "🗲",
        Type::EndeavourOS => "",
        Type::Fedora => fedora,
        Type::FreeBSD => free_bsd,
        Type::Garuda => "",
        Type::Gentoo => gentoo,
        Type::HardenedBSD => free_bsd,
        Type::Illumos => "",
        Type::InstantOS => arch,
        Type::Ios => apple,
        Type::Kali => "",
        Type::Linux => linux,
        Type::Mabox => arch,
        Type::Macos => apple,
        Type::Manjaro => "",
        Type::Mariner => azure,
        Type::MidnightBSD => free_bsd,
        Type::Mint => "󰣭",
        Type::NetBSD => free_bsd,
        Type::NixOS => "",
        Type::Nobara => "",
        Type::OpenBSD => "",
        Type::OpenCloudOS => cloud,
        Type::openEuler => linux,
        Type::openSUSE => suse,
        Type::OracleLinux => "",
        Type::PikaOS => "󱗆",
        Type::Pop => "",
        Type::Raspbian => "",
        Type::Redhat => redhat,
        Type::RedHatEnterprise => redhat,
        Type::Redox => "Ⓡ",
        Type::RockyLinux => "",
        Type::Solus => "",
        Type::SUSE => suse,
        Type::Ubuntu => "",
        Type::Ultramarine => fedora,
        Type::Unknown => infinity,
        Type::Uos => linux,
        Type::Void => "",
        Type::Windows => windows,
        Type::Zorin => "",
        _ => infinity,
    };

    PromptSegment::new_with_color(icon.to_string(), &color.unwrap_or(Color::White).to_string())
}
