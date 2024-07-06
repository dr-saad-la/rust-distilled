use std::io::{self, Write};

pub fn validate() {
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        if attempts >= max_attempts {
            println!("Maximum attempts reached. Exiting...");
            break;
        }

        print!("Enter a number between 1 and 10: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                attempts += 1;
                continue;
            }
        };

        if input >= 1 && input <= 10 {
            println!("Valid input: {}", input);
            break;
        } else {
            println!("Input out of range. Please enter a number between 1 and 10.");
            attempts += 1;
        }
    }
}
