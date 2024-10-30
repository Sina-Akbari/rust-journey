use std::fs;
use std::io::Error;

fn main() {
    // let text = fs::read_to_string("logs.txt");

    // println!("{:#?}", text);

    match divide(5.0, 0.0) {
        Ok(value) => {
            println!("{}", value);
        }
        Err(error) => {
            println!("{}", error);
        }
    }

    match validate_email(String::from("asdf@asdf.com")) {
        Ok(..) => println!("email is valid"),
        Err(error) => println!("{}", error),
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't devide by 0!"))
    } else {
        Ok(a / b)
    }
}
