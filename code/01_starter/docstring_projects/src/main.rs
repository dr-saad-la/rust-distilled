//================================================================================
//      By: Dr. Saad Laouadi
//      Course: Rust Distilled
//      Lesson: Documenting Rust Projects
//
//      © copy-right: Dr. Saad Laouadi
//================================================================================

//! This module provides utility functions for mathematical operations.
//! It includes basic arithmetic operations such as addition and multiplication.
//! This documentation is generated using `//!` syntax

/*!
 *
 * This module provides utility functions for mathematical operations.
 * It includes basic arithmetic operations such as addition and multiplication.
 * This is also an inner documentation but generated using the `/*! ... */` syntax
 *
 */

/*!
 * Generate Code Documentation in Rust
 * ===================================
 *
 * To generate the documentation, run the following command:
 *
 * ```
 * cargo doc --open
 * ```
 *
 * This command will compile your code and generate the documentation, then open it in your default web browser.
 *
 * Public and Private Functions
 * ============================
 *
 * In Rust, functions are private by default. A private function is indicated by a lock icon in the generated documentation.
 * To make a function accessible from outside its module, you need to mark it with the `pub` keyword.
 *
 * Example:
 *
 * ```rust
 * pub fn public_function() {
 *     // This function is public
 * }
 *
 * fn private_function() {
 *     // This function is private
 * }
 * ```
 *
 * The lock icon in the generated documentation signifies that the function is private and not accessible from outside the module.
 */

/// Adds two numbers and returns the result.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Example
///
/// ```
/// let result = add(5, 3);
/// println!("The result is {}", result); // The result is 8
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first and returns the result.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Example
///
/// ```
/// let result = subtract(5, 3);
/// println!("The result is {}", result); // The result is 2
/// ```
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers and returns the result.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Example
///
/// ```
/// let result = multiply(5, 3);
/// println!("The result is {}", result); // The result is 15
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// The main function serves as the entry point for the program.
/// It demonstrates the usage of the `add` function and prints results to the console.
///
/// # Examples
///
/// ```
/// main();
///
fn main() {
    println!("{}", "*".repeat(52));
    let result = add(5, 3);
    println!("The result of adding 5 and 3 is: {}", result); // The result is 8
    println!("{}", "*".repeat(52));

    let result_subtract = subtract(5, 3);
    println!("The result of subtracting 3 from 5 is: {}", result_subtract); // The result is 2
    println!("{}", "*".repeat(52));
}
