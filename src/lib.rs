use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fmt::Write, fs, io, path::Path};

#[cfg(test)]
mod test;

pub const TODO_PATH: &str = "todo.json";

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
        texts: Vec<String> 
    },
    /// Remove from list by id(s)
    Remove { 
        #[arg(num_args = 1..)]
        ids: Vec<usize>
    },
    /// Show list
    List,
    /// Mark task(s) with id as done
    Done { 
        #[arg(num_args = 1..)]
        ids: Vec<usize>
    },
    /// Mark task(s) with id as undone
    Undone { 
        #[arg(num_args = 1..)]
        ids: Vec<usize>
    },
    /// Change task in list by id
    Change { id: usize, text: String }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    id: usize,
    text: String,
    done: bool,
}

pub fn save(path: &Path, tasks: &[Task]) -> Result<()> {
    let content = serde_json::to_string_pretty(tasks)?;
    fs::write(path, content).with_context(|| format!("failed to write {}", path.display()))?;

    Ok(())
}

pub fn load(path: &Path) -> Result<Vec<Task>> {
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content)
            .with_context(|| format!("failed to parse {}", path.display())),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e).with_context(|| format!("failed to read {}", path.display())),
    }
}

pub fn add_task(tasks: &mut Vec<Task>, texts: Vec<String>) {
    let next_id = tasks.len();

    for (i, text) in texts.into_iter().enumerate() {
        tasks.push(Task {
            text,
            id: next_id + 1 + i,
            done: false,
        })
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, ids: Vec<usize>) {
    tasks.retain(|t| !ids.contains(&t.id));
    for (new_id, task) in tasks.iter_mut().enumerate() {
        task.id = new_id + 1; 
    }
}

pub fn list_tasks(tasks: &[Task]) -> String {
    let mut list = String::new();
    for task in tasks {
        let task_status = if task.done { "✓" } else { " " };
        writeln!(list, "[{}] {}: {}", task_status, task.id, task.text).unwrap()
    }
    list
}

pub fn mark_done(tasks: &mut [Task], ids: Vec<usize>) -> Result<()> {
    for id in ids {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
        } else {
            anyhow::bail!("No task with id: {id}")
        }
    }

    Ok(())
}

pub fn mark_undone(tasks: &mut [Task], ids: Vec<usize>) -> Result<()> {
    for id in ids {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.done = false;
        } else {
            anyhow::bail!("No task with id: {id}")
        }
    }

    Ok(())
}

pub fn change_task(tasks: &mut [Task], id: usize, text: String) -> Result<()> {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.text = text;
        Ok(())
    } else {
        anyhow::bail!("No task with id: {id}")
    }
}
