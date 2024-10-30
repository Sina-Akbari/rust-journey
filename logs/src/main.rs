use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            println!("{}", text.len());
        }
        Err(error) => {
            println!("Failed to read file: {}", error);
        }
    }
}
