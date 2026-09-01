use std::collections::HashSet;
use tempfile::TempDir;

use super::*;
#[test]
fn empty_list_prints_no_task() {
    let tasks = Vec::new();
    assert_eq!(list_tasks(&tasks), "No Tasks found");
}

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
fn add_multiple_tasks() {
    let mut tasks = Vec::new();
    add_task(
        &mut tasks,
        vec![
            "Need to review assignment".to_string(),
            "Need to dress".to_string(),
        ],
    );

    let expected = vec![
        Task {
            text: "Need to review assignment".to_string(),
            done: false,
            id: 1,
        },
        Task {
            text: "Need to dress".to_string(),
            done: false,
            id: 2,
        },
    ];
    assert_eq!(tasks, expected);
}

#[test]
fn remove_multiple_tasks() {
    let mut tasks = Vec::new();
    add_task(
        &mut tasks,
        vec![
            "Need to review assignment".to_string(),
            "Need to dress".to_string(),
        ],
    );
    remove_task(&mut tasks, vec![1, 2]);

    let expected = vec![];
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
fn flips_task_to_done() {
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

    assert!(flip_task(&mut tasks, vec![2]).is_ok());

    let task = tasks.iter().find(|t| t.id == 2).unwrap();

    assert!(task.done);
}

#[test]
fn flips_task_to_undone() {
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

    assert!(flip_task(&mut tasks, vec![1]).is_ok());

    let task = tasks.iter().find(|t| t.id == 1).unwrap();

    assert!(!task.done);
}

#[test]
fn flip_returns_err_for_unknown_id() {
    let mut tasks = vec![Task {
        id: 1,
        text: "This is done".to_string(),
        done: true,
    }];

    assert!(flip_task(&mut tasks, vec![99]).is_err());
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

    let result = tasks
        .iter_mut()
        .filter(|t| t.id == 1)
        .map(|t| t.text.clone())
        .collect::<String>();

    assert_eq!(result, "not first");
}

#[test]
fn change_returns_err_for_unknown_id() {
    let mut tasks = vec![Task {
        id: 1,
        text: "This is unchanged".to_string(),
        done: true,
    }];

    assert!(change_task(&mut tasks, 99, "This is changed".to_string()).is_err());
}

#[test]
fn clear_tasks_returns_empty_vec() {
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

    assert!(clear_tasks(&mut tasks).is_ok());
    assert_eq!(tasks, vec![]);
}
