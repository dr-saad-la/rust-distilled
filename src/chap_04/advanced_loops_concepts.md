<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Advanced Loops Concepts in Rust
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); 
    let message = format!("{:^width$}", message, width = nchar); 
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

### Returning Values from Loops

In Rust, loops can be more than just control flow structures for repeated execution—they can also be used to compute and return values. This is made possible through the use of the `break` statement with an associated value, which allows a loop to return a result upon termination. This feature enables more flexible and expressive looping constructs, particularly useful in scenarios where the loop's purpose is to find or compute a specific result.

#### Syntax

The syntax for returning a value from a loop involves initializing a variable with the `loop` construct and using the `break` statement to return a value:

```rust
let result = loop {
    // code to execute
    if condition {
        break value;
    }
};
```

In this structure:
 - loop: Initializes an infinite loop that will continue running until explicitly terminated.
 - if condition { break value; }: Inside the loop, a condition is evaluated. When this condition is true, the break statement is executed with an associated value, which terminates the loop and returns the value as the result of the loop.
 
### Benefits

- Using loops to return values can streamline your code by combining iteration and value computation into a single, concise construct. It eliminates the need for additional variables and state management outside the loop, leading to cleaner and more maintainable code. This approach is particularly powerful in scenarios such as searching, accumulating values, or any other context where the loop's purpose is to produce a single result.

## Example
Consider the following example where we use a loop to find the first number greater than 10 that is divisible by both 2 and 3:


```Rust
fn main() {
    banner("*", 52, "Returning Values from a loop");
    let mut num = 1;

    let result = loop {
        num += 1;

        if num > 10 && num % 2 == 0 && num % 3 == 0 {
            break num;
        }
    };

    println!("The number is: {}", result);
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                Returning Values from a loop            
    ****************************************************
    The number is: 12
    ****************************************************


### Code in Details

**Initialization**:

- `let mut num = 1`: A mutable integer variable `num` is initialized to 1. This variable will be incremented in each iteration of the loop.

**Infinite Loop**:

- `let result = loop { ... }`: An infinite loop is initiated, which will run until it is terminated by a `break` statement.

**Increment and Check**:

- `num += 1`: The variable `num` is incremented by 1 in each iteration.
- `if num > 10 && num % 2 == 0 && num % 3 == 0 { break num; }`: Inside the loop, a condition is evaluated. If `num` is greater than 10 and divisible by both 2 and 3, the `break` statement is executed with `num` as its value. This terminates the loop and assigns `num` to the variable `result`.

**Post-loop**:

- `println!("The number is: {}", result);`: After the loop terminates, the value of `result` is printed to the console, displaying the first number greater than 10 that meets the specified conditions.

## Loop Labels and Nesting

In Rust, you can label loops and refer to these labels to control nested loops. This is particularly useful when you need to break or continue outer loops. When you have nested loops, the `break` and `continue` statements by default apply to the innermost loop. However, by using loop labels, you can specify which loop the `break` or `continue` statements should apply to. Loop labels must begin with a single quote.

#### Syntax

```rust
'label: loop {
    // code for the outer loop
    loop {
        // code for the inner loop
        break 'label; // breaks the outer loop
    }
}
```

### Structure Explanation

**Loop Label**:

- `'label: loop { ... }`: The outer loop is labeled with `'label`. This label can be referenced later in the code to control the flow of the outer loop.

**Outer Loop**:

- The outer loop contains some code to execute and is identified by the label `'label`. The code within this loop will execute repeatedly unless explicitly interrupted by a `break` statement.

**Inner Loop**:

- Within the outer loop, there is an inner loop. The code within this loop will also execute repeatedly. By default, any `break` or `continue` statements within this inner loop apply only to the inner loop.

**Break Statement with Label**:

- `break 'label;`: This statement breaks the outer loop labeled `'label`. This allows the code to exit the outer loop from within the inner loop, demonstrating the control provided by loop labels.

### Example with Loop Labels and Nesting

Let us consider the next example, where we break out of an outer loop using a label


```Rust
fn main() {
    banner("*", 52, "Breaking Loop by labels");
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // Condition to break the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                  Breaking Loop by labels               
    ****************************************************
    Entered the outer loop
    Entered the inner loop
    Exited the outer loop
    ****************************************************


### Code in Details
**Outer Loop Initialization**:

- `'outer: loop { ... }`: The outer loop is labeled with `'outer`. This label is used to control the flow of the outer loop from within the nested inner loop.

**Entering the Outer Loop**:

- `println!("Entered the outer loop");`: This statement is executed each time the outer loop begins its iteration.

**Inner Loop Initialization**:

- `'inner: loop { ... }`: Within the outer loop, an inner loop is defined and labeled with `'inner`.

**Entering the Inner Loop**:

- `println!("Entered the inner loop");`: This statement is executed each time the inner loop begins its iteration.

**Breaking the Outer Loop**:

- `break 'outer;`: Inside the inner loop, this statement uses the loop label `'outer` to break the outer loop. This terminates the outer loop immediately, skipping any remaining code inside both the inner and outer loops.

**Unreachable Code in the Outer Loop**:

- `println!("This point will never be reached");`: This statement is placed after the inner loop within the outer loop. However, it will never be executed because the `break 'outer;` statement exits the outer loop before this point is reached.

**Post-loop Execution**:

- `println!("Exited the outer loop");`: After the `break 'outer;` statement terminates the outer loop, this statement is executed, indicating that the program has exited the outer loop.


### Another Example

This example is adapted from rust book, official documentation.


```Rust
fn main() {
    banner("*", 52, "Understanding nested loop and labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
            Understanding nested loop and labels        
    ****************************************************
    count = 0
    remaining = 10
    remaining = 9
    count = 1
    remaining = 10
    remaining = 9
    count = 2
    remaining = 10
    End count = 2
    ****************************************************


### Practical Example: Finding a Specific Value in a 2D Array

Let's consider a practical example where we have a 2D array (matrix) and we want to find a specific value. We will use nested loops to iterate through the array, and loop labels to break out of both loops once we find the value.


```Rust
fn main() {
    banner("*", 52, "Practical Example of Nested loop with labels");
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    let target = 5;
    let mut found = false;

    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target {
                println!("Found {} at position ({}, {})", target, i, j);
                found = true;
                break 'outer;
            }
        }
    }

    if !found {
        println!("{} not found in the matrix", target);
    }
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
        Practical Example of Nested loop with labels    
    ****************************************************
    Found 5 at position (1, 1)
    ****************************************************


### Code in Details

**Matrix Initialization**:

- `let matrix = [...];`: A 2D array (matrix) is initialized with integers from 1 to 9.

**Target Value**:

- `let target = 5;`: The target value we are searching for in the matrix is set to 5.
- `let mut found = false;`: A mutable boolean variable `found` is initialized to false. It will be set to true if the target value is found.

**Outer Loop Initialization**:

- `'outer: for (i, row) in matrix.iter().enumerate() { ... }`: The outer loop is labeled with `'outer` and iterates over each row in the matrix. The `enumerate()` method is used to get both the index and the row.

**Inner Loop Initialization**:

- `for (j, &value) in row.iter().enumerate() { ... }`: The inner loop iterates over each value in the current row. The `enumerate()` method is used to get both the index and the value.

**Condition Check and Break**:

- `if value == target { ... }`: Inside the inner loop, this condition checks if the current value is equal to the target value.
- `println!("Found {} at position ({}, {})", target, i, j);`: If the condition is met, a message is printed indicating the position of the target value.
- `found = true;`: The `found` variable is set to true.
- `break 'outer;`: The `break 'outer;` statement is used to exit the outer loop immediately.

**Post-loop Check**:

- `if !found { ... }`: After the loops, this condition checks if the target value was not found.
- `println!("{} not found in the matrix", target);`: If the target value was not found, a message is printed indicating that the value was not found in the matrix.


## Practical Example 02: Image Processing

Let consider an example in image processing where we want to search for a specific color pattern in a 3D image represented as a matrix of RGB values. If we find the pattern, we break out of the nested loops.


```Rust
fn main() {
    // Example 3D image matrix (2x2 pixels for simplicity)
    let image = [
        [[255, 0, 0], [0, 255, 0]],  // Row 1: [Red, Green]
        [[0, 0, 255], [255, 255, 0]], // Row 2: [Blue, Yellow]
    ];

    // Target pattern (Yellow)
    let target = [255, 255, 0];
    let mut found = false;

    'outer: for (i, row) in image.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            if pixel == &target {
                println!("Found the pattern at position ({}, {})", i, j);
                found = true;
                break 'outer;
            }
        }
    }

    if !found {
        println!("Pattern not found in the image");
    }
}

main();
```

    Found the pattern at position (1, 1)


### Code in Details

**Image Initialization:**

```rust
let image = [
    [[255, 0, 0], [0, 255, 0]],  // Row 1: [Red, Green]
    [[0, 0, 255], [255, 255, 0]], // Row 2: [Blue, Yellow]
];
```

A 3D array (image) is initialized with RGB values representing the colors Red, Green, Blue, and Yellow.

**Target Pattern:**

```rust
let target = [255, 255, 0]; // Yellow
let mut found = false;
```
The target color pattern we are searching for is set to Yellow [255, 255, 0]. A mutable boolean variable found is initialized to false. It will be set to true if the target pattern is found.

**Outer Loop Initialization:**

```rust
'outer: for (i, row) in image.iter().enumerate() { ... }
```
The outer loop is labeled with 'outer and iterates over each row in the image matrix. The enumerate() method is used to get both the index and the row.

**Inner Loop Initialization:**

```rust
for (j, pixel) in row.iter().enumerate() { ... }
```

Within the outer loop, an inner loop iterates over each pixel in the current row. The enumerate() method is used to get both the index and the pixel.

**Condition Check and Break:**

```rust
if pixel == &target { ... }
```
Inside the inner loop, this condition checks if the current pixel matches the target pattern.

**Print and Break:**

```rust
println!("Found the pattern at position ({}, {})", i, j);
found = true;
break 'outer;
```

If the condition is met, a message is printed indicating the position of the target pattern. The found variable is set to true, and the break 'outer; statement is used to exit the outer loop immediately.

**Post-loop Check:**
```rust
if !found { ... }
```

After the loops, this condition checks if the target pattern was not found. If the target pattern was not found, a message is printed indicating that the pattern was not found in the image

## Conclusion

In this section, we explored the more advanced features of Rust loops, specifically focusing on two crucial aspects: returning values from loops and utilizing loop labels to effectively manage nested loops.

We began by discussing how Rust's unique capability of allowing loops to return values using the `break` statement with a value can greatly enhance the flexibility and control over loop executions. This feature allows for more expressive and powerful loop constructs, enabling developers to capture and utilize results directly from within the loop.

Next, we examined the use of loop labels to manage nested loops. By labeling loops with descriptive identifiers, we can precisely control which loop is affected by `break` and `continue` statements, thus avoiding confusion and ensuring the correct loop is targeted. This is particularly useful in complex scenarios involving multiple layers of nested loops, where direct control over the outer loops from within inner loops is necessary.

