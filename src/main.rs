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
            let removed: Vec<String> = tasks
                .iter()
                .filter(|t| ids.contains(&t.id))
                .map(|t| t.text.clone())
                .collect();
            remove_task(&mut tasks, ids);
            save(path, &tasks)?;
            for text in removed {
                println!("Removed {text}");
            }
        }
        TodoCommand::Flip { ids } => {
            flip_task(&mut tasks, ids.clone())?;
            save(path, &tasks)?;
            for task in tasks.iter().filter(|t| ids.contains(&t.id)) {
                let status = if task.done { "done" } else { "undone" };
                println!("Flipped task {}: {} -> {status}", task.id, task.text);
            }
        }
        TodoCommand::Change { id, text } => {
            let old = tasks.iter().find(|t| t.id == id).map(|t| t.text.clone());
            change_task(&mut tasks, id, text)?;
            save(path, &tasks)?;
            if let (Some(old), Some(task)) = (old, tasks.iter().find(|t| t.id == id)) {
                println!("Changed task {}: {old} -> {}", task.id, task.text);
            }
        }
        TodoCommand::Clear => {
            let count = tasks.len();
            clear_tasks(&mut tasks);
            save(path, &tasks)?;
            println!("Cleared {count} task(s)");
        }
    }

    Ok(())
}
