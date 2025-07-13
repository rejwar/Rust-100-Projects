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

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdin().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input ");
    input
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("Task.json") {
        Ok(content) => serde_json::from_str(& content).unwrap_or_else(|_| Vec::new())
        Err(_) => Vec::new(),
    }
}

fn save_tasks(Tasks: &Vec<Task>) {
    let json = serde_json::to_string(Tasks).expect("Failed to serailize ");
    let mut file = File::create("Task.json").expect("Failed to save task");
    file.write_all(json.as_bytes()).expect("failed to get");
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description");
    let id = tasks.len() +1 ;
    Tasks.push(Task {
        id,
        description: description.trim().to_string(),
    })
}