use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("Basic Timer Tool");
    println!("Enter time duration (format: hours minutes seconds, e.g., 0 1 30):");

    let duration = match get_timer_input() {
        Some(dur) => dur,
        None => {
            eprintln!(
                "Invalid input. Please enter three non-negative integers (e.g., 0 1 30)."
            );
            return Ok(());
        }
    };

    println!(
        "Timer set for {:02} hours, {:02} minutes, {:02} seconds",
        duration.0, duration.1, duration.2
    );

    if duration.0 == 0 && duration.1 == 0 && duration.2 == 0 {
        println!("Duration is zero. Nothing to count down.");
        return Ok(());
    }

    start_timer(duration.0, duration.1, duration.2)?;
    println!("\nTime's up!");
    print!("\x07"); // Beep sound (ASCII bell)
    io::stdout().flush()?;

    Ok(())
}

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    if minutes >= 60 || seconds >= 60 {
        return None;
    }

    Some((hours, minutes, seconds))
}

fn start_timer(hours: u64, minutes: u64, seconds: u64) -> io::Result<()> {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;

    for remaining in (1..=total_seconds).rev() {
        let hrs = remaining / 3600;
        let mins = (remaining % 3600) / 60;
        let secs = remaining % 60;

        print!("\rTime remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush()?;
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}