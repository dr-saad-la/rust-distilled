<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
  Advanced Rust Project Structure 
</div>

As you progress in your Rust journey, understanding how to structure more complex projects becomes essential. In this section, we’ll delve into advanced concepts, including libraries and testing, that will help you build robust and maintainable Rust applications. We'll explore how to organize your code into libraries for reusability and how to set up comprehensive test suites to ensure the reliability of your code. By mastering these concepts, you’ll be well-equipped to tackle larger projects with confidence.

### Project Structure

A well-organized directory layout is crucial for managing the code effectively in a Rust project. Here’s a typical structure that balances simplicity and scalability:


```text
my_project/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── module1.rs
│   ├── module2.rs
│   ├── module2/
│   │   ├── submodule1.rs
│   │   └── submodule2.rs
├── tests/
│   └── integration_test.rs
├── examples/
│   └── example.rs
├── target/
├── Cargo.toml
└── Cargo.lock
```

### Directory Breakdown

- **`src/`**: Contains the main source code of your project.
  - **`main.rs`**: The entry point for a binary application. This file contains the `main` function.
  - **`lib.rs`**: The entry point for a library, containing reusable code that can be shared across multiple binaries or external projects.
  - **`module1.rs`**: A file representing `module1`, encapsulating specific functionality.
  - **`module2.rs`**: The main file for `module2`, declaring its submodules.
  - **`module2/`**: A directory containing submodules related to `module2`.
    - **`submodule1.rs`**: A submodule file within `module2`.
    - **`submodule2.rs`**: Another submodule file within `module2`.

- **`tests/`**: Contains integration tests that test the public API of your library or application.
  - **`integration_test.rs`**: An example integration test file.

- **`examples/`**: Contains example programs that demonstrate how to use your library.
  - **`example.rs`**: An example file showing usage patterns and functionalities.

- **`target/`**: The directory where compiled artifacts are stored. This directory is created by Cargo and includes the results of the build process.

- **`Cargo.toml`**: The configuration file for your project, specifying dependencies, project metadata, and build settings.

- **`Cargo.lock`**: A lock file that ensures consistency of dependency versions, generated automatically by Cargo.


## Organizing Code

1. **Modules and Files**: Rust uses modules to encapsulate code into namespaces. A module can be a file or a directory. If a directory has submodules, you should use the `directory_name.rs` instead of using `mod.rs` (the old approach) in each submodule directory, since the `directory_name.rs` approach is the recommended and the future Rust projects.


2. Declaring Modules: To declare the modules and submodule you can use the  `mod` keyword , for example:

```rust
// In lib.rs or main.rs
mod module1;
mod module2;
```

3. Using Modules: Import modules with the `use` keyword.

```rust
use module1::function1;
use module2::submodule1::function2;
```

4. Defining Modules in Files: Each module can be defined in its own file:

  - module1.rs:

```rust
pub fn function1() {
    println!("Function in module1");
}
```
  - module2.rs:

```rust
pub mod submodule1;
pub mod submodule2;

pub fn function2() {
    println!("Function in module2");
}
```

  - module2/submodule1.rs:

```rust
pub fn function2() {
    println!("Function in submodule1");
}
```

5. Using lib.rs and main.rs
  - lib.rs: Contains library code, which can be shared across multiple binaries or external projects.

```rust
pub mod module1;
pub mod module2;

pub fn public_function() {
    println!("Public function in the library");
}
```
  - main.rs: Entry point for binary applications. It can use functions from lib.rs.

```rust
fn main() {
    println!("Hello, world!");
    my_project::public_function();
}
```

## Writing Tests in Rust Applications

Rust encourages test-driven development, providing robust tools to ensure your code behaves as expected. Organizing tests effectively can help maintain code quality and catch potential bugs early in the development process.

### Types of Tests

1. **Unit Tests**: 
   - These are small tests focused on individual components or functions. 
   - Located within the same file as the code they test, unit tests ensure each function performs correctly in isolation.

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_add() {
           assert_eq!(add(2, 3), 5);
       }
   }
  ```
- **`#[cfg(test)]`**: This attribute indicates that the module should only be compiled when running tests.
- **`#[test]`**: Marks a function as a test case.
- **`assert_eq!`**: A macro that asserts two values are equal.

### Integration Tests

- These tests evaluate the behavior of your code when different modules or components work together.
- Located in the `tests` directory, integration tests focus on the public API of your library or application.

**Example Structure**:
```text
my_project/
├── src/
│   ├── lib.rs
├── tests/
│   ├── integration_test.rs
```

### Example Code:

```rust
// tests/integration_test.rs
use my_project::calculator;

#[test]
fn test_integration() {
    assert_eq!(calculator::add::add(2, 3), 5);
    assert_eq!(calculator::subtract::subtract(5, 3), 2);
}
```

Here, `use my_project::calculator`; imports the modules being tested.

## Writing Effective Tests

 - Be Descriptive: Use meaningful names for test functions to describe what the test verifies.
 - Test Edge Cases: Consider unusual or extreme inputs to ensure your code handles them gracefully.
 - Keep Tests Isolated: Tests should not depend on each other. Each test should run independently.


## Running Tests
To run your tests, use the following command in your terminal:

```bash
cargo test
```

This command compiles your code and runs all the tests in your project, displaying the results in the terminal.

## Test Output

- Passed: Indicates that the test ran successfully.
- Failed: Indicates that the test did not pass, providing details about the failure for debugging.

## Best Practices

- Keep Functions Small: Ensure each function has a single responsibility.
- Use Result and Option for Error Handling: Leverage Rust’s powerful enums for handling errors gracefully.
- Write Tests: Maintain unit and integration tests to ensure code quality.
- Organize Code Logically: Group related functionality in modules.
- Use Documentation Comments: Use /// for public functions and modules to generate documentation with cargo doc.

## Running the Project

To execute your Rust project and see it in action, use the following command in your terminal:

```bash
cargo run
```

This command compiles your code and runs the executable, allowing you to interact with your application. It’s a quick way to test changes and verify functionality.

## Building Documentation

Rust provides a powerful documentation generation tool that creates comprehensive and easily navigable documentation for your project. To generate and view the documentation, use:

```bash
cargo doc --open
```

This command compiles the documentation from comments and metadata in your code, then opens it in your default web browser. It’s an excellent way to ensure that your code is well-documented and accessible to other developers.

## Creating a Rust Application Walkthrough

### Creating the Application Using Cargo

To create your Rust application using Cargo, follow these steps:

1. **Open your terminal** and navigate to the directory where you want to create your project.

2. **Run the following command** to create a new Rust project:

   ```bash
   cargo new --lib advanced_calculator
   ```
   
3. Navigate into the project directory:

```bash
cd advanced_calculator
```

4. Create the necessary files and directories:

```bash
mkdir src/calculator
touch src/calculator.rs
touch src/calculator/add.rs
touch src/calculator/subtract.rs
touch src/calculator/multiply.rs
touch src/calculator/divide.rs
mkdir tests
touch tests/integration_test.rs
```

5. Open the project in your favorite code editor and add the appropriate code to each file as described in the project structure.





## Writing Code for Each Module

- src/lib.rs

```rust
pub mod calculator;
```

- src/calculator.rs

```rust
pub mod add;
pub mod subtract;
pub mod multiply;
pub mod divide;
```

- src/calculator/add.rs

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
- src/calculator/subtract.rs

```rust
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

- src/calculator/multiply.rs

```rust
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

- src/calculator/divide.rs

```rust
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

- Integration Tests: tests/integration_test.rs

```rust
use advanced_calculator::calculator;

#[test]
fn test_add() {
    assert_eq!(calculator::add::add(10, 5), 15);
    assert_eq!(calculator::add::add(0, 0), 0);
}

#[test]
fn test_subtract() {
    assert_eq!(calculator::subtract::subtract(10, 5), 5);
    assert_eq!(calculator::subtract::subtract(5, 10), -5);
}

#[test]
fn test_multiply() {
    assert_eq!(calculator::multiply::multiply(10, 5), 50);
    assert_eq!(calculator::multiply::multiply(0, 5), 0);
}

#[test]
fn test_divide() {
    assert_eq!(calculator::divide::divide(10, 5), Some(2));
    assert_eq!(calculator::divide::divide(10, 0), None); // Tests division by zero
}
```

**Running the Project**

To execute your Rust project and see it in action, use the following command in your terminal:

```bash
cargo run
```

**Running the Tests**
To run your tests, use the following command in your terminal:

```bash
cargo test
```

**Building Documentation**

Build the documentation for your application:

```bash
cargo doc --open
```

## Summary

In this section, we explored the structure of advanced Rust projects, focusing on modularity, organization, and testing. We discussed how to create a well-organized directory layout using Cargo, including the use of libraries and submodules. We demonstrated how to write unit and integration tests to ensure code quality and reliability. Additionally, we covered the use of Cargo commands to run the application and generate documentation, highlighting the importance of a clean, maintainable, and scalable project structure in Rust development. 
