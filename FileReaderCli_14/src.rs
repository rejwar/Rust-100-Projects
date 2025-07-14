use std::env;
use std::fs::File;
use std::io::{self , BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <2 {
        eprintln!("Usage cargo run < File path> [--lines ] [--search");
        return;
    }

    let file_path = &args[1];
    let show_lines = args.contains(& "--lines ". to_string());

    let keyword = if let Some(pos) = args.iter().position(|x| x  == "__search") {
        args.get(pos +1)
    } else {
        None
    };
}