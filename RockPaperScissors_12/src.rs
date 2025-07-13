use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock Paper Scissors!");
    println!("Instructions: Enter 'rock', 'paper', or 'scissors'. Type 'quit' to exit.");

    loop {
        println!("\nMake your choice:");

        let user_choice = get_user_choice();
        if user_choice == "quit" {
            println!("Thanks for playing! Goodbye!");
            break;
        }

        let computer_choice = get_computer_choice();
        println!("Computer chose: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("🎉 You win!"),
            GameResult::Lose => println!("😞 You lose."),
            GameResult::Draw => println!("🤝 It's a draw."),
        }
    }
}

fn get_user_choice() -> String {
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "rock" | "paper" | "scissors" | "quit" => return choice,
            _ => {
                println!("Invalid choice. Please enter 'rock', 'paper', 'scissors', or 'quit'.");
            }
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let index = rand::thread_rng().gen_range(0..choices.len());
    choices[index].to_string()
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn determine_winner(user: &str, computer: &str) -> GameResult {
    if user == computer {
        return GameResult::Draw;
    }
    match (user, computer) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => GameResult::Win,
        _ => GameResult::Lose,
    }
}