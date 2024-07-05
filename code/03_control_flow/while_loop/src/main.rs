fn main() {
    banner("*", 52, "While loop Examples");
    simple_while_loop();

    while_with_increment();

    complex_while_loop();
    nested_while_loop();
    safe_while_loop();
}

// simple while_loop

fn simple_while_loop() {
    let mut count = 5;

    while count > 0 {
        println!("Count: {}", count);
        count -= 1; // the short and common way of decrementing

        // The previous can be written like this
        // count = count - 1;     //
    }

    println!("Loop has ended.");
}

// other example where a counter is incrementing
fn while_with_increment() {
    banner("*", 52, "Another while loop example");
    let mut num = 0;

    while num <= 5 {
        println!(" {}", num);
        num += 1;
    }

    println!(" End of while loop!!!");
    println!("{}", "*".repeat(52))
}

// Complex while loop
fn complex_while_loop() {
    banner("*", 52, "Complex while loop");
    let mut x = 5;
    let mut y = 10;

    while x < y && y < 20 {
        println!("x: {}, y: {}", x, y);
        x += 1;
        y += 2;
    }

    println!("Loop has ended.");
}

// Nested while loop
fn nested_while_loop() {
    banner("*", 52, "Nested while loop");

    let mut i = 0;

    while i < 3 {
        let mut j = 0;

        while j < 3 {
            println!("i: {}, j: {}", i, j);
            j += 1;
        }

        i += 1;
    }
    println!("End of loop");
}

// safe while loop
fn safe_while_loop() {
    banner("*", 52, "Safe While Loop");
    let mut count = 0;
    let max_iterations = 10;

    while count < max_iterations {
        // Perform some task
        println!("Iteration: {}", count);

        count += 1;

        // Safe exit condition
        if count == max_iterations {
            println!("Reached maximum iterations, exiting loop.");
            break;
        }
    }
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
