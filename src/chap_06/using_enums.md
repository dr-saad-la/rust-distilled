<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Using Enum Types 
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

## Using Enums in Rust

In previous section, we have covered how to create enum types in Rust, sing the `enum` keyword followed by a name and a set of variants. We have also peeked how to use them using the `EnumName::VariantName` syntax. In this section, we will explore how to effectively use them through a series of simple examples. We will cover basic usage, pattern matching, and practical applications, demonstrating the power and flexibility of enums.


## Using Enum Types with the `::` Syntax

In Rust, the `EnumName::VariantName` syntax is used to access the variants of an enum. This syntax allows you to refer to enum variants in a clear and concise way, both when defining variables and when matching against them. This section will demonstrate how to use the `EnumName::VariantName` syntax effectively.

### Defining and Using Enum Variants

When you define an enum, each variant is namespaced under the enum's name. To create an instance of an enum variant, you use the `::` syntax. Here is an example:

### Basic Usage

Let's start with a basic example, the cardinal directions  Enum type: 


```Rust
// Cardinal directions Example  
#[derive(Debug)]              // This to implement the debug trait to allows us to print enum variants
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    banner("*", 52, "Accessing Enum Variants");
    // Creating instances of enum variants using the `::` syntax
    let north = Direction::North;
    let east = Direction::East;
    
    println!("Direction: {:?}", north);
    println!("Direction: {:?}", east);
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                  Accessing Enum Variants               
    ****************************************************
    Direction: North
    Direction: East
    ****************************************************


## Using `match` with Enums in Rust

One of the most powerful features of Rust is its pattern matching capabilities, especially when used with enums. The `match` statement allows you to destructure and handle different enum variants in a concise and readable manner. We will focus onf using `match` with enums.

### Why Use `match` with Enums?

The `match` statement in Rust provides several advantages when working with enums:

- **Exhaustiveness Checking**: The Rust compiler ensures that all possible variants of an enum are handled. This reduces the risk of runtime errors and makes the code more robust.
- **Readability**: `match` statements clearly express how different cases are handled, making the code easier to read and understand.
- **Flexibility**: You can destructure enums and bind variables to their associated data, allowing for complex pattern matching.

#### Pattern Matchin Example: Traffic Light Enum Type

Let's consider a simple example where we have an enum representing different traffic lights states. We will use a match statement to match against the TrafficLight variants inside the match statement.


```Rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}

fn main() {
    banner("*", 52, "Using Match with Enum Type");
    let light = TrafficLight::Red;
    
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                 Using Match with Enum Type             
    ****************************************************
    Stop!
    ****************************************************


#### Example: Basic Enum

Here is another example using the cardinal directions and using `match` statement to access the `Direction` enum variants.


```Rust
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let dir = Direction::North;
    match dir {
        Direction::North => println!("Heading North!"),
        Direction::East => println!("Heading East!"),
        Direction::South => println!("Heading South!"),
        Direction::West => println!("Heading West!"),
    }
}
main();
```

    Heading North!


## Accessing Enum Variants in Functions

When working with enums, it is common to pass their variants as arguments to functions. The `::` syntax allows you to do this in a clear and concise manner. By specifying the enum type and its variant, you ensure that the function receives the correct type, and the intention of the code is immediately apparent.

#### Example: Passing Enum Variants to a Function

Consider an enum representing different types of coins. We will create a function that accepts a coin variant and returns its value in cents. By using the `::` syntax, we can clearly and explicitly pass the correct variant to the function.



```Rust
// Define an enum named Coin with four variants
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Define a function to determine the value of a coin
fn coin_value(coin: Coin) -> u8 {
    // Use a match statement to return the value based on the coin variant
    match coin {
        Coin::Penny => 1,      // If the coin is a Penny, return 1
        Coin::Nickel => 5,     // If the coin is a Nickel, return 5
        Coin::Dime => 10,      // If the coin is a Dime, return 10
        Coin::Quarter => 25,   // If the coin is a Quarter, return 25
    }
}

fn main() {
    banner("*", 52, "Using Enum Type in Functions");
    let my_coin = Coin::Quarter;
    println!("The value of my coin is: {} cents", coin_value(my_coin));
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                Using Enum Type in Functions            
    ****************************************************
    The value of my coin is: 25 cents
    ****************************************************


#### Code in Details

**Enum Definition**: 
- The `Coin` enum is defined with four variants: `Penny`, `Nickel`, `Dime`, and `Quarter`.

**Function Definition**
 - The `coin_value` function takes a `Coin` enum as an argument and returns the value of the coin in cents. The function uses a `match` statement to handle each variant and return the corresponding value.

**Accessing Variants**
 - In the `main` function, an instance of the `Coin` enum is created using the `EnumName::VariantName` syntax (`Coin::Quarter`). This instance is then passed to the `coin_value` function, which matches the variant and returns its value.


## Practical Examples

Enums are versatile and can be used in various practical applications. Here are a few common scenarios:

### Example: Error Handling
Enums are commonly used for error handling in Rust, particularly with the Result type.


```Rust
// Define an enum named FileError with two variants
enum FileError {
    NotFound,
    PermissionDenied,
}

// Define a function to simulate opening a file
// The function returns a Result type with &str for success and FileError for error
fn open_file(filename: &str) -> Result<&str, FileError> {
    if filename == "secret.txt" {
        // If the filename is "secret.txt", return a PermissionDenied error
        Err(FileError::PermissionDenied)
    } else if filename == "missing.txt" {
        // If the filename is "missing.txt", return a NotFound error
        Err(FileError::NotFound)
    } else {
        // Otherwise, return success with a message
        Ok("File opened successfully!")
    }
}

fn main() {
    // Define a filename to open
    let filename = "secret.txt";
    // Use a match statement to handle the Result from open_file function
    match open_file(filename) {
        // If the file is opened successfully, print the success message
        Ok(message) => println!("{}", message),
        // If the file is not found, print an error message
        Err(FileError::NotFound) => println!("Error: File not found."),
        // If permission is denied, print an error message
        Err(FileError::PermissionDenied) => println!("Error: Permission denied."),
    }
}

main();
```

    Error: Permission denied.

## Conclusion

In this section, we explored the essential aspects of using enums in Rust, demonstrating their versatility through practical examples.

**Enum Definition**:
  - Defined enum types using the `enum` keyword and accessed variants with the `EnumName::VariantName` syntax.
**Pattern Matching with Enums**:
  - Utilized the `match` statement for exhaustive and readable handling of enum variants.
**Accessing Enum Variants in Functions**:
  - Passed enum variants to functions using the `::` syntax, ensuring type safety and clarity.
**Practical Applications**:
  - Showcased enums in real-world scenarios such as error handling.
