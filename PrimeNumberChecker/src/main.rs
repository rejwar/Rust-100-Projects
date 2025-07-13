use std::io;

fn main() {
    println!("Prime Number  Checker ");
    println!("Enter a positive integer to check if it's prime or not ");

    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input . Please enter a positive integer"),
            return;
        }
    };

    if number <= 1 {
        println!("Number must be greater than 1");
        return;
    }

    if is_prime(number) {
        println!("{} is a number " ,number);
    } else {
        println!("{} is not a prime number ", number);
    }
}

