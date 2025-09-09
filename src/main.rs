// use std::io::{self, Write};

// mod cli_manager;
// mod task;
// use crate::cli_manager::cli_manager::handle_cmd;
// use crate::task::task::Task;

mod commands;
mod task;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "A simple task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}

fn main() {
    // let mut tasks: Vec<Task> = Vec::new();
    // let mut id = 0;

    // loop {
    //     print!("> ");
    //     io::stdout().flush().unwrap();

    //     let mut input = String::new();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");

    //     if !handle_cmd(&input, &mut tasks, &mut id) {
    //         break;
    //     }
    // }

    let cli = Cli::parse();

    match cli.command {
        Command::Add { description } => commands::add_task(description),
        Command::List => commands::list_tasks(),
        Command::Done { id } => commands::mark_done(id),
        Command::Delete { id } => commands::delete_task(id),
    }
}
