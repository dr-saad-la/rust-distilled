<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Using Break
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); 
    let message = format!("{:^width$}", message, width = nchar); 
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

The `break` statement in Rust is a powerful control flow mechanism used to terminate a loop immediately, bypassing the loop's conditional checks. This statement is particularly useful in scenarios where you need to exit a loop as soon as a specific condition is satisfied, thus preventing further iterations.

### Syntax

The syntax of the `break` statement is straightforward and can be employed in various types of loops, such as `loop`, `while`, and `for` loops. Below is the general syntax for using `break` within a loop:

```rust
loop {
    if condition {
        break;
    }
    // Code to execute if the condition is not met
}
```

## Exiting loops Mechanism
This general syntax shows how to exit out a control flow with with `loop` in Rust. As discussed in a previous sections:

- The **loop** initiates an infinite loop, which will continue to execute until explicitly terminated.
- **if condition**: This is a conditional statement that checks if a specific condition is true. If the condition evaluates to true, the `break` statement is executed.
- **break**: The `break` statement immediately exits the loop, regardless of any remaining iterations or conditions.
- **// Code to execute if the condition is not met**: This is the code block that will be executed in each iteration of the loop if the condition is not satisfied.


## Example of Exiting a Loop with `break`

Here is a simple example that demonstrates how to use the `break` statement to exit a loop when a specific condition is met. In this example, we will exit the loop as soon as the variable `count` reaches 3.


```Rust
fn main() {
    banner("*", 52, "Breaking the loop:");
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count >= 5 {
            break;
        }
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52) );
}
main();
```

    
    ****************************************************
                     Breaking the loop:                 
    ****************************************************
    Count: 1
    Count: 2
    Count: 3
    Count: 4
    Count: 5
    Loop has ended.
    ****************************************************


### Code in Details

1. **Initialization**:
   - `let mut count = 0`: A mutable integer variable `count` is initialized to 0.

2. **Infinite Loop**:
   - `loop`: An infinite loop is started. This loop will continue running until it is explicitly terminated.

3. **Increment and Print**:
   - `count += 1`: The `count` variable is incremented by 1 in each iteration.
   - `println!("Count: {}", count)`: The current value of `count` is printed to the console.

4. **Condition Check and `break`**:
   - `if count >= 5`: Inside the loop, we check if the `count` variable is greater than of equal to 5.
   - `break`: If the condition is met, the `break` statement is used to exit the loop immediately.

5. **Post-loop Message**:
   - `println!("Loop has ended.")`: After exiting the loop, a message indicating the end of the loop is printed to the console.


## Using `break` with `while`

The `break` statement can also be used within a `while` loop to exit the loop when a specific condition is met. This is useful for dynamically controlled loops where the exit condition is determined at runtime.

### Example

Here is an example that demonstrates how to use `break` within a `while` loop:


```Rust
fn main() {
    banner("*", 52, "Breaking While Loop");
    let mut count = 0;
    

    while count < 10 {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            println!("Count has reached 5, breaking the loop.");
            break;
        }
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52))
}
main();
```

    
    ****************************************************
                    Breaking While Loop                 
    ****************************************************
    Count: 1
    Count: 2
    Count: 3
    Count: 4
    Count: 5
    Count has reached 5, breaking the loop.
    Loop has ended.
    ****************************************************


### Code in Details

1. **Initialization**:
   - `let mut count = 0`: A mutable integer variable `count` is initialized to 0.

2. **While Loop**:
   - `while count < 10`: This `while` loop continues running as long as `count` is less than 10.

3. **Increment and Print**:
   - `count += 1`: The `count` variable is incremented by 1 in each iteration.
   - `println!("Count: {}", count)`: The current value of `count` is printed to the console.

4. **Condition Check and `break`**:
   - `if count == 5`: Inside the loop, we check if the `count` variable is equal to 5.
   - `println!("Count has reached 5, breaking the loop.")`: A message is printed indicating that the count has reached 5.
   - `break`: The `break` statement is used to exit the loop immediately after the condition is met.

5. **Post-loop Message**:
   - `println!("Loop has ended.")`: After exiting the loop, a message indicating the end of the loop is printed to the console.

### Simple Example of Breaking Out of the `for` Loop

the `break` statement can also be used to exit a `for` loop prematurely when a specific condition is met. This can be particularly useful when you are searching for a particular value or condition within a collection and want to stop iterating once the condition is found.

#### Example

In this example, we will iterate over a range of numbers and break out of the loop when a specific condition is met.


```Rust
fn main() {
    banner("*", 52, "Breaking for loop");
    for i in 1..10 {
        if i == 5 {
            println!("Reached the number: {}", i);
            break;
        }
        println!("Current number: {}", i);
    }
    println!("Loop has ended.");
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                     Breaking for loop                  
    ****************************************************
    Current number: 1
    Current number: 2
    Current number: 3
    Current number: 4
    Reached the number: 5
    Loop has ended.
    ****************************************************


### Code in Details

1. **Loop through the Range**:
   - `for i in 1..10`: This `for` loop iterates over the range from 1 to 9 (note that the end value 10 is excluded).

2. **Condition Check and `break`**:
   - `if i == 5`: Inside the loop, we check if the current number `i` is equal to 5.
   - `println!("Reached the number: {}", i)`: If the condition is met, a message is printed indicating that the number 5 has been reached.
   - `break`: The `break` statement is used to exit the loop immediately after the condition is met.

3. **Loop Body**:
   - `println!("Current number: {}", i)`: The current value of `i` is printed to the console for each iteration of the loop.

4. **Post-loop Message**:
   - `println!("Loop has ended.")`: After exiting the loop, a message indicating the end of the loop is printed to the console.


## Practical Examples of Exiting a Loop with `break`

Here is a practical example that demonstrates how to use the `break` statement to exit a loop when a specific condition is met. In this example, we will search for a specific number in an array and exit the loop as soon as we find it.


```Rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    let mut found = false;

    for &number in numbers.iter() {
        if number == target {
            found = true;
            break;
        }
    }

    if found {
        println!("Found the target number: {}", target);
    } else {
        println!("Target number not found.");
    }
}
main();
```

    Found the target number: 7


### Code in Details

1. **Initialization**:
   - `numbers`: An array of integers from 1 to 10.
   - `target`: The number we are searching for in the array, set to 7.
   - `found`: A boolean variable initialized to `false`. It will be set to `true` if the target number is found.

2. **Loop through the Array**:
   - `for &number in numbers.iter()`: This `for` loop iterates over each element in the `numbers` array. The `&number` syntax means we are borrowing each element of the array.

3. **Condition Check and `break`**:
   - `if number == target`: Inside the loop, we check if the current number is equal to the target number.
   - `found = true`: If the condition is met, we set the `found` variable to `true`.
   - `break`: The `break` statement is used to exit the loop immediately after finding the target number.

4. **Post-loop Check**:
   - `if found`: After the loop, we check if the `found` variable is `true`.
   - `println!`: If the target number was found, we print a message indicating the number was found. If not, we print a message indicating the number was not found.


## Real-World Example: User Input Validation

In this example, we'll demonstrate how the `break` statement can be used in a real-world scenario: validating user input in a loop until a valid input is received.

```rust
use std::io::{self, Write};

fn main() {
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        if attempts >= max_attempts {
            println!("Maximum attempts reached. Exiting...");
            break;
        }

        print!("Enter a number between 1 and 10: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                attempts += 1;
                continue;
            }
        };

        if input >= 1 && input <= 10 {
            println!("Valid input: {}", input);
            break;
        } else {
            println!("Input out of range. Please enter a number between 1 and 10.");
            attempts += 1;
        }
    }
}
```
## Summary

The `break` statement is a powerful control flow tool in Rust that allows you to exit loops prematurely based on specific conditions. By integrating `break` within `loop`, `while`, and `for` loops, you can enhance the efficiency and clarity of your code. 

### Key Points:

- **Immediate Exit**: The `break` statement immediately terminates the loop, regardless of any remaining iterations or conditions.
- **Flexible Usage**: `break` can be used in various loop constructs (`loop`, `while`, and `for`) to handle different control flow requirements.
- **Condition-based Termination**: Combining `break` with conditional statements (`if`, `else if`) provides precise control over when and how loops should terminate.
- **Performance Considerations**: Proper use of `break` can improve the performance of your program by avoiding unnecessary iterations, thereby optimizing resource usage.
- **Real-world Applications**: `break` is particularly useful in scenarios such as searching through collections, handling user input, and managing complex control flows in applications.

