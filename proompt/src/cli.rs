use clap::Parser;
use std::path::PathBuf;
use std::string::String;

// Top-level CLI
#[derive(Debug, Parser)]
#[command(
    name = "proompt",
    version = "1.0",
    author = "Carlo <carlomus@gmail.com>",
    about = "Copies one or more files to your copy buffer to feed into AI models"
)]
pub struct Cli {
    /// Optional root directory.  If omitted, we walk up to find `.git`.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Optional print the prompt generated
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub print: bool,

    /// Optional include the files in gitignore
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub include: bool,

    /// Optinonal files to skip. Uses regex to match
    #[arg(short, long, value_delimiter = ' ')]
    pub skip: Vec<String>,
}
