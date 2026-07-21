use std::path::Path;
use anyhow::Result;
use clap::Parser;
use toodoo::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = Path::new(TODO_PATH);
    let mut tasks = load(path)?;
    match cli.command {
        TodoCommand::List => println!("{}", list_tasks(&tasks)),
        TodoCommand::Add { text } => { 
            add_task(&mut tasks, text);
            save(path, &tasks)?
        },
        TodoCommand::Remove { id } => {
            remove_task(&mut tasks, id);
            save(path, &tasks)?
        },
        TodoCommand::Done { id } => {
            mark_done(&mut tasks, id)?;
            save(path, &tasks)?
        }
    }

    Ok(())
}
