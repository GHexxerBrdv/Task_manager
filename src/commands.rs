use crate::task::{Task, load_tasks, save_tasks};

pub fn add_task(description: String) {
    let mut tasks = load_tasks();
    let id = (tasks.len() as u32) + 1;
    tasks.push(Task::new(id, description));
    save_tasks(&tasks);
    println!("Task added");
}

pub fn list_tasks() {
    let tasks = load_tasks();
    for task in tasks {
        println!(
            "[{}] {} | {}",
            task.id,
            task.description,
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
