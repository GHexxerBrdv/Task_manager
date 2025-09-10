use crate::task::{Priority, Task, load_tasks, save_tasks};

pub fn add_task(description: String, category: String, priority: String, due: Option<String>) {
    let mut tasks = load_tasks();
    let id = (tasks.len() as u32) + 1;

    let priority_enum = match priority.to_lowercase().as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        _ => Priority::Medium,
    };
    tasks.push(Task::new(id, description, category, priority_enum, due));
    save_tasks(&tasks);
    println!("Task added");
}

pub fn list_tasks(category: Option<String>) {
    let tasks = load_tasks();
    for task in tasks {
        if let Some(ref cat) = category {
            if &task.category != cat {
                continue;
            }
        }
        println!(
            "[{}] {} | Category: {} | Priority: {:?} | Due: {} | Status: {}",
            task.id,
            task.description,
            task.category,
            task.priority,
            task.due_date.clone().unwrap_or("N/A".to_string()),
            if task.status { "Done" } else { "Pending" }
        );
    }
}

pub fn mark_done(id: u32) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.status = true;
        save_tasks(&tasks);
        println!("Task {} marked done", id);
    } else {
        println!("Task not found");
    }
}

pub fn delete_task(id: u32) {
    let mut tasks = load_tasks();
    let before = tasks.len();
    tasks.retain(|t| t.id != id);
    save_tasks(&tasks);

    if tasks.len() < before {
        println!("Task {} deleted", id);
    } else {
        println!("Task not found");
    }
}
