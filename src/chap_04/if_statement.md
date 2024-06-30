<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   The if statement
</div>


The `if` statement allows you to execute a block of code only if a specified condition is true.

### Syntax

```rust
if condition {
    // code to execute if condition is true
}
```

- The curly braces `{}` are required even if there is only one statement.

### Example


```Rust
fn main() {
    let number = 5;

    if number > 0 {
        println!("The number is positive.");
    }
}
main();
```

    The number is positive.

In this example, the condition number > 0 is true, so the code inside the if block is executed, printing "The number is positive."

```rust
// Trying to remove the curly braces
fn main() {
    let number = 5;

    if number > 0 
        println!("The number is positive.");
    
}
main();
```

- This will produce an error:

```text
Error: expected `{`, found `println`
   ╭─[command_9:1:1]
   │
 5 │     if number > 0
   │        ─────┬────  
   │             ╰────── note: the `if` expression is missing a block after this condition
 6 │         println!("The number is positive.");
   │         ───┬───  
   │            ╰───── expected `{`
───╯
```


```Rust
// adding parentheses
fn main() {
    let number = 5;

    if (number > 0) {
        println!("The number is positive.");
    }
}
main();
```

    The number is positive.


- In jupyter notebook, no warnings are shown, I am not sure the reason why, however, if you run in text editor you will get warnings for using parentheses.

## Conditions Must Be Boolean

In Rust, conditions must explicitly evaluate to a boolean value (`true` or `false`). Unlike some other programming languages, you cannot implicitly compare values against `0`, empty strings, or other non-boolean types.

### Example of Invalid Conditions

- The following examples will produce errors because the conditions are not explicitly boolean:

```rust
fn main() {
    let number = 5;

    // Error: expected `bool`, found integer
    if number {
        println!("This will not compile.");
    }

    let text = "";

    // Error: expected `bool`, found `&str`
    if text {
        println!("This will not compile either.");
    }
}
```

- Here is the error from the previous code

```text
[E0308] Error: mismatched types
   ╭─[command_7:1:1]
   │
 5 │     if number {
   │        ───┬──  
   │           ╰──── expected `bool`, found integer
───╯
[E0308] Error: mismatched types
    ╭─[command_7:1:1]
    │
 12 │     if text {
    │        ──┬─  
    │          ╰─── expected `bool`, found `&str`
────╯
```

### Implicit Checking Against `0`

- If we try to implicity check against `0`, we will get an error. 
```rust
fn main() {
    let num = 0; 
    if num {
        println!("Something");
    }
}
main();
```

- If you run this code, you will get the following error:

```text
[E0308] Error: mismatched types
   ╭─[command_19:1:1]
   │
 3 │     if num {
   │        ─┬─  
   │         ╰─── expected `bool`, found integer
───╯
```

### No Parentheses Needed

- In Rust, conditions do not need to be enclosed in parentheses, although you can use them for clarity if desired.


```Rust
fn main() {
    let number = 5;

    // Both styles are valid
    if number > 0 {
        println!("No parentheses needed.");
    }

    if (number > 0) {
        println!("Parentheses are optional.");
    }
}
main();
```

    No parentheses needed.
    Parentheses are optional.


### The `else` Statements

- The else statement allows you to specify a block of code to execute if the if condition is false.

- **Syntax**
```rust
if condition {
    // code to execute if condition is true
} else {
    // code to execute if condition is false
}
```

### Example


```Rust
fn main() {
    let number = -5;

    if number > 0 {
        println!("The number is positive.");
    } else {
        println!("The number is not positive.");
    }
}
main();
```

    The number is not positive.


- In this example, the condition number > 0 is false, so the code inside the else block is executed, printing "The number is not positive."





