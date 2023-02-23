#![forbid(unsafe_code)]

use std::io;
use std::error::Error;

pub fn get_user_input(prompt: &str) -> Result<i32, Box<dyn Error>> {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input)?;

    let number: i32 = user_input.trim().parse().map_err(|_| "Please enter a valid integer!")?;

    if number <= 0 {
        return Err(From::from("The number must be greater than 0!"));
    }

    Ok(number)
}

pub fn input(prompt: &str) -> i32 {
    loop {
        match get_user_input(prompt) {
            Ok(count) => {
                return count;
            },
            Err(error) => {
                println!("Error: {}", error);
            },
        };
    }
}