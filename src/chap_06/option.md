<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    The Option Enum in Rust
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

In Rust, the `Option` type is a powerful and widely used enum that allows for the representation of optional (nullable) values. It helps in handling situations where a value might or might not be present, providing a safer alternative to null pointers that are common in other languages. By using Option, Rust ensures that absence of values is handled explicitly, thus preventing many types of runtime errors.

## Definition of `Option`

The `Option` enum is a generic type defined in Rust's standard library. It is a powerful tool for representing optional values, encapsulating the concept of a value that may or may not be present.

The `Option` enum is defined as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Key Components

#### `Option<T>`

- A generic enum that can be used with any type `T`. The `Option` type is parameterized by a type `T`, meaning it can hold a value of any type.
- This generic nature allows `Option` to be highly versatile and applicable in a wide range of scenarios.

#### `Some(T)`

- Represents an optional value of type `T`. When an `Option` is `Some`, it contains a value of type `T`.
- This variant is used when there is a value present.

#### `None`

- Represents the absence of a value. When an `Option` is `None`, it signifies that there is no value.
- This variant is used to indicate the absence of a value.


### Why Use Option

Using Option allows Rust to enforce that you handle cases where a value might be absent, preventing runtime errors related to null values. This is crucial for writing robust and error-free code. The compiler will check that you handle both Some and None cases, thus avoiding null pointer exceptions and other common errors related to missing values.

### Use Cases for Option

1. Optional Parameters: Use Option for function parameters that are optional.
2. Return Values: Use Option for return values that may or may not be present.
3. Configuration: Use Option for configuration settings that might be set or unset.
4. Handling Missing Data: Use Option to represent missing data in data structures.

### Basic Usage

**Declaring an Option**: You can declare an Option variable with either a value (`Some`) or without a value (`None`).

```rust
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;
```

The compiler is smart to infer the data type, so we 

### Pattern Matching with Option
One of the most common ways to handle `Option` values is through pattern matching. This allows you to specify different behaviors for the `Some` and `None` cases.

Here is a simple example to use Option enum.


```Rust
fn main() {
    banner("*", 52, "Using Enum Option");
    let some_number: Option<i32> = Some(5);
    let some_string: Option<String> = Some(String::from("Hello, Rust!"));
    let absent_number: Option<i32> = None;
    
    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
    
    println!("{}", "*".repeat(52));
}

main();    
```

    
    ****************************************************
                     Using Enum Option                  
    ****************************************************
    Some(5)
    Some("Hello, Rust!")
    None
    ****************************************************


### Using Type Inference with Option Enum
The compiler can infer types at compile time, allowing for type inference with the `Some` variant because it contains a value from which the type can be inferred. However, this is not possible with the `None` variant alone, as it does not provide any value to infer the type from. In such cases, you need to explicitly specify the type.


```Rust
fn main() {
    banner("*", 52, "Using Enum Option");

    // Type inference with `Some` variant
    let some_number = Some(5); // The compiler infers this as Option<i32>
    let some_string = Some(String::from("Hello, Rust!")); // The compiler infers this as Option<String>

    // Explicit type annotation is necessary for `None` variant
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);

    println!("{}", "*".repeat(52));
}

main(); 
```

    
    ****************************************************
                     Using Enum Option                  
    ****************************************************
    Some(5)
    Some("Hello, Rust!")
    None
    ****************************************************


## Introduction to Using Option in Functions

The `Option` enum is commonly used in Rust functions to handle cases where a value may or may not be present. By leveraging `Option`, functions can return or accept optional values in a clear and type-safe manner. Here's an example demonstrating how to use `Option` in functions  where an integer value might be present or absent:


```Rust
fn describe_number(number: Option<i32>) {
    match number {
        Some(n) => println!("The number is {}", n),
        None => println!("There is no number"),
    }
}

fn main() {
    banner("*", 52, "Using Enum Option");
    
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    describe_number(some_number); 
    describe_number(no_number);   
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                     Using Enum Option                  
    ****************************************************
    The number is 5
    There is no number
    ****************************************************


## Practical Examples

### Division Example
In this practical example we write a simple function that divides two numbers and returns an Option to handle division by zero:


```Rust
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    banner("*", 52, "Practical Example");
    
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }

    let result = divide(10.0, 0.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }
    
    println!("{}", "*".repeat(52));
}
main();    
```

    
    ****************************************************
                     Practical Example                  
    ****************************************************
    Result: 5
    Cannot divide by zero
    ****************************************************


#### Searching Example
The following example shows a use case of searching for index in an array:


```Rust
fn find_in_array(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    banner("*", 52, "Finding Index in an Array");
    let numbers = [1, 2, 3, 4, 5];
    match find_in_array(&numbers, 3) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                 Finding Index in an Array              
    ****************************************************
    Found at index: 2
    ****************************************************


### Code in Details

1. **Function Definition**:
    - `find_in_array` takes a slice of integers (`&[i32]`) and a target integer (`i32`). It returns an `Option<usize>`, indicating the index of the target if found.

2. **Returning `Some`**
    - If the target value is found in the array, the function returns `Some(index)`, where `index` is the position of the target in the array.

3. **Returning `None`**
    - If the target value is not found, the function returns `None`, indicating the absence of the target value.

4. **Using `Option` in `main`**
    - In the `main` function, the result of `find_in_array` is matched. If it is `Some(index)`, it prints the index. If it is `None`, it prints "Not found".

### Summary

In this section, we explored the `Option` enum, a fundamental feature in Rust for representing optional values. We demonstrated how `Option` provides a safe and explicit way to handle scenarios where values may or may not be present, significantly reducing the risk of runtime errors. By enforcing the handling of both `Some` and `None` cases, Rust ensures robust and reliable code.

We discussed various practical use cases for `Option`, such as optional parameters, return values, and handling missing data. Through examples, we illustrated the basic usage of `Option`, including pattern matching and type inference. Additionally, we showed how `Option` can be used effectively in functions to enhance type safety and code clarity.

