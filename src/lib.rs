use clap::{Parser, Subcommand};
use anyhow::{Result};
use std::{fmt::Write};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: TodoCommand,
}

#[derive(Subcommand)]
pub enum TodoCommand {
    /// Add to list
    Add { text: String },
    /// Show list
    List,
    /// Mark list with id as done
    Done { id: usize }
}

#[derive(Debug, PartialEq)]
struct Task {
    id: usize,
    text: String,
    done: bool
}

fn add_task(tasks: &mut Vec<Task>, text: String) {
    let new_task = Task {
        text,
        id: tasks.len() + 1,
        done: false
    };

    tasks.push(new_task);
}

fn list_tasks(tasks: &[Task]) -> String {
    let mut list = String::new();
    for task in tasks {
        let task_status = if task.done { "✓" } else { " " };
        writeln!(list, "[{}] {}: {}", task_status, task.id, task.text).unwrap()
    }
    list
}

fn mark_done(tasks: &mut Vec<Task>, id: usize) -> Result<()> {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        Ok(())
    } else {
        anyhow::bail!("No task with id: {id}")
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn add_task_to_empty_list_assigns_id_1() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Need to review assignment".to_string());

        let expected = vec![Task {
            text: "Need to review assignment".to_string(),
            done: false,
            id: 1
        }];
        assert_eq!(tasks, expected);
    }

    #[test]
    fn compare_listed_tasks() {
        let mut tasks = Vec::new(); 
        let task1 = Task {
            text: "Find the missing key".to_string(),
            done: true,
            id: 1
        };

        let task2 = Task {
            text: "Find the missing value".to_string(),
            done: false,
            id: 2
        };

        tasks.push(task1);
        tasks.push(task2);

        assert_eq!(list_tasks(&tasks), "[✓] 1: Find the missing key\n[ ] 2: Find the missing value\n");
    }

    #[test]
    fn mark_done_flips_done_flag() {
        let mut tasks = vec![
            Task { id: 1, text: "This is done".to_string(), done: true },
            Task { id: 2, text: "This is not done".to_string(), done: false },
        ];

        assert!(mark_done(&mut tasks, 2).is_ok());

        let task = tasks.iter().find(|t| t.id == 2).unwrap();

        assert!(task.done);
    }

    #[test]
    fn mark_done_returns_err_for_unknown_id() {
        let mut tasks = vec![
            Task { id: 1, text: "This is done".to_string(), done: true },
        ];

        assert!(mark_done(&mut tasks, 99).is_err());
    }
}
