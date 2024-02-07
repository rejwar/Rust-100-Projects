use std::fs::{self , File};
use std::io::{self, Write};
use serde::{Serialize , Deserialize};

#[derive(Serialize , Deserialize , Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn mut() {
    let mut Task: Vec<Task> = load_tasks();

    loop {
        println!("\n To do list Menu");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3 . Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("Enter your choice ");
        match choice.trim() {
            "1" => add_task(&mut Tasks),
            "2" => view_task(&Tasks),
            "3" => mark_task_complete(&mut Tasks),
            "4" => delete_task(&mut Tasks),
            "5" => {
                save_tasks(&Tasks);
                println!("Tasks saved . Goodbye");
                break;
            }
            _ => println!("Invalid choice . Please try again");
    }
}
}