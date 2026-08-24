use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fmt::Write, fs, io, path::Path};

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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn add_task_to_empty_list_assigns_id_1() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, vec!["Need to review assignment".to_string()]);

        let expected = vec![Task {
            text: "Need to review assignment".to_string(),
            done: false,
            id: 1,
        }];
        assert_eq!(tasks, expected);
    }

    #[test]
    fn compare_listed_tasks() {
        let mut tasks = Vec::new();
        let task1 = Task {
            text: "Find the missing key".to_string(),
            done: true,
            id: 1,
        };

        let task2 = Task {
            text: "Find the missing value".to_string(),
            done: false,
            id: 2,
        };

        tasks.push(task1);
        tasks.push(task2);

        assert_eq!(
            list_tasks(&tasks),
            "[✓] 1: Find the missing key\n[ ] 2: Find the missing value\n"
        );
    }

    #[test]
    fn mark_done_flips_done_flag() {
        let mut tasks = vec![
            Task {
                id: 1,
                text: "This is done".to_string(),
                done: true,
            },
            Task {
                id: 2,
                text: "This is not done".to_string(),
                done: false,
            },
        ];

        assert!(mark_done(&mut tasks, vec![2]).is_ok());

        let task = tasks.iter().find(|t| t.id == 2).unwrap();

        assert!(task.done);
    }

    #[test]
    fn mark_done_returns_err_for_unknown_id() {
        let mut tasks = vec![Task {
            id: 1,
            text: "This is done".to_string(),
            done: true,
        }];

        assert!(mark_done(&mut tasks, vec![99]).is_err());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();

        let path = dir.path().join("weez_todo_roundtrip.json");
        let tasks = vec![
            Task {
                id: 1,
                text: "first".to_string(),
                done: false,
            },
            Task {
                id: 2,
                text: "second".to_string(),
                done: true,
            },
        ];

        save(&path, &tasks).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded, tasks);
    }

    #[test]
    fn load_returns_empty_vec_when_file_missing() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("weez_todo_definitely_missing.json");

        let loaded = load(&path).unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn different_ids_after_task_remove() {
        let mut tasks = vec![
            Task {
                id: 1,
                text: "first".to_string(),
                done: false,
            },
            Task {
                id: 2,
                text: "second".to_string(),
                done: true,
            },
            Task {
                id: 3,
                text: "third".to_string(),
                done: false,
            },
        ];

        remove_task(&mut tasks, vec![2]);
        add_task(&mut tasks, vec!["fourth".to_string()]);

        let ids = tasks.iter().map(|t| t.id).collect::<HashSet<usize>>();

        assert_eq!(tasks.len(), ids.len());
        assert_eq!(tasks[2].id, 3);
    }

    #[test]
    fn change_task_from_id() {
        let mut tasks = vec![
            Task {
                id: 1,
                text: "first".to_string(),
                done: false,
            },
            Task {
                id: 2,
                text: "second".to_string(),
                done: true,
            },
            Task {
                id: 3,
                text: "third".to_string(),
                done: false,
            },
        ];
        change_task(&mut tasks, 1, "not first".to_string()).unwrap();

        let result = tasks.iter_mut().filter(|t| t.id == 1).map(|t| t.text.clone()).collect::<String>();

        assert_eq!(result, "not first");
    }
}
