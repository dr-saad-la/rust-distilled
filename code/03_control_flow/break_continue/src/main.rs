fn main() {
    // Breaking out of loops
    banner("*", 52, "Breaking out of loops");
    break_loop();

    banner("*", 52, "Breaking while loop");
    break_while();

    banner("*", 52, "Breaking out of for loop");
    break_for();

    banner("*", 52, "Continue with for loop");

    banner("*", 52, "Continue with while loop");

    banner("*", 52, "Continue with `loop` loop");
}

// break out of loops
#[allow(dead_code)]
fn break_loop() {
    let mut counter = 0;

    loop {
        println!("Counter: {}", counter);
        counter += 1;

        if counter == 5 {
            println!("Breaking the loop at counter : {}", counter);
            break;
        }
    }
}

// Breaking out of while loop
fn break_while() {
    let mut counter = 10;

    while counter >= 0 {
        println!("The counter: {}", counter);
        counter -= 1;

        if counter == 4 {
            println!("Breaking loop at counter: {}", counter);
            break;
        }
    }
}

// break out of for loop
fn break_for() {
    // let num = 0;
    let v: Vec<i32> = vec![2, 3, 1, 6, 7, 9, 10];

    for &i in v.iter() {
        println!("{}", i);
        if i >= 6 {
            println!("The element is greater or equal to 6");
            break;
        }
    }
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); // Creating a separator line
    let message = format!("{:^width$}", message, width = nchar); // Centering the message
    println!("\n{}\n{}\n{}", sep, message, sep); // Printing the banner
}
