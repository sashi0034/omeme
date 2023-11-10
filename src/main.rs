use std::io::{self, Write};
use std::env;

fn main() {
    match env::current_dir() {
        Ok(path) => println!("Current directory: {:?}", path),
        Err(e) => println!("Error: {}", e),
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        match input {
            "exit" => break,
            _ => println!("Unknown : {}", input),
        }
    }
}