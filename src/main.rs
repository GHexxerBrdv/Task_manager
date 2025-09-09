use std::io::{self, Write};

mod cli_manager;
mod task;
use crate::cli_manager::cli_manager::handle_cmd;
use crate::task::task::Task;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut id = 0;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if !handle_cmd(&input, &mut tasks, &mut id) {
            break;
        }
    }
}
