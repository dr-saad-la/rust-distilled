fn main() {
    // Looping with `loop`
    banner("*", 52, "Looping with `loop` in Rust");
    simple_loop();

    // the for loop
    banner("*", 52, "For loop in Rust");
    simple_for_loop();

    // while loop
    banner("*", 52, "While Loop in Rust");
    simple_while_loop();
}

// Function to demonstrate a simple loop
#[allow(dead_code)]
fn simple_loop() {
    let mut counter = 0;

    loop {
        println!("Counter: {}", counter);
        counter += 1;

        // To exit the infinite loop when counter reaches 5
        // if counter >= 5 {
        //     println!("Exiting the loop.");
        //     break;
        // }
    }
}

// We use the if to break out of the infinite loop,
// if we didn't do that, we should stop using `Ctrl+C` to stop the loop.

// Function to demonstrate a for loop
fn simple_for_loop() {
    for i in 0..5 {
        println!("{}", i);
    }
}

// Function to demonstrate a while loop
fn simple_while_loop() {
    let mut num = 5;
    while num > 0 {
        println!("{}", num);
        num -= 1; // Decrementing the value of num
    }
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); // Creating a separator line
    let message = format!("{:^width$}", message, width = nchar); // Centering the message
    println!("\n{}\n{}\n{}", sep, message, sep); // Printing the banner
}
