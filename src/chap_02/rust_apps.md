<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Structuring Rust Application
</div>

## Introduction

Structuring a Rust application involves organizing your code to enhance clarity, maintainability, and scalability. This section covers the basics of Rust modules and crates, including key terminology, creating a Rust application using Cargo, understanding how to import modules, and the final folder structure of a well-organized application.

### Key Concepts

- **Modules**: Logical groupings of code within a project, promoting encapsulation and reuse.  Modules can contain definitions for functions, structs, enums, constants, and other modules.
- **Crates**: Packages of Rust code that can be compiled into libraries or executables. Crates are the primary unit of compilation in Rust.
- **Cargo**: Rust’s package manager and build system, streamlining project creation and dependency management.

By following these guidelines, you'll learn how to build a clean and efficient Rust application structure.

## Create a Rust Application Using Cargo
Cargo is the Rust package manager and build system. You can create a new Rust application using the following command:

```bash
cargo new app_name
```

Usually, you will have the project structure like this:

```text
my_app
├── Cargo.toml
└── src
    └── main.rs
```

## Understanding the Namespace

Rust modules provide a **namespace** to prevent name conflicts. When you define or import items within a module, they are part of that module's namespace.

## What is a Namespace?

- A **namespace** is a context that allows you to group identifiers (such as functions, structs, enums, constants, and other modules) to avoid naming conflicts. In Rust, namespaces are created using modules. Each module has its own **namespace**, which helps in organizing code and controlling the scope of identifiers.

To simply put, **namespaces** in Rust provide a way to organize and manage different scopes of code, helping to avoid naming conflicts and making code more modular and maintainable. 

## Defining Modules and Namespaces

In Rust, you can define a module using the `mod` keyword. Modules can be defined within a single file or across multiple files.

### Single File Module

Here is an example of defining and using modules within a single file:

```rust
// main.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math::add(5, 3);
    let difference = math::subtract(5, 3);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
```

### Using Namespaces

In Rust, the `use` keyword allows you to bring items from modules into the current namespace, making it easier to access functions, structs, and other module contents.

## Importing Rust Modules

Modules in Rust can be imported using the `mod` keyword for modules defined within the same file, or the `use` keyword for modules and items defined in other files or crates. This allows you to organize your code effectively and reuse functionality across different parts of your application.

### Importing Modules within the Same File

You can define and import modules within the same file using the `mod` keyword.

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math::add(5, 3);
    let difference = math::subtract(5, 3);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
```

## Importing Modules from Other Files

For larger projects, it's common to organize modules in separate files. You can import these modules using the mod and use keywords.

File Structure:

```text
my_app
├── Cargo.toml
└── src
    ├── main.rs
    ├── math.rs
    ├── math
    │   ├── add.rs
    │   └── subtract.rs
```

**Code Example:**

1. src/main.rs
    ```rust
    mod math;
    use math::{add, subtract};

    fn main() {
        let sum = add::add(5, 3);
        let difference = subtract::subtract(5, 3);
        println!("Sum: {}", sum);
        println!("Difference: {}", difference);
    }
    ```

2. src/math.rs

    ```rust
    pub mod add;
    pub mod subtract;
    src/math/add.rs

    rust
    Copy code
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

2. src/math/subtract.rs

```rust
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

**Using External Crates**
To use external crates, you add them to your Cargo.toml and import them with extern crate and use.

Cargo.toml

```toml
[dependencies]
rand = "0.8"
```

1. src/main.rs

```rust
extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..101);
    println!("Random number: {}", n);
}
```

### Bringing Items into Scope

Here’s how you can bring items into scope using the `use` keyword:

- **Importing Specific Functions**: You can selectively import functions from a module, making them directly accessible without the module prefix.

**Module Definition**

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
```

**Importing and Using Functions**

```rust
use math::{add, subtract};

fn main() {
    let sum = add(5, 3);
    let difference = subtract(5, 3);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
```

In the previous code:
 - `mod math`: Defines a module named math containing two public functions, add and subtract.
 - `use math::{add, subtract};`: Imports the add and subtract functions into the current scope, allowing you to call them directly without needing to prefix them with the module name.
 - **Function Calls**: The add and subtract functions are called directly within main, and their results are printed.
 
### Without Using `use`

If you don't use the `use` keyword, you must reference items with their full paths, including the module name. This approach can make the code more verbose, but it clearly indicates where each function or item originates.

**Example Without `use`**

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math::add(5, 3);
    let difference = math::subtract(5, 3);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
```

Notice that when we don't use the `use` to bring the module to the local namespace:
 - **Full Path Reference**: When calling add and subtract, you must prefix them with `math::`, the module name. This explicitly shows that these functions belong to the math module.
 - **Clarity vs. Verbosity**: While this method is clear and indicates the function’s origin, it can become cumbersome in larger projects where functions from the same module are called frequently.
 
> Using `use` can reduce verbosity and enhance code readability by simplifying how you reference items from modules. It is recommended to import modules at the top of your Rust files. However, in situations where different modules contain functions with the same names, you may need to use the fully qualified path to avoid ambiguity.

### Using Nested Modules

Rust allows you to create nested modules, organizing your code hierarchically. You can bring items from these nested modules into scope for easier access:

```rust
mod math {
    pub mod operations {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub fn divide(a: i32, b: i32) -> Option<i32> {
            if b == 0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
}

use math::operations::{multiply, divide};

fn main() {
    let product = multiply(5, 3);
    let quotient = divide(6, 2);
    match quotient {
        Some(q) => println!("Quotient: {}", q),
        None => println!("Cannot divide by zero"),
    }
    println!("Product: {}", product);
}
```
**Code Explained**

- **Nested Modules**: `math` contains a submodule `operations`, which defines `multiply` and `divide` functions.
- **Using `use`**: This imports `multiply` and `divide` from `math::operations`, allowing you to call them directly without the full path.
- **Function Usage**: The `main` function demonstrates calling `multiply` and `divide`. The `divide` function returns an `Option`, handling division by zero gracefully.
- **Output Handling**: The `match` statement checks the result of `divide`, printing a message if the division was successful or indicating an error if dividing by zero.

### Multi-File Module: Scenario 01

You can also split modules across multiple files for better organization. This is was the old way of organizing modules:

```text
my_app
├── Cargo.toml
└── src
    ├── main.rs
    ├── math
    │   ├── mod.rs
    │   ├── add.rs
    │   └── subtract.rs
```

```rust
mod math;

fn main() {
    let sum = math::add::add(5, 3);
    let difference = math::subtract::subtract(5, 3);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
```

- The **src/math/mod.rs:**

```rust
pub mod add;
pub mod subtract;
```

2. **src/math/add.rs:**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- **src/math/subtract.rs:**

```rust
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

### Multi-File Module: Using Directory Names as Namespaces

Instead of adding a `mod.rs` file in the folder like shown in the previous scenario, you can structure your modules by using the directory name as the namespace instead of relying on `mod.rs`, in other words you can add a file named `<folder_name>.rs` at the same level as the folder. This approach simplifies the directory structure and improves clarity, making it easier to understand the module hierarchy at a glance.

Note that this approach is becoming increasingly popular among Rust developers for several reasons:

#### Updated File Structure

```text
my_app
├── Cargo.toml
└── src
    ├── main.rs
    ├── math.rs
    ├── math
    │   ├── add.rs
    │   └── subtract.rs
```

## Why Use Directory Names as Namespaces?

1. **Simplicity and Clarity**: Using directory names instead of mod.rs files simplifies the directory structure and makes it clearer what each directory represents. Each directory directly maps to a module, reducing ambiguity.
2. **Easier Navigation**: Developers can easily navigate through the codebase because each directory name explicitly represents a module, and files within the directory represent submodules or related functionality.
3. **Avoids mod.rs Overload**: Using mod.rs for every module can lead to confusion, especially in larger projects where there may be multiple mod.rs files. Naming directories explicitly avoids this issue.
4. **Consistency with Other Languages**: This approach aligns more closely with conventions in other programming languages, making it more intuitive for developers coming from different backgrounds.
5. **Improved Readability**: The directory and file names serve as a natural documentation of the module hierarchy, improving overall readability and maintainability of the code.

```rust
mod math;

fn main() {
    let sum = math::add::add(5, 3);
    let difference = math::subtract::subtract(5, 3);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}
```

- The **src/math.rs:**

```rust
pub mod add;
pub mod subtract;
```

2. **src/math/add.rs:**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- **src/math/subtract.rs:**

```rust
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

By using directory names as namespaces, Rust application can be more maintainable and scalable . This approach enhances the clarity of your module structure, making your codebase easier to navigate and understand. As Rust continues to evolve, this practice is becoming a recommended way to organize larger projects efficiently.

## The Final Application Folder Structure

Here is an example of a more complex folder structure for a Rust application:

```text
my_app
├── Cargo.toml
└── src
    ├── main.rs
    ├── module1.rs
    |── module2.rs
    └── module2
    |    ___submodule1.rs
    |    ___submodule2.rs
```

In this structure:
  - **main.rs** is the entry point for the binary crate.
  - **module1.rs** is a module file.
  - **module2.rs** is another module, demonstrating the use of nested modules

## Practical Example: Application Structure

Here’s a typical structure for a simple calculator application:

```text
simple_calculator
├── Cargo.toml
└── src
    ├── main.rs
    ├── calculator.rs
    ├── calculator
    │   ├── add.rs
    │   ├── subtract.rs
    │   ├── multiply.rs
    │   └── divide.rs
```

### Understanding the Project Structure

1. The `calculator.rs` file serves an important role in the structure of your Rust application. By declaring submodules within this file, you inform the Rust compiler about the existence of additional modules within the `calculator` directory.

### How It Works

When you create a directory for modules, Rust needs a way to understand that this directory is more than just a folder—it contains related modules. The `calculator.rs` file acts as an entry point or a gateway, explicitly declaring each submodule with the `pub mod` keyword.

### Why It’s Important

- **Namespace Clarity**: By specifying submodules in `calculator.rs`, you create a clear namespace. This helps in organizing code logically, making it easier to navigate and maintain.
- **Compiler Guidance**: The Rust compiler relies on these declarations to locate and compile the corresponding Rust files within the directory. Without `calculator.rs`, the compiler wouldn’t know which files to include or how they relate to each other.
- **Modular Codebase**: This approach promotes modularity. Each submodule (e.g., `add.rs`, `subtract.rs`) contains specific functionality, allowing you to break down complex logic into manageable parts.

### Example

In `calculator.rs`, you might have:

```rust
pub mod add;
pub mod subtract;
pub mod multiply;
pub mod divide;
```

### The `calculator` Directory

The `calculator` directory contains the individual submodules that define specific arithmetic operations. Each operation (addition, subtraction, multiplication, division) is encapsulated in its own file, promoting modularity and clarity.

#### Structure

```text
calculator
├── add.rs
├── subtract.rs
├── multiply.rs
└── divide.rs
```

#### The Submodules

1. `add.rs`: This module handles addition.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- Purpose: Implements a simple function to add two integers.
- Visibility: The function is marked pub, making it accessible from outside the module.

2. `subtract.rs`: This module handles subtraction.

```rust
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

- Purpose: Implements a function to subtract the second integer from the first.
- Visibility: The pub keyword makes it available to other modules.

3. `multiply.rs`: This module handles multiplication.

```rust
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

- Purpose: Multiplies two integers and returns the result.
- Visibility: Public, allowing access from other parts of the program.

4. `divide.rs`: This module handles division.

```rust
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

- Purpose: Divides the first integer by the second. If the divisor is zero, it returns None to prevent a runtime error.
- Return Type: Uses `Option<i32>` to safely handle division by zero.
- Visibility: Public, so it can be used in the main application.

## The `main.rs` File

The `main.rs` file serves as the entry point of the application, orchestrating the use of different modules to perform calculations. It imports the `calculator` module and utilizes the arithmetic operations defined within it.

### Code Overview

```rust
mod calculator;

fn main() {
    let a = 10;
    let b = 5;

    let sum = calculator::add::add(a, b);
    let difference = calculator::subtract::subtract(a, b);
    let product = calculator::multiply::multiply(a, b);
    let quotient = calculator::divide::divide(a, b);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}
```

1. Module Declaration:
```rust
mod calculator;
```

This line declares the calculator module, making all the submodules available for use in the main function.

2. Main Function: The main function is where the program execution begins.

```rust
fn main() {
    // body
}
```

In the main function:

1. Variable Initialization:

```rust
let a = 10;
let b = 5;
```

Two integer variables, a and b, are initialized with values to demonstrate the calculator's functionality.

2. Using Calculator Functions:

```rust
let sum = calculator::add::add(a, b);
let difference = calculator::subtract::subtract(a, b);
let product = calculator::multiply::multiply(a, b);
let quotient = calculator::divide::divide(a, b);
```

Each arithmetic operation is performed using the functions from the respective submodules (add, subtract, multiply, divide). The results are stored in variables.

3. Output:

```rust
println!("Sum: {}", sum);
println!("Difference: {}", difference);
println!("Product: {}", product);
println!("Quotient: {}", quotient);
```

The results of the calculations are printed to the console. If b were zero, the division operation would handle it gracefully by returning None.

### The `Cargo.toml`

The `Cargo.toml` file is the configuration file for your Rust project. It contains metadata about your package, including its name, version, and dependencies. Here's an example for the simple calculator application:

```toml
[package]
name = "simple_calculator"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Explanation

- **`[package]`**: This section defines the package metadata.
  - **`name`**: The name of your project (`simple_calculator`).
  - **`version`**: The version of your application (`0.1.0`).
  - **`edition`**: Specifies the Rust edition you're using (`2021`), which includes the latest features and improvements.

- **`[dependencies]`**: This section lists the external crates your project depends on. In this case, there are no external dependencies, but you can add them as needed.


## Running the Application

To compile and run your Rust application, use the following command:

```bash
cargo run
```

This command will:
 - Compile the source code.
 - Execute the main function.
 - Display the results of the calculations in the console.

### Summary
- Namespace: A context that allows grouping identifiers to avoid naming conflicts.
- Module: In Rust, a module creates a namespace. Use the mod keyword to define a module.
- Single File Module: Modules can be defined and used within a single file.
- Multi-File Module: Modules can be split across multiple files for better organization.
- Importing Items: Use the use keyword to bring items from a module into the current namespace.
- Nested Modules: Modules can be nested, and items from nested modules can be brought into scope.

## Conclusion

Structuring a Rust application correctly is crucial for maintainability and scalability. By following best practices, organizing code into modules, and writing tests, you can create robust and efficient Rust applications. Keep your code modular, test thoroughly, and document your public interfaces for the best results.

## References

1. [In Rust, what is the purpose of a mod.rs file?](https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file)
2. [Module Source Filenames - Rust Reference](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames)


