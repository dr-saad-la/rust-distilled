fn main() {
    banner("*", 52, "Loops in Rust");
    simple_for_loop();

    banner("*", 52, "Loops in Rust");

    banner("*", 52, "While loop");
    simple_while_loop();
}

// simple loop
#[allow(dead_code)]
fn simple_loop() {}

// for loop
fn simple_for_loop() {
    for i in 0..5 {
        println!("{}", i)
    }
}

// While loop
fn simple_while_loop() {
    let mut num = 5;
    while num > 0 {
        println!("{}", num);
        num = num - 1;
    }
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
