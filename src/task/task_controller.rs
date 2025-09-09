use super::task::{Duration, Priority, Status, Task};

pub fn create_task(
    tasks: &mut Vec<Task>,
    id: &mut u32,
    description: &str,
    priority: Priority,
    duration: Duration,
    status: Status,
) {
    *id += 1;
    let task = Task::new(*id, description.to_string(), priority, status, duration);
    tasks.push(task);
}
