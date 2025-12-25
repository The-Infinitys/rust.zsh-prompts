use clap::Parser;
use std::io::{self, Write};
use zsh_prompts::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let segments: Vec<PromptSegment> = cli.command.exec();

    let full_output: String = segments
        .into_iter()
        .map(|segment| segment.format())
        .collect::<Vec<String>>()
        .join(" ");

    io::stdout().write_all(full_output.as_bytes())?;
    Ok(())
}
