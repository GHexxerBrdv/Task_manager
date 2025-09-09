// use crate::task::task::{Duration, Priority, Status, Task};
// use crate::task::task_controller;

// pub fn handle_cmd(input: &str, tasks: &mut Vec<Task>, id: &mut u32) -> bool {
//     let parts: Vec<&str> = input.trim().splitn(2, "").collect();

//     match parts[0] {
//         "add" => {
//             if parts.len() > 1 {
//                 // task_controller::create_task(tasks, id, description, priority, status, duration);
//             } else {
//                 println!("Usage: add <description> [priority] [duration]");
//             }
//         }
//         "list" => {}
//         "delete" => {}
//         "update" => {}
//         "done" => {}
//         "exit" => {
//             return false;
//         }
//         "" => {}
//         _ => println!("Invalid Command: {}", parts[0]),
//     }
//     return true;
// }
