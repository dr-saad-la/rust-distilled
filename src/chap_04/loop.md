<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Looping with `loop`
</div>

### Infinite Loop with `loop`

The `loop` keyword in Rust creates an infinite loop, which repeatedly executes a block of code indefinitely. This type of loop is particularly useful in scenarios where continuous execution is required until a certain condition is met internally, at which point the `break` statement can be used to exit the loop.

#### Syntax

```rust
loop {
    // code to execute repeatedly
    // use break to exit the loop
}
```


### Example

Here is an example of an infinite loop that continuously prints a message. The loop is exited when a specific condition is met:

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);
        
        // This to break out of the infinite loop
        if count >= 5 {
            break;
        }
    }
}
```

	Count: 1
	Count: 2
	Count: 3
	Count: 4
	Count: 5

The previous `loop` prints the iteration count and increments it by 1 in each cycle. The loop is terminated when count reaches 5, using the `break` statement. The `break` statement will be discussed in a later section.

Note that without the `if ... break`, the loop will run forever. You may comment those lines to test that out.

## Stopping the Infinite Loop

If an infinite loop is running without a break condition, you may need to stop it manually depending on the platform you are using:

1. **Linux and macOS**: Press `Ctrl + C` in the terminal where the program is running to terminate the loop.

2. **Windows**: Press `Ctrl + C` in the Command Prompt or PowerShell where the program is running to terminate the loop.


## Usage Examples of Infinite Loops

### Server Listening for Connections

Infinite loops are commonly used in server applications that need to continuously listen for incoming connections:

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    loop {
        match listener.accept() {
            Ok((_socket, addr)) => println!("New connection: {}", addr),
            Err(e) => println!("Error accepting connection: {}", e),
        }
    }
}
```


The server continuously listens for incoming TCP connections on port 8080.

### Reading from a Stream

Infinite loops can be used to continuously read from a data stream, such as reading sensor data:

```rust
fn main() {
    let mut sensor_data = vec![1, 2, 3, 4, 5].into_iter();

    loop {
        match sensor_data.next() {
            Some(data) => println!("Sensor data: {}", data),
            None => break,
        }
    }
}
```

Here, the loop reads sensor data until the data source is exhausted.

### Game Loop

Many video games use an infinite loop to keep the game running and updating frames:

```rust
fn main() {
    loop {
        // Process user input
        // Update game state
        // Render frame

        // Example of a break condition
        if should_exit() {
            break;
        }
    }
}

fn should_exit() -> bool {
    // Replace with actual exit condition
    false
}
```

The game loop processes user input, updates the game state, and renders frames continuously until an exit condition is met.
