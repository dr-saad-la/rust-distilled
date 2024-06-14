<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
  Constant Variables in Rust
</div>


```Rust
use std::any::type_name;

pub fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
```


Constants are immutable values that are known at **compile time** and do not change throughout the execution of a program. They are a fundamental part of many programming languages, and Rust provides robust support for them. This chapter will cover the details of using constants in Rust, including their syntax, scope, and some practical examples.

## What are Constants?

**Constants** in Rust are values that are bound to a name and are not allowed to change. They are similar to variables, but with some key differences:

1. **Immutability**: Constants are always immutable. Once a value is assigned to a constant, it cannot be changed.
2. **Type Annotation**: Unlike variables, **constants require an explicit type annotation**.
3. **Scope**: Constants can be declared in any scope, including the global scope, and are accessible from anywhere within their scope.
4. **Compile-Time Evaluation**: Constants must be evaluated at compile time, meaning the value of a constant must be known and fixed during compilation.

### Naming Conventions

- By convention, constant names in Rust are written in SCREAMING_SNAKE_CASE, meaning all uppercase letters with underscores separating words.

### Declaring Constants

- To declare a constant in Rust, use:
    - The `const` keyword, followed by the name of the constant, a colon, the type of the constant
    - Assign a value to this constant using the equals sign. 

```rust
const MAX_POINTS: u32 = 100_000;
```


```Rust
fn main(){
    banner("=", 62, "Contant Variable in Rust");
    const MAX_POINTS: u32 = 100_000;
    
    println!("The constant variable is: {}", MAX_POINTS);
    
    // Check the type of the constant
    println!("The constant variable is: {}", type_of(MAX_POINTS));
}

main();
```

    
    ==============================================================
                       Contant Variable in Rust                   
    ==============================================================
    The constant variable is: 100000
    The constant variable is: u32


In this example:

- `const` is the keyword used to declare a constant.
- `MAX_POINTS` is the name of the constant.
- `u32` is the type of the constant.
- `100_000` is the value assigned to the constant.

## Canstants are Immutable

If you attempt to modify a constant you will ge a compilation error;

```rust
fn main() {
    const MAX_POINTS: u32 = 100_000;
    // Attempting to modify a constant will result in a compilation error
    // Uncommenting the next line will cause an error
    // MAX_POINTS = 200_000;
}
```

The error will look similar to this: 

```text
[E0070] Error: invalid left-hand side of assignment
   ╭─[command_7:1:1]
   │
 5 │     MAX_POINTS = 200_000;
   │     ─────┬──── ┬  
   │          ╰──────── cannot assign to this expression
   │                │  
   │                ╰── error: invalid left-hand side of assignment

```

## Constants vs. Immutable Variables

- It's important to understand the difference between constants and immutable variables in Rust:
    - Constants: Declared with `const`, must have a type annotation, and are evaluated at compile time.
    - Immutable Variables: Declared with `let`, type annotation is optional (inferred by default), and are evaluated at **runtime**.


```Rust
fn main() {
    const CONSTANT_VALUE: i32 = 10;
    let variable_value = 20;

    println!("Constant: {}", CONSTANT_VALUE);
    println!("Variable: {}", variable_value);
}
main(); 
```

    Constant: 10
    Variable: 20


- In this example, CONSTANT_VALUE is a compile-time constant, whereas variable_value is a runtime variable. 

## Reassignment of constants is not Allowed

Once a constant is defined, it cannot be reassigned or redefined within the same scope. Attempting to do so will result in a compilation error. Constants are designed to represent values that are fixed for the duration of the program, and their immutability is enforced by the compiler.

Here is an example: 

```rust
fn main() {
    const MAX_POINTS: u32 = 100_000;
    // reassigning a const variable
    const MAX_POINTS: u32 = 200_000;
}
main();
```

will raise this error:

```text
[[E0428] Error: the name `MAX_POINTS` is defined multiple times
   ╭─[command_12:1:1]
   │
 2 │     const MAX_POINTS: u32 = 100_000;
   │     ────────────────┬───────────────  
   │                     ╰───────────────── previous definition of the value `MAX_POINTS` here
   │ 
 4 │     const MAX_POINTS: u32 = 200_000;
   │     ────────────────┬───────────────  
   │                     ╰───────────────── `MAX_POINTS` redefined here
───╯
```

While it is permissible to shadow variables of other data types. Variable shadowing, this will be discussed in a later chapter, allows you to declare a new variable with the same name as a previous variable, effectively creating a new binding. This is commonly used to update the value of a variable without mutating it directly.

Here is an example demonstrating the concept:


```Rust
fn main() {
    banner("=", 62, "Reassigning Integer Variable");

    // Initial declaration of an integer variable
    let x: i32 = 10;
    println!("x: {}", x);

    // Reassign x by shadowing the previous variable
    let x: i32 = 20;
    println!("x now is: {}", x);
}

// Call the main function to execute the program
main();
```

    
    ==============================================================
                     Reassigning Integer Variable                 
    ==============================================================
    x: 10
    x now is: 20


### Scope and Accessibility
Constants can be declared in various scopes, including:

- **Global Scope**: Accessible from anywhere in the code.
- **Function Scope**: Accessible only within the function where they are declared.

### Example of Global Scope Constant


```Rust
// Declare a global constant
const GLOBAL_CONST: &str = "I am accessible everywhere";

fn main() {
    banner("=", 62, "Print the global constant from the main function");
    
    println!("From main: {}", GLOBAL_CONST);
    // Call another function that also accesses the global constant
    another_function();
}

// Define another function that accesses the global constant
fn another_function() {
    banner("=", 62, "Print the global constant from from another function");
    println!("From another_function: {}", GLOBAL_CONST);
}

main();
```

    
    ==============================================================
           Print the global constant from the main function       
    ==============================================================
    From main: I am accessible everywhere
    
    ==============================================================
         Print the global constant from from another function     
    ==============================================================
    From another_function: I am accessible everywhere


Example of Function Scope Constant


```Rust
fn main() {
    banner("=", 62, "Local Constant Example in Rust");

    // Declare a local constant
    const LOCAL_CONST: i32 = 42;
    
    println!("The value of the local constant is: {}", LOCAL_CONST);
}

main();
```

    
    ==============================================================
                    Local Constant Example in Rust                
    ==============================================================
    The value of the local constant is: 42


### Constants and Functions

- Constants in Rust cannot be the result of a function call or any runtime computation. They must be simple values or expressions that the compiler can evaluate at compile time.

#### Invalid Example

```rust
const INVALID_CONST: i32 = calculate_value(); // This will cause a compile-time error

fn calculate_value() -> i32 {
    42
}
```

#### Valid Example

```rust
const VALID_CONST: i32 = 42 * 2; // This is allowed because it's a compile-time expression
```

### Practical Use Cases

Constants are particularly useful for values that are used throughout your program and do not change, such as mathematical constants, configuration values, or any fixed data that needs to be globally accessible.

Example with Configuration Values


```Rust
// Declare global constants for server configuration
const SERVER_ADDRESS: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8080;

fn main() {
    banner("=", 62, "Server Configuration Example in Rust");

    // Print the server address and port
    println!("Server running at {}:{}", SERVER_ADDRESS, SERVER_PORT);
}

main();
```

    
    ==============================================================
                 Server Configuration Example in Rust             
    ==============================================================
    Server running at 127.0.0.1:8080


## Summary

Constants in Rust provide a way to define immutable values that are known at compile time. They are essential for creating safe and predictable programs, as they ensure certain values remain constant throughout the program's execution.
