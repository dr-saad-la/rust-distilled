<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Using Continue
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); 
    let message = format!("{:^width$}", message, width = nchar); 
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

The `continue` statement is a control flow tool used within loops to skip the remainder of the current iteration and proceed directly to the next iteration. This is particularly useful in scenarios where certain conditions or values should be bypassed without terminating the loop entirely.

### Syntax

```rust
loop {
    if condition {
        continue;
    }
    // Code to execute if the condition is not met
}
```

### The Mechanism of `continue` flow

1. **Loop Initialization**: The `loop` keyword initiates an infinite loop, which will continue executing until explicitly terminated by a `break` statement or an external condition.

2. **Condition Check**: `if condition`: Inside the loop, a condition is evaluated. If this condition evaluates to `true`, the `continue` statement is executed.

3. **Continue Statement**: `continue`: The `continue` statement immediately skips the remaining code in the current iteration and jumps to the next iteration of the loop. This allows the loop to proceed without executing the code below the `continue` statement for the current iteration.

4. **Remaining Code**: Any code following the `continue` statement within the loop will not be executed for the current iteration if the condition is met. Instead, the loop control proceeds to the next iteration.

### Practical Use

The `continue` statement is valuable in various scenarios, such as:

- **Filtering Data**: Skipping specific values in a collection that do not meet certain criteria.
- **Error Handling**: Ignoring erroneous or invalid data points during processing.
- **Efficiency**: Avoiding unnecessary computations or operations for specific conditions.


### Example

In this example, we will iterate over a range of numbers and use the `continue` statement to skip even numbers.


```Rust
fn main() {
    banner("*", 52, "Using continue with for loop");
    for number in 1..10 {
        if number % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", number);
    }
    println!("Loop ended here");
    println!("{}", "*".repeat(52))
}
main();
```

    
    ****************************************************
                Using continue with for loop            
    ****************************************************
    Odd number: 1
    Odd number: 3
    Odd number: 5
    Odd number: 7
    Odd number: 9
    Loop ended here
    ****************************************************


### Code in Details

1. **Loop through the Range**:
   - `for i in 1..10`: This `for` loop iterates over the range from 1 to 9 (note that the end value 10 is excluded).

2. **Condition Check**:
   - `if i % 2 == 0`: Inside the loop, we check if the current number `i` is even by using the modulus operator (`%`). If the result of `i % 2` is 0, then `i` is an even number.

3. **Continue Statement**:
   - `continue`: If the condition is met (i.e., `i` is even), the `continue` statement is executed. This immediately skips the remaining code in the current iteration and jumps to the next iteration of the loop.

4. **Remaining Code**:
   - `println!("Odd number: {}", i)`: This line prints the current value of `i` if it is not skipped by the `continue` statement. Since the `continue` statement is executed for even numbers, only odd numbers are printed.


### Using `continue` with `while`

The `continue` statement can also be used with `while` loops to skip the current iteration and proceed to the next one based on a condition.

### Example

We will use the same example with `while` loop to iterate over a range of numbers and skip even numbers using the `continue` statement.


```Rust
fn main() {
    banner("*", 52, "Using continue with while loop");
    let mut i = 1;

    while i < 10 {
        if i % 2 == 0 {
            i += 1;
            continue;
        }
        println!("Odd number: {}", i);
        i += 1;
    }
    println!("Program ends");
    println!("{}", "*".repeat(52))
}
main();
```

    
    ****************************************************
               Using continue with while loop           
    ****************************************************
    Odd number: 1
    Odd number: 3
    Odd number: 5
    Odd number: 7
    Odd number: 9
    Program ends
    ****************************************************


### Using `continue` with `loop`

Similar to `for` and `while` loops, the `continue` statement can be utilized within a `loop` construct in Rust. However, since `loop` creates an infinite loop, it is essential to control the loop's termination to prevent it from running indefinitely. This typically involves incorporating a `break` statement at an appropriate point.

### Example

Here is the previous example adapted to use the `loop` construct along with the `continue` statement:


```Rust
fn main() {
    banner("*", 52, "Continue statement with loop");
    let mut count = 0;

    loop {
        count += 1;

        if count % 2 == 0 {
            continue;
        }

        if count > 10 {
            break;
        }

        println!("Odd count: {}", count);
    }
    println!("Program ended!");
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                Continue statement with loop            
    ****************************************************
    Odd count: 1
    Odd count: 3
    Odd count: 5
    Odd count: 7
    Odd count: 9
    Program ended!
    ****************************************************


## Combining `break` and `continue`

It is commom to combine the `break` and `continue` statements within a single loop to achieve more precise control over the flow of the program. This allows you to skip certain iterations based on specific conditions and exit the loop entirely under other conditions. This combination is particularly useful in complex scenarios where both skipping and terminating conditions need to be managed within the same loop.

### Comprehensive Example

In the following example, we will iterate over a range of numbers, skip even numbers using `continue`, and break the loop when a specific number is reached using `break`.


```Rust
fn main() {
    banner("*", 52, "Continue-Break Combination");
    let mut count = 0;

    loop {
        count += 1;

        // Skip even numbers
        if count % 2 == 0 {
            continue;
        }

        // Print the odd number
        println!("Odd number: {}", count);

        // Break the loop when count reaches 15
        if count >= 15 {
            println!("Reached the limit, breaking the loop.");
            break;
        }
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                 Continue-Break Combination             
    ****************************************************
    Odd number: 1
    Odd number: 3
    Odd number: 5
    Odd number: 7
    Odd number: 9
    Odd number: 11
    Odd number: 13
    Odd number: 15
    Reached the limit, breaking the loop.
    Loop has ended.
    ****************************************************


### Code in Details

**Loop Initialization**:

- `let mut count = 0`: A mutable integer variable `count` is initialized to 0 to start the iteration.

**Infinite Loop**:

- `loop`: This creates an infinite loop that will continue running until it is explicitly terminated with a `break` statement.

**Increment and Print**:

- `count += 1`: The `count` variable is incremented by 1 in each iteration.

**Condition Check for `continue`**:

- `if count % 2 == 0 { continue; }`: This condition checks if the current value of `count` is even using the modulus operator (`%`). If the result of `count % 2` is 0, then `count` is an even number. The `continue` statement is executed, which skips the remaining code in the current iteration and proceeds to the next iteration.

**Print Odd Numbers**:

- `println!("Odd number: {}", count)`: This line prints the current value of `count` if it is not skipped by the `continue` statement. Since the `continue` statement is executed for even numbers, only odd numbers are printed.

**Condition Check for `break`**:

- `if count >= 15 { break; }`: This condition checks if the `count` has reached or exceeded 15. If true, the `break` statement is executed to exit the loop. Before breaking, a message is printed indicating that the limit has been reached.

**Post-loop Message**:

- `println!("Loop has ended.")`: After exiting the loop, a message indicating the end of the loop is printed to the console.


### Conclusion

In this section, we explored the use of the `break` and `continue` statements in Rust, powerful tools for controlling the flow of loops. 

- **The `break` Statement**: We learned how to use the `break` statement to exit loops immediately when a specific condition is met. This technique is essential for terminating potentially infinite loops or stopping iterations when a goal has been achieved.
- **The `continue` Statement**: We examined the `continue` statement, which allows us to skip the rest of the current iteration and proceed directly to the next one. This is particularly useful for bypassing specific conditions or values without exiting the loop entirely.
- **Combining `break` and `continue`**: By combining both statements in a single loop, we can achieve precise control over complex looping scenarios. This combination helps in scenarios where certain iterations need to be skipped, while others necessitate the termination of the loop.
