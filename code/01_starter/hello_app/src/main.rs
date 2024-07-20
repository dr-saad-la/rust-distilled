//================================================================================
//      By: Dr. Saad Laouadi
//      Course: Rust Distilled
//      Lesson: Hello World Application in Rust
//
//      © copy-right: Dr. Saad Laouadi
//================================================================================

/// The main function serves as the entry point for the program.
/// It prints a banner, calls various functions to demonstrate different
/// printing methods in Rust, and shows how to use the debug formatter.
fn main() {
    banner("*", 52, "Hello World App in Rust");
    println!("Hello, world!");

    // Call the simple function
    println!("{}\n", "*".repeat(52));
    use_println();

    println!("{}\n", "*".repeat(52));
    use_print();

    // Using the debug formatter
    println!("{}\n", "*".repeat(52));
    print_with_debug_formatter();

    println!("{}", "*".repeat(52));
}

/// A simple function that prints introductory messages about Rust programming.
fn use_println() {
    println!("Hello Rust programming!");
    println!("Rust programming is an awesome language");
}

#[allow(dead_code)]
fn use_println_docted() {
    // The `println!` macro is used to print to the console with a newline at the end.
    // Print the first message: "Hello Rust programming!"
    println!("Hello Rust programming!");

    // Print the second message: "Rust programming is an awesome language"
    println!("Rust programming is an awesome language");
}

/// Demonstrates the use of the `print!` macro in Rust for printing without a newline,
/// and the use of the debug formatter `:?`.
fn use_print() {
    print!("{}", "Hello ");
    print!("{}", "World!\n");

    // Using the debug formatter
    print!("{:?}", "My name is ");
    print!("{:?}", "Rust");

    // Printing a new line
    print!("\n");
}

// Demonstrates the use of the `print!` macro and the debug formatter
#[allow(dead_code)]
fn use_print_ducmented() {
    // The `print!` macro is used to print to the console without a newline at the end.
    // This allows for continuous printing on the same line.

    // Print "Hello " without a newline
    print!("{}", "Hello ");

    // Print "World!" followed by a newline character
    print!("{}", "World!\n");

    // Using the debug formatter `:?` to print debug information
    // The `{:?}` marker tells `print!` to use the `Debug` trait for the value provided.
    // This is useful for debugging as it shows the value in a human-readable format.

    // Print "My name is " using the debug formatter (Note: debug formatting isn't necessary here for a simple string)
    print!("{:?}", "My name is ");

    // Print "Rust" using the debug formatter
    print!("{:?}", "Rust");

    // Print a newline character to move to the next line
    print!("\n");
}

/// Demonstrates the use of the `println!` macro with the debug formatter `:?`.
// The `println!` macro is used to print to the console with a newline at the end.
// The `:?` formatter inside the curly braces is used for debugging purposes.
// It prints the value in a format that is useful for debugging, known as the "Debug" format.
// The `{:?}` marker tells `println!` to use the `Debug` trait for the value provided.
fn print_with_debug_formatter() {
    println!("{:?}", "Rust is a systems programming language");
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
