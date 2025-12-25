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
    Os,
    /// Get current working directory info
    Pwd,
    /// Get current time
    Time,
    /// Get git status
    Git,
    /// Get last command execution info
    Cmd {
        #[arg(long)]
        last_status: i32,
        #[arg(long)]
        last_command_executed: Option<f64>,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let segments: Vec<PromptSegment> = match &cli.command {
        Commands::Os => vec![os::get_os_icon()],
        Commands::Pwd => vec![pwd::get_smart_pwd()],
        Commands::Time => vec![time::get_time()],
        Commands::Git => git::get_git_status(),
        Commands::Cmd { last_status, last_command_executed } => {
            vec![cmd::get_execution_info(*last_status, *last_command_executed)]
        }
    };

    let full_output: String = segments.into_iter()
        .map(|segment| segment.format())
        .collect::<Vec<String>>()
        .join(" ");

    io::stdout().write_all(full_output.as_bytes())?;
    Ok(())
}