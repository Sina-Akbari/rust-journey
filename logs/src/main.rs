use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())
    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let error_logs = extract_errors(text.as_str());

    // fs::write("errors.txt", errors.join("\n")).expect("Failed to write to errors.txt");

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Errors extracted successfully!"),
    //             Err(error) => println!("{}", error),
    //         }
    //     }
    //     Err(error) => {
    //         println!("Failed to read file: {}", error);
    //     }
    // }
}
