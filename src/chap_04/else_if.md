<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   The `else if` Statements
</div>

The `else if` statement allows you to specify a new condition to check if the previous if condition is false. This helps in checking multiple conditions sequentially.

- **Syntax**

```rust
if condition1 {
    // code to execute if condition1 is true
} else if condition2 {
    // code to execute if condition2 is true
} else {
    // code to execute if none of the above conditions are true
}
```

### Example


```Rust
fn main() {
    let number = 0;

    if number > 0 {
        println!("The number is positive.");
    } else if number < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}
main();
```

    The number is zero.


- In this example:
    - The first condition number > 0 is false.
    - The second condition number < 0 is also false.
    - Since both conditions are false, the code inside the else block is executed, printing "The number is zero."

### Using Conditions with `let Bindings`

- Rust allows you to use if, else, and else if conditions in combination with let bindings for more concise and expressive code.

```rust
let my_var = if cond {result} else {the_other_result} 
```

### Example

```Rust
fn main() {
    let number = 5;
    let is_positive = if number > 0 {
        true
    } else {
        false
    };

    println!("Is the number positive? {}", is_positive);
}
main();
```

    Is the number positive? true

In this example, the variable is_positive is assigned the value true if number > 0 and false otherwise. This is a concise way to handle simple conditions and bindings.

### Summary

- else if Statement: Allows for checking multiple conditions sequentially.
- Using Conditions with let Bindings: Enables concise and expressive code by combining conditions with variable bindings.

### Example Combining All Concepts


```Rust
fn main() {
    let number = 10;

    let description = if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    };

    println!("The number is {} and it is {}.", number, description);
}
main();
```

    The number is 10 and it is positive.


In this comprehensive example, the description variable is assigned a string based on the value of number, and the final message prints out the number and its description. This demonstrates the flexibility and power of Rust's control flow structures.
