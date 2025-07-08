fn main() {

    println!("Fibonacci Sequence Generator ");
    println!("Enter the numebr of terms you want to generate");

    let num_terms = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input . Please enter a positive integer");
            return;
        }
    };

    if num_terms ==0 {
        println!("Number of terms must be greater than zero");
        return;
    }

    let Sequence = generate_fibonacci(num_terms);
    println!("Fibonacci Sequence ( {} terms ) : {:?}", num_terms,Sequence);
}


fn get_input_as_u32 () -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64>
{
    let mut Sequence = Vec::new();

    if n >= 1 {
        Sequence.push(0);
    }
    if n > 2 {
        Sequence.push(1);
    }
    for i in 2..n {
        let next = Sequence[ i as usize-1] + Sequence [ i as usize -2];
        Sequence.push(next);
    }
    Sequence
}

