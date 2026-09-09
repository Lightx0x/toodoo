use anyhow::Result;
use clap::Parser;
use std::path::Path;
use toodoo::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = Path::new(TODO_PATH);
    let mut tasks = load(path)?;
    match cli.command {
        TodoCommand::List => println!("{}", list_tasks(&tasks)),
        TodoCommand::Add { texts } => {
            let task_len = tasks.len();
            add_task(&mut tasks, texts);
            save(path, &tasks)?;
            for task in &tasks[task_len..] {
                println!("Added {}", task.text);
            }
        }
        TodoCommand::Remove { ids } => {
            remove_task(&mut tasks, ids);
            save(path, &tasks)?
        }
        TodoCommand::Flip { ids } => {
            flip_task(&mut tasks, ids)?;
            save(path, &tasks)?
        }
        TodoCommand::Change { id, text } => {
            change_task(&mut tasks, id, text)?;
            save(path, &tasks)?
        }
        TodoCommand::Clear => {
            clear_tasks(&mut tasks);
            save(path, &tasks)?
        }
    }

    Ok(())
}
