use serde::{Serialize, Deserialize};
use crate::modules::{Color, PromptSegment};
use clap::Args;
use git2::{Repository, Status, StatusOptions};
use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    // URLからホストを判定するための正規表現（必要に応じて）
    static ref RE_GITHUB: Regex = Regex::new(r"github\.com").unwrap();
    static ref RE_GITLAB: Regex = Regex::new(r"gitlab\.com").unwrap();
}
#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct GitStatusOptions {
    pub default_color_option: Option<Color>,
    pub git_icon_color_option: Option<Color>,
    pub branch_color_option: Option<Color>,
    pub staged_color_option: Option<Color>,
    pub unstaged_color_option: Option<Color>,
    pub untracked_color_option: Option<Color>,
    pub conflict_color_option: Option<Color>,
    pub stashed_color_option: Option<Color>,
    pub clean_color_option: Option<Color>,
    pub ahead_color_option: Option<Color>,
    pub behind_color_option: Option<Color>,
}

pub fn get_git_status(options: GitStatusOptions) -> Vec<PromptSegment> {
    let mut segments: Vec<PromptSegment> = Vec::new();

    let get_color = |specific_color: Color, override_color: Option<Color>| {
        override_color
            .or(options.default_color_option.clone())
            .unwrap_or(specific_color)
            .to_string()
    };

    // 1. カレントディレクトリからリポジトリを探索
    let mut repo = match Repository::discover(".") {
        Ok(r) => r,
        Err(_) => return segments, // Gitリポジトリではない
    };

    // --- Remote Icon の取得 ---
    let remote_icon = if let Ok(remote) = repo.find_remote("origin") {
        let url = remote.url().unwrap_or("");
        if RE_GITHUB.is_match(url) {
            ""
        } else if RE_GITLAB.is_match(url) {
            ""
        } else {
            "󰊢"
        }
    } else {
        "󰊢"
    };
    segments.push(PromptSegment::new_with_color(
        remote_icon.to_string(),
        &get_color(Color::Blue, options.git_icon_color_option.clone()),
    ));

    // --- Branch / Detached HEAD の取得 ---
    let branch_display;
    let mut is_detached = false;

    if let Ok(head) = repo.head() {
        if head.is_branch() {
            // 通常のブランチ
            branch_display = head.shorthand().unwrap_or("unknown").to_string();
        } else {
            // 特定のブランチにいない場合（Detached HEAD）
            is_detached = true;
            branch_display = format!(
                ":{}",
                &head
                    .target()
                    .map(|oid| oid.to_string()[..7].to_string())
                    .unwrap_or_else(|| "unknown".into())
            );
        }
    } else {
        branch_display = "empty".to_string();
    }

    segments.push(PromptSegment::new_with_color(
        "".to_string(),
        &get_color(Color::White, options.git_icon_color_option.clone()),
    ));
    segments.push(PromptSegment::new_with_color(
        branch_display,
        &get_color(
            if is_detached {
                Color::Red
            } else {
                Color::Yellow
            },
            options.branch_color_option.clone(),
        ),
    ));

    // --- ステータス解析 (Staged, Unstaged, etc.) ---
    let mut opts = StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);

    let mut staged = 0;
    let mut unstaged = 0;
    let mut untracked = 0;
    let mut conflicts = 0;

    if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
        for entry in statuses.iter() {
            let s = entry.status();
            if s.is_conflicted() {
                conflicts += 1;
            }
            if s.is_wt_new() {
                untracked += 1;
            }
            if s.intersects(
                Status::WT_MODIFIED
                    | Status::WT_DELETED
                    | Status::WT_RENAMED
                    | Status::WT_TYPECHANGE,
            ) {
                unstaged += 1;
            }
            if s.intersects(
                Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_RENAMED
                    | Status::INDEX_TYPECHANGE,
            ) {
                staged += 1;
            }
        }
    }

    // --- Ahead / Behind の取得 ---
    let (mut ahead, mut behind) = (0, 0);
    if let Ok(head) = repo.head()
        && let Some(local_oid) = head.target()
        && let Ok(upstream_branch) = repo.branch_upstream_name(head.name().unwrap_or(""))
        && let Ok(upstream_oid) = repo.refname_to_id(upstream_branch.as_str().unwrap_or(""))
        && let Ok((a, b)) = repo.graph_ahead_behind(local_oid, upstream_oid)
    {
        ahead = a;
        behind = b;
    }

    // --- Stash の確認 ---
    let mut has_stash = false;
    let _ = repo.stash_foreach(|_, _, _| {
        has_stash = true;
        false // 1つ見つかれば十分なのでイテレーションを止める
    });

    // --- セグメントの組み立て ---
    if staged > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("+{}", staged),
            &get_color(Color::Green, options.staged_color_option),
        ));
    }
    if unstaged > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("!{}", unstaged),
            &get_color(Color::Red, options.unstaged_color_option),
        ));
    }
    if untracked > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("?{}", untracked),
            &get_color(Color::Cyan, options.untracked_color_option),
        ));
    }
    if conflicts > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("{}", conflicts),
            &get_color(Color::Magenta, options.conflict_color_option),
        ));
    }
    if has_stash {
        segments.push(PromptSegment::new_with_color(
            "".to_string(),
            &get_color(Color::Blue, options.stashed_color_option),
        ));
    }
    if staged == 0 && unstaged == 0 && untracked == 0 && conflicts == 0 && !has_stash {
        segments.push(PromptSegment::new_with_color(
            "".to_string(),
            &get_color(Color::Green, options.clean_color_option),
        ));
    }
    if ahead > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("↑{}", ahead),
            &get_color(Color::White, options.ahead_color_option),
        ));
    }
    if behind > 0 {
        segments.push(PromptSegment::new_with_color(
            format!("↓{}", behind),
            &get_color(Color::Red, options.behind_color_option),
        ));
    }

    segments
}
