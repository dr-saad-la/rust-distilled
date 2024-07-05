<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Conditional Loop with `while`
</div>

The following function will be used to format the output using nice banner.


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); 
    let message = format!("{:^width$}", message, width = nchar); 
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

The `while` loop in Rust is one of the simplest and most straightforward looping constructs available. It repeatedly executes a block of code as long as a specified condition evaluates to `true`. This type of loop is especially useful when the number of iterations is **not known beforehand** and the loop's continuation depends on a dynamic condition that changes within the loop. 

The `while` loop ensures that the block of code is executed only when necessary by evaluating the condition at the start of each iteration. However, it is crucial to include a **condition control** within the loop body that modifies the loop condition; otherwise, the loop will run indefinitely, creating an infinite loop. Properly managing this stopping condition is essential to avoid unintentional infinite loops and to ensure that the program executes as intended.

#### Syntax

```rust
while condition {
    // code to execute while the condition is true
    // ensure the condition is modified to eventually become false
}
```

In this syntax, condition is a **boolean expression**. The loop continues to execute as long as condition evaluates to true. It is important to ensure that the condition is influenced by the code inside the loop so that the loop can eventually terminate. This prevents the loop from running forever and potentially causing the program to hang.

### The While Loop Flow Execution

1. Initialization:
   - The variable is initialized to a specific value.

2. Condition Check:
    - Before each iteration, the given condition is evaluated.

3. Loop Body:
    - If the condition is true, the current value of a variable is used such as printing it, increment it of decrement it. 

4. Termination:
    - The loop continues until the condition becomes `false`, at which point the condition count > 0 evaluates to false, and the loop terminates

### Example

Let us consider a `while` loop that decrements a counter until it reaches zero. This ensures that the loop has a clear stopping condition.

**Note**:
The counter must be `mutable` variable otherwise you will get a compile error, thus we need to declare it so using the `mut` keyword.


```Rust
fn main() {
    banner("*", 52, "While loop in Rust");
    // Initialize a mutable variable `count` with a value of 5
    // This must be mutable variable
    let mut count = 5;

    // Begin a while loop that continues as long as `count` is greater than 0
    while count > 0 {
        // Print the current value of `count`
        println!("Count: {}", count);
        // Decrement `count` by 1
        count -= 1;              // the short and common way of decrementing
        
        // The previous can be written like this
        count = count - 1;     // 
    }

    // Print a message indicating the loop has ended
    println!("Loop has ended.");
}
main();
```

    
    ****************************************************
                     While loop in Rust                 
    ****************************************************
    Count: 5
    Count: 3
    Count: 1
    Loop has ended.


## Code in Details

### Initialization
The loop starts with the variable `count` initialized to `5`.

### Condition Check
Before each iteration, the loop checks if the condition `count > 0` is true. If the condition is true, the loop body executes. If the condition is false, the loop terminates.

### Loop Body
Inside the loop body, the current value of `count` is printed. Then, the `count` variable is decremented by `1` using the `-= 1` operation.

### Condition Control
The decrement operation ensures that the value of `count` changes with each iteration, moving closer to the stopping condition where `count` equals `0`. Without this operation, if the condition relies solely on the value of count, the loop could potentially run indefinitely.

### Termination
When `count` becomes `0`, the condition `count > 0` evaluates to false, causing the loop to terminate. The program then proceeds to execute the code following the loop, which in this case is printing "Loop has ended."

## Flow of Execution

### First Iteration
- `count` is `5`, the condition `count > 0` is true.
- "Count: 5" is printed.
- `count` is decremented to `4`.

### Second Iteration
- `count` is `4`, the condition `count > 0` is true.
- "Count: 4" is printed.
- `count` is decremented to `3`.

### Third Iteration
- `count` is `3`, the condition `count > 0` is true.
- "Count: 3" is printed.
- `count` is decremented to `2`.

### Fourth Iteration
- `count` is `2`, the condition `count > 0` is true.
- "Count: 2" is printed.
- `count` is decremented to `1`.

### Fifth Iteration
- `count` is `1`, the condition `count > 0` is true.
- "Count: 1" is printed.
- `count` is decremented to `0`.

### Termination
- `count` is `0`, the condition `count > 0` is false.
- The loop terminates, and "Loop has ended." is printed.

### Example

Here is another example where we a counter is incremented.


```Rust
fn main() {
    banner("*", 52, "Another while loop example");
    let mut num = 0;

    while num <= 5 {
        println!(" {}", num);
        num += 1;
    }

    println!(" End of while loop!!!");
    println!("{}", "*".repeat(52))
}
main();
```

    
    ****************************************************
                 Another while loop example             
    ****************************************************
     0
     1
     2
     3
     4
     5
     End of while loop!!!
    ****************************************************


## Complex Conditions

The `while` loop is a versatile construct that excels in handling complex conditions involving multiple variables and logical operations. This capability allows programmers to create more sophisticated and dynamic control flows that can adapt to a wide range of scenarios. By leveraging the power of logical operators and multi-variable conditions, `while` loops can manage intricate tasks with precision and efficiency.

Consider the following example where we have two variables:


```Rust
fn main() {
    banner("*", 52, "Complex while loop");
    let mut x = 5;
    let mut y = 10;

    while x < y && y < 20 {
        println!("x: {}, y: {}", x, y);
        x += 1;
        y += 2;
    }

    println!("Loop has ended.");
    println!("{}", "*".repeat(52))
}
main();
```

    
    ****************************************************
                     Complex while loop                 
    ****************************************************
    x: 5, y: 10
    x: 6, y: 12
    x: 7, y: 14
    x: 8, y: 16
    x: 9, y: 18
    Loop has ended.
    ****************************************************


Notice that the while loop continues to execute as long as both conditions x < y and y < 20 are true. The loop’s body prints the current values of x and y, then increments x by 1 and y by 2 in each iteration. The loop will terminate once either of the conditions becomes `false`.

## Nested while Loops

`while` loops in Rust can be nested within each other, providing a powerful mechanism for managing more complex iteration patterns. Nested loops enable the execution of a loop inside another loop, which is particularly useful in scenarios where multiple dimensions of iteration are required, such as traversing multi-dimensional data structures or performing repeated operations within repeated operations.

Consider the following example where we have two nested `while` loops:


```Rust
fn main() {
    banner("*", 52, "Nested while loop");
    
    let mut i = 0;

    while i < 3 {
        let mut j = 0;
        
        while j < 3 {
            println!("i: {}, j: {}", i, j);
            j += 1;
        }

        i += 1;
    }
    println!("End of loop");
    println!("{}", "*".repeat(52))
}
main();
```

    
    ****************************************************
                     Nested while loop                  
    ****************************************************
    i: 0, j: 0
    i: 0, j: 1
    i: 0, j: 2
    i: 1, j: 0
    i: 1, j: 1
    i: 1, j: 2
    i: 2, j: 0
    i: 2, j: 1
    i: 2, j: 2
    End of loop
    ****************************************************


The outer loop runs three times, and for each iteration of the outer loop, the inner loop also runs three times. The nested structure allows the program to iterate over a combination of `i` and `j` values, printing each pair of indices.

### Handling Infinite Loops Safely

When using `while` loops that may potentially run indefinitely, it is crucial to implement safe exit conditions or timeouts to prevent the program from hanging or consuming resources indefinitely. Properly handling these scenarios ensures that your program remains responsive and efficient.

#### Safe Exit Conditions

A safe exit condition is a condition that guarantees the termination of the loop after a certain number of iterations or upon meeting a specific criterion. This is essential in preventing infinite loops that could otherwise cause the program to freeze or crash.

#### Example

Consider the following example where a `while` loop runs until a maximum number of iterations is reached:


```Rust
fn main() {
    let mut count = 0;
    let max_iterations = 100;

    while count < max_iterations {
        // Perform some task
        println!("Iteration: {}", count);

        count += 1;

        // Safe exit condition
        if count == max_iterations {
            println!("Reached maximum iterations, exiting loop.");
            break;
        }
    }
}
// main();  // Uncoment this line to execute the program
```

### Code in Details

#### Initialization:

- The variable `count` is initialized to `0`.
- The variable `max_iterations` is set to `100`, defining the maximum number of loop iterations.

#### Condition Check:

- The loop condition `count < max_iterations` ensures that the loop will execute as long as `count` is less than `100`.

#### Loop Body:

- Inside the loop, the current iteration number is printed.
- The `count` variable is incremented by `1` on each iteration.

#### Safe Exit Condition:

- An additional check within the loop body ensures that if `count` reaches `max_iterations`, a message is printed, and the `break` statement exits the loop.

### Benefits of Safe Exit Conditions

#### Preventing Infinite Loops:

- Safe exit conditions ensure that loops terminate after a finite number of iterations, preventing the program from running indefinitely.

#### Resource Management:

- By guaranteeing loop termination, safe exit conditions help manage system resources efficiently, preventing excessive CPU usage and memory consumption.

#### Program Stability:

- Implementing safe exit conditions enhances the stability of the program, making it less prone to freezing or crashing due to runaway loops.

### Considerations for Safe Exit Conditions

#### Timeout Mechanisms:

- In addition to iteration limits, consider implementing timeout mechanisms where the loop exits after a certain time period has elapsed.

#### User Interruption:

- For interactive programs, provide mechanisms for user interruption (e.g., pressing a key to exit the loop).

#### Monitoring Loop Progress:

- Regularly monitor the progress of the loop to ensure it is behaving as expected and not stuck in an infinite state.


### Performance Considerations

When using `while` loops in Rust, it is important to consider performance implications, particularly in scenarios where the loop may execute numerous iterations or involve computationally intensive tasks. Efficiently managing these aspects can significantly impact the overall performance of your program.

1. **Efficient Condition Checks**:
   - Ensure that the loop's condition is evaluated efficiently. Avoid complex or computationally expensive conditions that need to be checked at each iteration.
   - Example:
     ```rust
     // Less efficient
     while expensive_function() {
         // Loop body
     }

     // More efficient
     let condition = expensive_function();
     while condition {
         // Loop body
     }
     ```

2. **Minimal Work Within the Loop**:
   - Strive to minimize the amount of work performed within each iteration of the loop. This can help reduce the overall execution time and improve performance.
   - Avoid unnecessary computations or operations inside the loop body. Move invariant computations outside the loop when possible.

3. **Avoiding Infinite Loops**:
   - Ensure that the loop has a clear and achievable exit condition to prevent it from running indefinitely. Infinite loops can cause the program to hang or consume excessive resources.
   - Example:
     ```rust
     let mut count = 0;
     while count < 100 {
         // Perform some work
         count += 1; // Ensure the loop will terminate
     }
     ```

4. **Memory Management**:
   - Be mindful of memory allocation within the loop. Frequent allocations and deallocations can lead to performance bottlenecks. Reuse memory when possible or allocate memory outside the loop.

5. **Optimizing for Data Locality**:
   - Access data sequentially when possible to take advantage of CPU cache locality. This can significantly speed up memory access times and improve performance.
   - Example:
     ```rust
     let mut array = [0; 1000];
     let mut i = 0;
     while i < array.len() {
         array[i] = i;
         i += 1;
     }
     ```

6. **Parallel Processing**:
   - For loops that involve heavy computations, consider parallelizing the workload to leverage multi-core processors. Rust's concurrency primitives, such as threads and async tasks, can be utilized to achieve this.
   - Example:
     ```rust
     use std::thread;

     let mut handles = vec![];
     for i in 0..4 {
         handles.push(thread::spawn(move || {
             // Perform parallel computation
         }));
     }

     for handle in handles {
         handle.join().unwrap();
     }
     ```


Paying more attention to these performance considerations, you can write more efficient and effective `while` loops in Rust. This will help ensure that your programs run smoothly and make the best use of available computational resources.

## The while loop usage

The `while` loop is a versatile and powerful control flow construct used in various real-world applications. Below are some common scenarios where `while` loops are particularly useful:

1. **Reading Data from a Stream**:
   - In applications that involve reading data from a continuous data stream (e.g., sensor data, network sockets), a `while` loop can be used to keep reading data until a certain condition is met, such as receiving a specific signal or end-of-file (EOF).

   ```rust
   use std::io::{self, BufRead};

   fn main() {
       let stdin = io::stdin();
       let mut handle = stdin.lock();

       let mut buffer = String::new();

       while handle.read_line(&mut buffer).unwrap() > 0 {
           println!("Read line: {}", buffer.trim());
           buffer.clear();
       }

       println!("End of input stream.");
   }
   ```
   
   
2. **Polling for Changes:**
    - In applications that require regularly checking for changes or updates (e.g., file modifications, new messages in a queue), a while loop can repeatedly poll for updates until a stop condition is triggered.
    
```rust
use std::time::{Duration, Instant};

fn main() {
    let start_time = Instant::now();

    while start_time.elapsed() < Duration::from_secs(10) {
        println!("Checking for updates...");
        // Simulate a delay for polling
        std::thread::sleep(Duration::from_secs(1));
    }

    println!("Finished polling for updates.");
}
    ```

3. **Waiting for User Input**:
    - Interactive applications often need to wait for user input in a loop. A while loop can be used to repeatedly prompt the user until valid input is provided or a certain condition is met.

```rust
use std::io;

fn main() {
    let mut input = String::new();

    while input.trim() != "exit" {
        println!("Enter a command (type 'exit' to quit):");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() != "exit" {
            println!("You entered: {}", input.trim());
        }
    }

    println!("Exiting the application.");
}
```

4. **Game Loops:**
 - Many games use a main loop to repeatedly update the game state, process user input, and render the game. This loop continues until the game is exited.

```rust
fn main() {
    let mut running = true;

    while running {
        // Process user input
        // Update game state
        // Render game frame

        // Example condition to stop the game loop
        if user_wants_to_exit() {
            running = false;
        }
    }

    println!("Game loop has ended.");
}

fn user_wants_to_exit() -> bool {
    // Replace with actual logic to determine if the user wants to exit
    false
}
```

5. **Retry Mechanisms:**
 - In applications that perform operations prone to temporary failures (e.g., network requests, database queries), a while loop can be used to retry the operation until it succeeds or a maximum number of retries is reached.

```rust
fn main() {
    let max_retries = 5;
    let mut attempts = 0;

    while attempts < max_retries {
        attempts += 1;
        if perform_operation() {
            println!("Operation succeeded on attempt {}", attempts);
            break;
        } else {
            println!("Operation failed on attempt {}. Retrying...", attempts);
        }
    }

    if attempts == max_retries {
        println!("Operation failed after {} attempts.", max_retries);
    }
}

fn perform_operation() -> bool {
    // Replace with actual operation logic
    false
}
```


## Summary

- The `while` loop is a fundamental control flow construct in Rust that provides a straightforward way to perform repeated actions based on dynamic conditions.

- Its simplicity and flexibility make it an essential tool for managing iterations in various real-world applications. By ensuring proper condition control and understanding the flow of execution, Rust programmers can leverage the `while` loop to handle complex tasks efficiently and effectively. 

- Whether reading data from streams, polling for changes, waiting for user input, or managing game loops, the `while` loop remains a powerful and versatile component of Rust programming.

