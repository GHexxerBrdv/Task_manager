use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: bool,
    pub category: String,
    pub priority: Priority,
    pub due_date: Option<String>,
}

impl Task {
    pub fn new(
        id: u32,
        description: String,
        category: String,
        priority: Priority,
        due_date: Option<String>,
    ) -> Self {
        Self {
            id,
            description,
            status: false,
            category,
            priority,
            due_date,
        }
    }
}

pub fn load_tasks() -> Vec<Task> {
    if Path::new("tasks.json").exists() {
        let data = fs::read_to_string("tasks.json").unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        Vec::new()
    }
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}
