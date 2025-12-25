use clap::{Parser, Subcommand};
use std::io::{self, Write};

mod modules;

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

    let output = match &cli.command {
        Commands::Os => modules::os::get_os_icon(),
        Commands::Pwd => modules::pwd::get_smart_pwd(),
        Commands::Time => modules::time::get_time(),
        Commands::Git => modules::git::get_git_status(),
        Commands::Cmd { last_status, last_command_executed } => {
            modules::cmd::get_execution_info(*last_status, *last_command_executed)
        }
    };

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}