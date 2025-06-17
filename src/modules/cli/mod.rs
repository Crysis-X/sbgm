use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommands>,
    
    #[arg(short, long)]
    pub path: Option<String>,
}
#[derive(Subcommand)]
pub enum CliCommands {
    FolderIndex,
    Index,
    Prev,
    Next,
    PrevFolder,
    NextFolder,
    Set {
        #[arg(short, long)]
        index: usize
    },
    SetFolder {
        #[arg(short, long)]
        index: usize
    }
}