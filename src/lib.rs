mod modules;
use std::path::PathBuf;

use crate::modules::git::GitStatusOptions;
use clap::Subcommand;
pub use modules::*;
pub use serde::Deserialize;
pub use serde::Serialize;

#[derive(
    Subcommand,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
)]
pub enum Commands {
    /// Get OS icon
    Os {
        #[arg(long)]
        color: Option<String>,
    },
    /// Get current working directory info
    Pwd {
        #[arg(long)]
        color: Option<String>,
    },
    /// Get current time
    Time {
        #[arg(long)]
        color: Option<String>,
    },
    /// Get git status
    Git {
        #[command(flatten)]
        options: GitStatusOptions,
        #[arg(long)]
        #[rkyv(with = rkyv::with::Map<rkyv::with::AsString>)]
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<PathBuf>,
    },
    /// Get last command execution info
    Cmd {
        #[arg(long)]
        last_status: String,
        #[arg(long)]
        last_command_executed: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
}
impl Commands {
    pub fn exec(&self) -> Vec<PromptSegment> {
        match self {
            Self::Os { color } => {
                let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
                vec![os::get_os_icon(parsed_color)]
            }
            Self::Pwd { color } => {
                let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
                pwd::get_smart_pwd(parsed_color)
            }
            Self::Time { color } => {
                let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
                vec![time::get_time(parsed_color)]
            }
            Self::Git { options, path } => {
                let parsed_default_color = options.default_color_option;
                let parsed_git_icon_color = options.git_icon_color_option;
                let parsed_branch_color = options.branch_color_option;
                let parsed_staged_color = options.staged_color_option;
                let parsed_unstaged_color = options.unstaged_color_option;
                let parsed_untracked_color = options.untracked_color_option;
                let parsed_conflict_color = options.conflict_color_option;
                let parsed_stashed_color = options.stashed_color_option;
                let parsed_clean_color = options.clean_color_option;
                let parsed_ahead_color = options.ahead_color_option;
                let parsed_behind_color = options.behind_color_option;
                git::get_git_status(
                    GitStatusOptions {
                        default_color_option: parsed_default_color,
                        git_icon_color_option: parsed_git_icon_color,
                        branch_color_option: parsed_branch_color,
                        staged_color_option: parsed_staged_color,
                        unstaged_color_option: parsed_unstaged_color,
                        untracked_color_option: parsed_untracked_color,
                        conflict_color_option: parsed_conflict_color,
                        stashed_color_option: parsed_stashed_color,
                        clean_color_option: parsed_clean_color,
                        ahead_color_option: parsed_ahead_color,
                        behind_color_option: parsed_behind_color,
                    },
                    path,
                )
            }
            Self::Cmd {
                last_status,
                last_command_executed,
                color,
            } => {
                let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
                vec![cmd::get_execution_info(
                    last_status,
                    last_command_executed,
                    parsed_color,
                )]
            }
        }
    }
}
