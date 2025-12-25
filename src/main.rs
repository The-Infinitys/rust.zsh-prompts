use clap::{Parser, Subcommand};
use std::io::{self, Write};
use zsh_prompts::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        git_icon_color: Option<String>,
        #[arg(long)]
        branch_color: Option<String>,
        #[arg(long)]
        staged_color: Option<String>,
        #[arg(long)]
        unstaged_color: Option<String>,
        #[arg(long)]
        untracked_color: Option<String>,
        #[arg(long)]
        conflict_color: Option<String>,
        #[arg(long)]
        stashed_color: Option<String>,
        #[arg(long)]
        clean_color: Option<String>,
        #[arg(long)]
        ahead_color: Option<String>,
        #[arg(long)]
        behind_color: Option<String>,
    },
    /// Get last command execution info
    Cmd {
        #[arg(long)]
        last_status: i32,
        #[arg(long)]
        last_command_executed: Option<f64>,
        #[arg(long)]
        color: Option<String>,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let segments: Vec<PromptSegment> = match &cli.command {
        Commands::Os { color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![os::get_os_icon(parsed_color)]
        }
        Commands::Pwd { color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![pwd::get_smart_pwd(parsed_color)]
        }
        Commands::Time { color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![time::get_time(parsed_color)]
        }
        Commands::Git {
            color,
            git_icon_color,
            branch_color,
            staged_color,
            unstaged_color,
            untracked_color,
            conflict_color,
            stashed_color,
            clean_color,
            ahead_color,
            behind_color,
        } => {
            let parsed_default_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            let parsed_git_icon_color = git_icon_color
                .as_ref()
                .and_then(|c| c.parse::<Color>().ok());
            let parsed_branch_color = branch_color.as_ref().and_then(|c| c.parse::<Color>().ok());
            let parsed_staged_color = staged_color.as_ref().and_then(|c| c.parse::<Color>().ok());
            let parsed_unstaged_color = unstaged_color
                .as_ref()
                .and_then(|c| c.parse::<Color>().ok());
            let parsed_untracked_color = untracked_color
                .as_ref()
                .and_then(|c| c.parse::<Color>().ok());
            let parsed_conflict_color = conflict_color
                .as_ref()
                .and_then(|c| c.parse::<Color>().ok());
            let parsed_stashed_color = stashed_color.as_ref().and_then(|c| c.parse::<Color>().ok());
            let parsed_clean_color = clean_color.as_ref().and_then(|c| c.parse::<Color>().ok());
            let parsed_ahead_color = ahead_color.as_ref().and_then(|c| c.parse::<Color>().ok());
            let parsed_behind_color = behind_color.as_ref().and_then(|c| c.parse::<Color>().ok());
            git::get_git_status(
                parsed_default_color,
                parsed_git_icon_color,
                parsed_branch_color,
                parsed_staged_color,
                parsed_unstaged_color,
                parsed_untracked_color,
                parsed_conflict_color,
                parsed_stashed_color,
                parsed_clean_color,
                parsed_ahead_color,
                parsed_behind_color,
            )
        }
        Commands::Cmd {
            last_status,
            last_command_executed,
            color,
        } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![cmd::get_execution_info(
                *last_status,
                *last_command_executed,
                parsed_color,
            )]
        }
    };

    let full_output: String = segments
        .into_iter()
        .map(|segment| segment.format())
        .collect::<Vec<String>>()
        .join(" ");

    io::stdout().write_all(full_output.as_bytes())?;
    Ok(())
}
