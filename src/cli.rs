use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: TodoCommand,
}

#[derive(Subcommand)]
pub enum TodoCommand {
    /// Add to list
    Add {
        #[arg(num_args = 1..)]
        texts: Vec<String>,
    },
    /// Remove from list by id(s)
    Remove {
        #[arg(num_args = 1..)]
        ids: Vec<usize>,
    },
    /// Show list
    List,
    /// Flips task(s) between done and undone
    Flip {
        #[arg(num_args = 1..)]
        ids: Vec<usize>,
    },
    /// Change task in list by id
    Change { id: usize, text: String },
}
