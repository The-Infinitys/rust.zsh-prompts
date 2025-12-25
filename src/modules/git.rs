use crate::modules::{Color, PromptSegment};
use regex::Regex;
use std::process::Command;

pub fn get_git_status(
    default_color_option: Option<Color>,
    git_icon_color_option: Option<Color>,
    branch_color_option: Option<Color>,
    staged_color_option: Option<Color>,
    unstaged_color_option: Option<Color>,
    untracked_color_option: Option<Color>,
    conflict_color_option: Option<Color>,
    stashed_color_option: Option<Color>,
    clean_color_option: Option<Color>,
    ahead_color_option: Option<Color>,
    behind_color_option: Option<Color>,
) -> Vec<PromptSegment> {
    let mut segments: Vec<PromptSegment> = Vec::new();

    // Helper to get color, preferring user-provided, then fall back to specific, then White
    let get_color = |specific_color: Color, override_color: Option<Color>| {
        override_color
            .or(default_color_option.clone())
            .unwrap_or(specific_color)
            .to_string()
    };

    // 1. Check if inside Git repository
    let is_git_repo = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !is_git_repo {
        return segments; // Not a git repository, return empty vector
    }

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
        "" // GitHub icon (blue)
    } else if remote_url_output.contains("gitlab.com") {
        "" // GitLab icon (orange)
    } else {
        "󰊢" // Generic remote icon (white)
    };
    segments.push(PromptSegment::new_with_color(
        remote_icon.to_string(),
        &get_color(Color::Blue, git_icon_color_option.clone()),
    ));

    // Branch name
    let branch_output = Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()
        .or_else(|_| {
            // Fallback to short hash if not on a branch
            Command::new("git")
                .arg("rev-parse")
                .arg("--short")
                .arg("HEAD")
                .output()
        })
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    segments.push(PromptSegment::new_with_color(
        "".to_string(),
        &get_color(Color::White, git_icon_color_option.clone()),
    )); // Git icon (white)
    segments.push(PromptSegment::new_with_color(
        branch_output,
        &get_color(Color::Yellow, branch_color_option.clone()),
    )); // Branch name (yellow)

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
            if let Some(captures) = re.captures(line)
                && let (Some(a), Some(b)) = (captures.get(1), captures.get(2)) {
                    ahead = a.as_str().parse().unwrap_or(0);
                    behind = b.as_str().parse().unwrap_or(0);
                }
        } else if line.starts_with("1") || line.starts_with("2") {
            // Normal, Renamed, Copied
            let x = line.chars().nth(2).unwrap_or('.'); // Staged
            let y = line.chars().nth(3).unwrap_or('.'); // Unstaged

            if x != '.' {
                staged_changes += 1;
            }
            if y != '.' {
                unstaged_changes += 1;
            }
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
    if staged_changes > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("+{}", staged_changes),
            &get_color(Color::Green, staged_color_option.clone()),
        ));
    }
    if unstaged_changes > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("!{}", unstaged_changes),
            &get_color(Color::Red, unstaged_color_option.clone()),
        ));
    }
    if untracked_files > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("?{}", untracked_files),
            &get_color(Color::Cyan, untracked_color_option.clone()),
        ));
    }
    if conflicts > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("{}", conflicts),
            &get_color(Color::Magenta, conflict_color_option.clone()),
        ));
    }
    if stashed {
        segments.push(PromptSegment::new_with_color(
            "".to_string(),
            &get_color(Color::Blue, stashed_color_option.clone()),
        ));
    }

    if staged_changes == 0
        && unstaged_changes == 0
        && untracked_files == 0
        && conflicts == 0
        && !stashed
    {
        segments.push(PromptSegment::new_with_color(
            "".to_string(),
            &get_color(Color::Green, clean_color_option.clone()),
        )); // Clean icon
    }

    // Assemble push/pull status
    if ahead > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("↑{}", ahead),
            &get_color(Color::White, ahead_color_option.clone()),
        ));
    }
    if behind > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("↓{}", behind),
            &get_color(Color::Red, behind_color_option.clone()),
        ));
    }

    segments
}
