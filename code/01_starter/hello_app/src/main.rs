fn main() {
    println!("{}", "*".repeat(52));
    println!("Hello, world!");

    // call the simple function
    println!("{}", "*".repeat(52));
    simple_func();

    println!("{}", "*".repeat(52));
    use_print();

    // Using the debug formatter
    println!("{}", "*".repeat(52));
    print_with_debug_formatter();
}

// simple function
fn simple_func() {
    println!("Hello Rust programming");
    println!("Rust programming is an awesome language");
}

// using print
fn use_print() {
    print!("{}", "Hello ");
    print!("{}", "World!\n");

    // using the debug formatter
    print!("{:?}", "My name is ");
    print!("{:?}", "Rust");

    // printing a new line
    print!("\n");
}

// Using the debug formatter `:?`
fn print_with_debug_formatter() {
    println!("{:?}", "Rust is a systems programming language");
}
