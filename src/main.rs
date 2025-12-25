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
        },
        Commands::Pwd { color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![pwd::get_smart_pwd(parsed_color)]
        },
        Commands::Time { color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![time::get_time(parsed_color)]
        },
        Commands::Git { color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            git::get_git_status(parsed_color)
        },
        Commands::Cmd { last_status, last_command_executed, color } => {
            let parsed_color = color.as_ref().and_then(|c| c.parse::<Color>().ok());
            vec![cmd::get_execution_info(*last_status, *last_command_executed, parsed_color)]
        }
    };

    let full_output: String = segments.into_iter()
        .map(|segment| segment.format())
        .collect::<Vec<String>>()
        .join(" ");

    io::stdout().write_all(full_output.as_bytes())?;
    Ok(())
}