<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Booleans (Logical Values): bool type in Rust
</div>

This next code is used format the code output and the variables type:
```Rust
use std::any::type_name;

// Function to create a formatted banner
pub fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
```
> If you want to learn from the example, you may check [the comprehensive example](#comprehensive-example)

> 
## Introduction

- Booleans are fundamental data types in Rust that represent truth values: `true` or `false`. 
- The bool type is essential for controlling flow in programs through conditional statements and logical operations.

### Declaring and Using bool

- Using booleans in Rust is simple. You can directly assign `true` or `false` (all lowercase)to a variable of type `bool`.
- We can use `bool` explicitly to declare a boolean variable.

```rust
let b1: bool = true;
let b2: bool = false;
```

### Example:


```Rust
fn main() {
    banner("=", 62, "Boolean Types in Rust" );
    let is_tall: bool = true;
    let is_short: bool = false;

    println!("Is Rust awesome? {}", is_tall);
    println!("Is it raining? {}", is_short);
}
main();
```

    
    ==============================================================
                        Boolean Types in Rust                     
    ==============================================================
    Is Rust awesome? true
    Is it raining? false


- Let us enhance upon the previous example, so we can check the type and memory size of boolean variables: 


```Rust
fn main() {
    banner("=", 62, "Boolean Types in Rust, Types and Size in Memory" );
    let is_tall: bool = true;
    let is_short: bool = false;

    println!("int8: {:<15} type: {:<10} size: {} bytes",
        is_tall, type_of(is_tall), std::mem::size_of::<bool>());
}
main();
```

    ==============================================================
    Boolean Types in Rust, Types and Size in Memory
    ==============================================================
    int8: true            type: bool       size: 1 bytes


- Booleans has only one byte. 

### Common Operations with `bool`

- Booleans support standard logical operations that allow you to perform logical comparisons and control the flow of your program. These operations include:

    - `AND` (&&) (double ampersand characters),
    - `OR` (||) (double pipe characters).
    - and `NOT` (!) (Exclamation point).
    
-  Below are descriptions and examples of these operations:

#### Logical AND (&&)
- The AND operation returns `true` only if both operands are `true`. It is represented by the double ampersand characters (`&&`).

#### Logical OR (||)

- The OR operation returns true if at least one of the operands is true. It is represented by the double pipe characters (||).

#### Logical NOT (!)
- The NOT operation inverts the boolean value. It is represented by the exclamation point (!).

#### Example


```Rust
fn main() {
    banner("=", 62, "Boolean Operations");
    let a = true;
    let b = false;

    
    // Demonstrating boolean operations
    let are_both_true: bool = a && b; // AND operation
    let is_either_true: bool = a || b; // OR operation
    let is_not_a: bool = !a; // NOT operation
    let is_not_b: bool = !b;
    
    // Logical AND
    println!("a AND a is: {:<10} {}", "", a && a);
    println!("a AND b is: {:<10} {}", "", a && b);
    println!("b AND a is: {:<10} {}", "", b && a);
    println!("b AND b is: {:<10} {}", "", b && b);

    // Logical OR
    println!("a OR a is:  {:<10} {}", "", a || a);
    println!("a OR b is:  {:<10} {}", "", a || b);
    println!("b OR a is:  {:<10} {}", "", b || a);
    println!("b OR b is:  {:<10} {}", "", b || b);

    // Logical NOT
    println!("NOT a is :  {:<10} {}", "", !a);
    println!("NOT b is :  {:<10} {}", "", !b);
}
main();
```

    
    ==============================================================
                          Boolean Operations                      
    ==============================================================
    a AND a is:            true
    a AND b is:            false
    b AND a is:            false
    b AND b is:            false
    a OR a is:             true
    a OR b is:             true
    b OR a is:             true
    b OR b is:             false
    NOT a is :             false
    NOT b is :             true


## Combining Logical Operations

- You can combine these logical operations to form more complex expressions. Here's an example that combines AND, OR, and NOT operations:


```Rust
fn main() {
    banner("=", 62, "Combining Logical Operations");
    
    let has_umbrella = true;
    let is_raining = false;
    let is_sunny = true;

    // Complex expression combining AND, OR, and NOT
    let is_prepared = (is_raining && has_umbrella) || (!is_raining && is_sunny);

    println!("Am I prepared for the weather? {}", is_prepared); // Prints: true
}
main();
```

    
    ==============================================================
                     Combining Logical Operations                 
    ==============================================================
    Am I prepared for the weather? true


- In the previous example we used complex combination of booleans. The parentheses in this case are essential, so understanding the context of the project determines the place where to put them; misplacement of parentheses will lead to erroneous results:

    - `(is_raining && has_umbrella)`: Checks if it is raining and you have an umbrella.
    - `(!is_raining && is_sunny)`: Checks if it is not raining and it is sunny.
    - The overall expression `(is_raining && has_umbrella) || (!is_raining && is_sunny)`: Checks if either condition is true, meaning you are prepared for the weather.

## Casting Booleans in Rust

- Booleans (bool) represent truth values and can only be `true` or `false`. Rust provides strong type safety, and explicit conversions between types are encouraged to avoid unintended behavior. While you cannot directly cast booleans to integers using the `as` keyword, Rust allows for casting a bool to an integer type with specific semantics:
    - `true` is cast to 1.
    - `false` is cast to 0.

- This is useful in scenarios where you need to convert logical conditions into numeric representations, such as for use in arithmetic operations or when interfacing with systems that expect numeric values.


```Rust
fn main(){
    banner("=", 62, "Casting Booleans");
    
    let b1: bool = true;
    let b2: bool = false;

    // Casting booleans to integers using the `as` keyword
    let b1_as_int: i32 = b1 as i32;
    let b2_as_int: i32 = b2 as i32;

    println!("b1 (true) as integer: {}", b1_as_int); // Expected output: 1
    println!("b2 (false) as integer: {}", b2_as_int); // Expected output: 0
}
main();
```

    
    ==============================================================
                           Casting Booleans                       
    ==============================================================
    b1 (true) as integer: 1
    b2 (false) as integer: 0


## Control Flow with bool

- Booleans are often used in control flow statements like `if`, `else if`, and `else`. Control flow will be discussed in a later chapter. 

### Example:


```Rust
fn main() {
    let is_sunny = true;

    if is_sunny {
        println!("It's sunny outside!");
    } else {
        println!("It's not sunny outside.");
    }
}
main();
```

    It's sunny outside!



```Rust
fn main() {
    banner("=", 62, "Boolean in Flow-control");
    
    let is_rust_fun: bool = true;
    let is_learning_easy: bool = false;
    
    if is_rust_fun {
        println!("Rust is fun!");
    } else {
        println!("Rust is not fun.");
    }
    
    if !is_learning_easy {
        println!("Learning can be challenging, but it's worth it!");
    }
}
main();
```

    
    ==============================================================
                       Boolean in Flow-control                    
    ==============================================================
    Rust is fun!
    Learning can be challenging, but it's worth it!


### Comparison Operations

- Booleans are the result of comparison operations. Rust provides several comparison operators such as
    - `==`, `!=`: Equality and not equality operators
    - `<`, `>`, `<=`, and `>=`: Less than, greater than, less than or equal, greater than or equal operators respectively.

### Example:


```Rust
fn main() {
    banner("=", 32, "Comparison Operators");
    let x = 5;
    let y = 10;

    println!("x == y: {}", x == y);
    println!("x != y: {}", x != y);
    println!("x < y : {}", x < y);
    println!("x > y : {}", x > y);
    println!("x <= y: {}", x <= y);
    println!("x >= y: {}", x >= y);
}
main();
```

    
    ================================
          Comparison Operators      
    ================================
    x == y: false
    x != y: true
    x < y : true
    x > y : false
    x <= y: true
    x >= y: false


### Boolean Methods

- Rust provides a few useful methods for the bool type that can help you perform conditional operations and handle optional boolean values. Two notable methods are `then` and `unwrap_or`.

#### `then` Method

- The `then` method is used for conditional execution. It allows you to execute a specific action if the boolean value is true. Essentially, it helps in running code conditionally based on the boolean value.

- **Syntax:**

```rust
bool_value.then(|| some_expression)
```

- If bool_value is true, some_expression is executed and returned as an **Option**.
- If bool_value is false, None is returned.


```Rust
fn main() {
    banner("=", 62, "Understandin then method");
    let is_sunny = true;
    let enjoy_outdoor = is_sunny.then(|| 
        {
            "Let's go for a walk!"
    });
    
    match enjoy_outdoor {
        Some(activity) => println!("{}", activity),
        None => println!("Stay indoors."),
    }
}
main();
```

    
    ==============================================================
                       Understandin then method                   
    ==============================================================
    Let's go for a walk!


- The `is_sunny.then(|| "Let's go for a walk!")`:
    - If is_sunny is true, the string "Let's go for a walk!" is returned wrapped in Some. 
    - If is_sunny were false, it would return None.

- The match statement prints the activity if it's a `Some`, otherwise it prints "Stay indoors."

- **Extra Example by assigning a value to the `then` method**


```Rust
fn main() {
    let cond1 = true;
    let cond2 = false;

    // Using `then` method to conditionally execute a closure
    let result = cond1.then(|| {
        println!("Condition is true");
        42 // Returning a value from the closure
    });

    // Checking and printing the result of `then`
    match result {
        Some(value) => println!("`then` returned Some({})", value),
        None => println!("`then` returned None"),
    }
    
    // Using `then` method to conditionally execute a closure
    let result = cond2.then(|| {
        println!("Condition is false");
        42 
    });

    // Checking and printing the result of `then`
    match result {
        Some(value) => println!("`then` returned Some({})", value),
        None => println!("`then` returned None"),
    }
}

main();
```

    Condition is true
    `then` returned Some(42)
    `then` returned None


### The `unwrap_or` Method

- The `unwrap_or` method is used to handle optional boolean values (`Option<bool>`). It returns the boolean value if it exists (Some), or a default value if it is `None`.


- **Syntax:**

```rust
option_bool_value.unwrap_or(default_value)
```
    
- If option_bool_value is Some(true) or Some(false), the contained boolean value is returned.
- If option_bool_value is None, default_value is returned.

Example:


```Rust
fn main() {
    banner("=", 62, "The unwrap_or method");
    
    let is_sunny: Option<bool> = Some(true);
    let is_raining: Option<bool> = None;
    
    let sunny_day = is_sunny.unwrap_or(false);
    let raining_day = is_raining.unwrap_or(false);
    
    println!("Is it sunny? {}", sunny_day); 
    println!("Is it raining? {}", raining_day); 
}
main();    
```

    
    ==============================================================
                         The unwrap_or method                     
    ==============================================================
    Is it sunny? true
    Is it raining? false


- In this example:
    - `is_sunny.unwrap_or(false)`: Since is_sunny is Some(true), it unwraps and returns true.
    - `is_raining.unwrap_or(false)`: Since is_raining is None, it returns the default value `false`.

### Combining `then` and `unwrap_or`

- You can combine these methods to perform more complex operations. For example, you can conditionally execute a task and provide a default outcome if the condition isn't met.

- **Example:**


```Rust
fn main() {
    banner("=", 62, "Combing then and unwrap_or methods");
    
    let is_sunny: Option<bool> = Some(true);
    let go_for_walk = is_sunny.unwrap_or(false).then(|| "Let's go for a walk!");
    
    match go_for_walk {
        Some(activity) => println!("{}", activity),
        None => println!("Stay indoors."),
    }
}
main();
```

    
    ==============================================================
                  Combing then and unwrap_or methods              
    ==============================================================
    Let's go for a walk!


- In this example:
    - `is_sunny.unwrap_or(false)`: Unwraps is_sunny, returning true.
    - `.then(|| "Let's go for a walk!")`: Executes the closure and returns Some("Let's go for a walk!") if the unwrapped value is true, otherwise returns None.

## The `then` and `unwrap_or_else` Methods

- The `unwrap_or_else` method is called on the result of the `then` method.

- This method takes a closure, closures will be discussed in a later chapter, as an argument like this: `|| expression`
    - If the value on which unwrap_or_else is called is Some, it unwraps the value and does nothing with the closure.
    - If the value on which unwrap_or_else is called is None, it executes the closure.

- we can combine the two methods to achieve a certain result.


```Rust
fn main() {
    let condition = true;

    condition.then(|| println!("Condition is true")).unwrap_or_else(|| println!("Condition is false"));
}

main();
```

    Condition is true



```Rust
fn main() {
    let condition = false;

    condition.then(|| println!("Condition is true")).unwrap_or_else(|| println!("Condition is false"));
}
main();
```

    Condition is false


## Comprehensive Example


```Rust
fn main() {
    // Print a banner for the section
    banner("=", 62, "Boolean Types in Rust");

    // Declaring boolean variables
    let is_rust_awesome: bool = true;
    let is_raining: bool = false;

    // Print the values of the boolean variables
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is it raining? {}", is_raining);

    // Demonstrating boolean operations
    let are_both_true: bool = is_rust_awesome && is_raining; // AND operation
    let is_either_true: bool = is_rust_awesome || is_raining; // OR operation
    let is_not_rust_awesome: bool = !is_rust_awesome; // NOT operation

    // Print the results of boolean operations
    println!("Are both 'is_rust_awesome' and 'is_raining' true? {}", are_both_true);
    println!("Is either 'is_rust_awesome' or 'is_raining' true? {}", is_either_true);
    println!("Is 'is_rust_awesome' not true? {}", is_not_rust_awesome);

    // Using booleans in conditional statements
    if is_rust_awesome {
        println!("Rust is indeed awesome!");
    } else {
        println!("Rust is not awesome? That's surprising!");
    }

    if is_raining {
        println!("Don't forget to take an umbrella!");
    } else {
        println!("It's a nice day outside!");
    }
    
        banner("=", 62, "Comparison Operators");
    let x = 5;
    let y = 10;

    println!("x: {}, y: {}, x == y: {:<10} {}",  x, y, "", x == y);
    println!("x: {}, y: {}, x != y: {:<10} {}",  x, y, "", x != y);
    println!("x: {}, y: {}, x < y : {:<10} {}",  x, y, "", x < y);
    println!("x: {}, y: {}, x > y : {:<10} {}",  x, y, "", x > y);
    println!("x: {}, y: {}, x <= y: {:<10} {}",  x, y, "", x <= y);
    println!("x: {}, y: {}, x >= y: {:<10} {}",  x, y, "", x >= y);
}

// Call the main function to run the program
main();
```

    
    ==============================================================
                        Boolean Types in Rust                     
    ==============================================================
    Is Rust awesome? true
    Is it raining? false
    Are both 'is_rust_awesome' and 'is_raining' true? false
    Is either 'is_rust_awesome' or 'is_raining' true? true
    Is 'is_rust_awesome' not true? false
    Rust is indeed awesome!
    It's a nice day outside!
    
    ==============================================================
                         Comparison Operators                     
    ==============================================================
    x: 5, y: 10, x == y:            false
    x: 5, y: 10, x != y:            true
    x: 5, y: 10, x < y :            true
    x: 5, y: 10, x > y :            false
    x: 5, y: 10, x <= y:            true
    x: 5, y: 10, x >= y:            false


### Practical Example: Validating User Input

- Let's put booleans into a more practical context with an example of validating user input.


```Rust
fn main() {
    let username: &'static str = "user123";
    let password: &'static str = "password";

    let is_valid_username = username.len() >= 3;
    let is_valid_password = password.len() >= 8;

    if is_valid_username && is_valid_password {
        println!("Username and password are valid.");
    } else {
        if !is_valid_username {
            println!("Username is too short.");
        }
        if !is_valid_password {
            println!("Password is too short.");
        }
    }
}
main();
```

    Username and password are valid.


### Summary

- The bool type in Rust is used to represent `true` or `false` values.
- Booleans are essential for controlling the flow of programs using conditional statements and logical operations.
- Comparison operations yield boolean values.
- Booleans support logical operations such as `AND`, `OR`, and `NOT`.
- Booleans are commonly used in control flow statements and can be used to validate conditions in practical scenarios.

