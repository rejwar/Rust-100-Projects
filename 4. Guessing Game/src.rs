use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("I am thinking of a number between 1 and 100. Can you guess it?");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(_) => {
                println!("Failed to read input. Please try again.");
                continue;
            }
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) if (1..=100).contains(&num) => num,
            Ok(_) => {
                println!("Please enter a number between 1-100.");
                continue;
            }
            Err(err) => {
                println!("Error: {}. Please enter a valid number.", err);
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number {} in {} attempts!", secret_number, attempts);
                break;
            }
        }
    }
}
