//================================================================================
//      By: Dr. Saad Laouadi
//      Course: Rust Distilled
//      Lesson: Comments in Rust
//
//     © copy-right: Dr. Saad Laouadi
//================================================================================

// Comments in Rust:
// =================

/* Comments follow the general C++ style of line:
 1. Line comments: (//)
 2. Multi-line comments of block (/* ... */) comment forms.
 3. Nested block comments are supported as well
*/

// This is a single-line comment in Rust. It's useful for short, inline comments.

/*
 * This is a multi-line comment in Rust.
 * It can span multiple lines and is useful for larger blocks of comments.
*/

// A function that adds two integers and returns their sum
fn add(a: i32, b: i32) -> i32 {
    // Adding two numbers
    a + b
}

fn main() {
    // This statement is used only to print a banner to stdout
    println!("{}", "*".repeat(52));
    // Calling the add function with arguments 5 and 3
    let result = add(5, 3);

    // Printing the result to the console
    println!("The result of adding 5 and 3 is: {}", result); // The result of adding 5 and 3 is: 8
    println!("{}", "*".repeat(52));
    /*
     * You can use multi-line comments to temporarily disable a block of code.
     * Uncommenting the following block will cause the program to print "This is a multi-line comment example."
     */

    /*
    println!("This is a multi-line comment example.");
    */
}
