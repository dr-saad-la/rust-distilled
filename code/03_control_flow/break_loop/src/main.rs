mod validate_user_input;
use validate_user_input::validate;

fn main() {
    banner("*", 52, "Break Statement in Rust");
    break_loop();
    break_while();
    break_for();
    search_number();
    banner("*", 52, "Practical Example of Break: Validate User Input");
    validate();
}

// Breaking the `loop`
fn break_loop() {
    banner("*", 52, "Breaking the loop:");
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count >= 5 {
            break;
        }
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52));
}

// Break while loop
fn break_while() {
    banner("*", 52, "Breaking While Loop");
    let mut count = 0;

    while count < 10 {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            println!("Count has reached 5, breaking the loop.");
            break;
        }
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52))
}

// Break for loop
fn break_for() {
    banner("*", 52, "Breaking for loop");
    for i in 1..10 {
        if i == 5 {
            println!("Reached the number: {}", i);
            break;
        }
        println!("Current number: {}", i);
    }
    println!("Loop has ended.");
    println!("{}", "*".repeat(52));
}

// Practical Example
fn search_number() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    let mut found = false;

    for &number in numbers.iter() {
        if number == target {
            found = true;
            break;
        }
    }

    if found {
        println!("Found the target number: {}", target);
    } else {
        println!("Target number not found.");
    }
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
