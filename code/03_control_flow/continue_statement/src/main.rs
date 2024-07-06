fn main() {
    continue_with_for();
    continue_with_while();
    continue_with_loop();
    continue_with_break()
}

// Continue with for loop
fn continue_with_for() {
    banner("*", 52, "Using continue with for loop");
    for number in 1..10 {
        if number % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", number);
    }
    println!("Loop ended here");
    println!("{}", "*".repeat(52))
}

// Continue with while loop

fn continue_with_while() {
    banner("*", 52, "Using continue with while loop");
    let mut i = 1;

    while i < 10 {
        if i % 2 == 0 {
            i += 1;
            continue;
        }
        println!("Odd number: {}", i);
        i += 1;
    }
    println!("Program ends");
    println!("{}", "*".repeat(52))
}

// continue with loop
fn continue_with_loop() {
    banner("*", 52, "Continue statement with loop");
    let mut count = 0;

    loop {
        count += 1;

        if count % 2 == 0 {
            continue;
        }

        if count > 10 {
            break;
        }

        println!("Odd count: {}", count);
    }
    println!("Program ended!");
    println!("{}", "*".repeat(52));
}

// combine continue with break
fn continue_with_break() {
    banner("*", 52, "Continue-Break Combination");
    let mut count = 0;

    loop {
        count += 1;

        // Skip even numbers
        if count % 2 == 0 {
            continue;
        }

        // Print the odd number
        println!("Odd number: {}", count);

        // Break the loop when count reaches 15
        if count >= 15 {
            println!("Reached the limit, breaking the loop.");
            break;
        }
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52));
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
