use std::process::Command;
use regex::Regex;

pub fn get_git_status() -> String {
    // 1. Check if inside Git repository
    let is_git_repo = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !is_git_repo {
        return "".to_string(); // Not a git repository, return empty
    }

    let mut output_parts = Vec::new();

    // Remote icon
    let remote_url_output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_default();

    let remote_icon = if remote_url_output.contains("github.com") {
        ""
    } else if remote_url_output.contains("gitlab.com") {
        ""
    } else {
        "󰊢"
    };
    output_parts.push(remote_icon.to_string());

    // Branch name
    let branch_output = Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()
        .or_else(|_| {
            // Fallback to short hash if not on a branch
            Command::new("git").arg("rev-parse").arg("--short").arg("HEAD").output()
        })
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    
    output_parts.push("".to_string()); // Git icon
    output_parts.push(branch_output);

    // Git status --porcelain=v2 --branch
    let status_output = Command::new("git")
        .arg("status")
        .arg("--porcelain=v2")
        .arg("--branch")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_default();

    let mut ahead = 0;
    let mut behind = 0;
    let mut staged_changes = 0;
    let mut unstaged_changes = 0;
    let mut untracked_files = 0;
    let mut conflicts = 0;

    for line in status_output.lines() {
        if line.starts_with("# branch.ab") {
            let re = Regex::new(r"\+([0-9]+) -([0-9]+)").unwrap();
            if let Some(captures) = re.captures(line) {
                if let (Some(a), Some(b)) = (captures.get(1), captures.get(2)) {
                    ahead = a.as_str().parse().unwrap_or(0);
                    behind = b.as_str().parse().unwrap_or(0);
                }
            }
        } else if line.starts_with("1") || line.starts_with("2") {
            // Normal, Renamed, Copied
            let x = line.chars().nth(2).unwrap_or('.'); // Staged
            let y = line.chars().nth(3).unwrap_or('.'); // Unstaged

            if x != '.' { staged_changes += 1; }
            if y != '.' { unstaged_changes += 1; }
        } else if line.starts_with("u") {
            // Unmerged (conflict)
            conflicts += 1;
        } else if line.starts_with("?") {
            // Untracked
            untracked_files += 1;
        }
    }

    // Stash status
    let stashed = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg("refs/stash")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    // Assemble status icons
    let mut status_icons_str = String::new();
    if staged_changes > 0 { status_icons_str.push_str(&format!("+{} ", staged_changes)); }
    if unstaged_changes > 0 { status_icons_str.push_str(&format!("!{} ", unstaged_changes)); }
    if untracked_files > 0 { status_icons_str.push_str(&format!("?{} ", untracked_files)); }
    if conflicts > 0 { status_icons_str.push_str(&format!("{} ", conflicts)); }
    if stashed { status_icons_str.push_str(" "); }

    if status_icons_str.is_empty() {
        output_parts.push("".to_string()); // Clean icon
    } else {
        output_parts.push(status_icons_str.trim().to_string());
    }

    // Assemble push/pull status
    if ahead > 0 { output_parts.push(format!("↑{}", ahead)); }
    if behind > 0 { output_parts.push(format!("↓{}", behind)); }

    output_parts.join(" ")
}
