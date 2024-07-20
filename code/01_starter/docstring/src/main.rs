//================================================================================
//      By: Dr. Saad Laouadi
//      Course: Rust Distilled
//      Lesson: Documenting Code in Rust
//
//     © copy-right: Dr. Saad Laouadi
//================================================================================

// Documentation in Rust
// =====================

// Outer Docstring
// ===============

/**
 * This is a documentation comment in Rust.
 * Documentation comments are used to generate API documentation.
 * They start with `///` and can be used with markdown formatting.
*/

// To see the documentation run this command:
// ```cargo doc --open```

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
fn add(a: i32, b: i32) -> i32 {
    // Adding two numbers
    a + b
}

//
fn main() {
    // This statement is used only to print a banner to stdout
    println!("{}", "*".repeat(52));
    // Calling the add function with arguments 5 and 3
    let result = add(5, 3);

    // Printing the result to the console
    println!("The result of adding 5 and 3 is: {}", result); // The result of adding 5 and 3 is: 8
    println!("{}", "*".repeat(52));

    // This attribute to silence the compiler warning about unused inline documentation
    #[allow(unused_doc_comments)]
    /// Inline documentation comment example
    /// Here we demonstrate how to use documentation comments inline.
    /// This can be useful for explaining specific parts of your code in detail.
    ///
    /// ```rust
    /// let example = "This is a documentation comment";
    /// println!("{}", example);
    /// ```
    let _inline_doc_example = "Documentation comments can be used inline as well.";
}
