// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct TaskSeed {
    name: String,
    completed: bool,
}

#[derive(Debug, Serialize)]
struct Task {
    id: String,
    name: String,
    completed: bool,
}

const DB_PATH: &str = "./db.sqlite3";

#[tauri::command]
fn list_tasks() -> Result<Vec<Task>, String> {
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, completed FROM tasks")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut tasks = Vec::new();
    for task in rows {
        tasks.push(task.map_err(|e| e.to_string())?);
    }

    Ok(tasks)
}

#[tauri::command]
fn create_task(new_task: TaskSeed) -> Result<Task, String> {
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;

    let id = Uuid::new_v4();
    conn.execute(
        "INSERT INTO tasks (id, name, completed) VALUES (?1, ?2, ?3)",
        (&id.to_string(), &new_task.name, &new_task.completed),
    )
    .map_err(|e| e.to_string())
    .inspect_err(|e| {
        eprintln!("Error inserting task: {}", e);
    })?;

    let new_task_with_id = Task {
        id: id.to_string(),
        name: new_task.name,
        completed: new_task.completed,
    };

    Ok(new_task_with_id)
}

#[tauri::command]
fn toggle_task(id: String) -> Result<String, String> {
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;

    let task_completed: bool = conn
        .query_row("SELECT completed FROM tasks WHERE id = ?1", [&id], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE tasks SET completed = ?1 WHERE id = ?2",
        (!task_completed, &id),
    )
    .map_err(|e| e.to_string())
    .inspect_err(|e| {
        eprintln!("Error updating task: {}", e);
    })?;

    Ok(String::from("ok"))
}

fn main() -> anyhow::Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id VARCHAR(36) PRIMARY KEY,
            name TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        (),
    )?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_tasks,
            create_task,
            toggle_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
