use std::io;

fn main() {
    println!("Temerature Converter");
    println!("1: Celcius to Fahrenheit");
    println!("2: Fahrenheit to Celcius ");
    println!("Please select an option (1 & ̦2)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read Input");

    let choice : u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice , Please Enter 1 or 2");
            return;
        }
    };
    
    if choice ==1 {
        celcius_to_farenheit();
    } else if choice ==2 {
        farenheit_to_celciuous();
    } else {
        println!("Invalid choice , Please Enter 1 or 2");
    }

    fn celcius_to_farenheit() {
        println!("Enter temperature in Celsius :");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read Input");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input Please enter valid number :");
                return;
            }
        };

        let farenheit = (temp *9.0 / 5.0) + 32.0;
        println!("{:.2} C is {:.2} F" ,temp, farenheit);
    }

    fn farenheit_to_celciuous() {
        println!("Enter temperature in Farenheit");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("failed to read input");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input . Plase enter valid input");
                return;
            }
        };

        let celsius = (temp -32.0) * 5.0 / 9.0;
        println!("{:.2} F is {:.2}", temp,celsius);
    }
}
