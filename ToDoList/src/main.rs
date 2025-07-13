use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\nTo-Do List Menu");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Tasks saved. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again"),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Failed to serialize");
    let mut file = File::create("tasks.json").expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ").trim().to_string();
    let id = if tasks.is_empty() { 1 } else { tasks.last().unwrap().id + 1 };
    tasks.push(Task {
        id,
        description,
        completed: false,
    });
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }
    for task in tasks {
        println!(
            "ID: {}, Description: {}, Completed: {}",
            task.id, task.description, task.completed
        );
    }
}

fn mark_task_complete(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as complete: ")
        .trim()
        .parse::<usize>()
        .expect("Please enter a valid number");
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Task {} marked as complete.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to delete: ")
        .trim()
        .parse::<usize>()
        .expect("Please enter a valid number");
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("Task {} deleted.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}