use clap::Parser;
use toodoo::{Cli, TodoCommand};

fn main() {
    let cli = Cli::parse();
    match cli.command {
         TodoCommand::List => println!("You have a list"),
         TodoCommand::Add { text } => println!("Added {text} to list"),
         TodoCommand::Done { id } => println!("Task {id} is done")
    }
}
