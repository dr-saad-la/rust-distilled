<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   The if statements
</div>


The `if` statement allows you to execute a block of code only if a specified condition is true.

### Syntax

```rust
if condition {
    // code to execute if condition is true
}
```

Note that the curly braces `{}` are required even if there is only one statement.

### Example

Here is a simple example:

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

## Using Curly Braces in Rust if Statements

The syntax for `if` statement always requires curly braces `{ }` around the block of code that follows the condition. This rule applies to all if statements, regardless of the number of statements in the block. In other words, even if the block contains only one statement or expression, the curly braces are still mandatory. This requirement helps maintain consistency and readability in the code.

The reason this requirement is to avoid ambiguity and potential errors. By enforcing the use of curly braces, Rust ensures that the code is explicit and clear, which can help prevent mistakes that could arise from misunderstanding the scope of the if statement. In fact, I am in favor of this approach, even with languages that do not enforce this rule, I see that it is best practice to add the curly braces in case of adding new statements. 

### Example Without Curly Braces

Consider the following example where we try to omit the curly braces for an if statement with a single expression:


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

In order for the previous example to compile, you must add the curly braces, even if it contains only one statement. This is different than other programming languages.

## Parentheses in Rust `if` Statements

Parentheses around the condition in an `if` statement are optional and typically omitted. Unlike some other programming languages where parentheses are required, Rust allows you to write cleaner and more concise conditional expressions without them. Using parentheses is not necessary and may trigger compiler warnings for unnecessary syntax. Even the conditions do not need to be enclosed in parentheses, although you can use them for clarity if desired but you should use the `#[allow(unused_parens)]` attribute.

### Example Without Parentheses (Preferred)


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


### Example With Parentheses (Discouraged)
While the following code is syntactically correct, it is not idiomatic Rust and may produce compiler warnings:


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


In Jupyter Notebook, no warnings are displayed when using parentheses around conditions in if statements. This behavior is likely due to differences in how the Rust kernel in Jupyter handles linting and warnings. Jupyter Notebook may suppress warnings by default to provide a cleaner and more user-friendly experience, especially for educational purposes. However, if you run the same code in a traditional text editor or integrated development environment (IDE) that supports Rust, you will receive warnings about the unnecessary use of parentheses. This discrepancy highlights how different development environments handle compiler messages.

## Conditions Must Be Boolean

In Rust, conditions in `if` statements must explicitly evaluate to a boolean value (`true` or `false`). Unlike some other programming languages, Rust does not implicitly convert non-boolean types such as integers or strings to boolean values. This design decision ensures that conditions are always clear and unambiguous.

### Example of Invalid Conditions

The following examples will produce errors because the conditions are not explicitly boolean:

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

Here is the error

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

To correct these errors, you need to explicitly compare the values to obtain a boolean expression:


```Rust
fn main() {
    let number = 5;

    if number != 0 {
        println!("The number is not zero.");
    }
}
main();
```

    The number is not zero.


**Note**

> - number != 0 explicitly checks if number is not zero, resulting in a boolean value.

By using explicit comparisons, you ensure that the conditions in your `if` statements are always boolean expressions, making your code clear and preventing potential errors.

### Implicit Checking Against `0`

You cannot implicitly check if a number is `zero` in an `if` statement without an explicit comparison. Attempting to do so will result in a compilation error because Rust expects a boolean expression in the condition.

Here is an example that demonstrates this error:

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

This error occurs because if statements in Rust require a boolean expression. The variable num is an integer, and Rust does not implicitly convert integers to booleans.

To correctly check if a number is zero, you need to explicitly compare it to 0:


```Rust
// Here is the correct version
fn main() {
    let num = 0; 
    if num == 0 {
        println!("The number is zero.");
    }
}
main();
```

    The number is zero.


## Checking Against Empty Strings

In Rust, you cannot directly use a string as a condition in an `if` statement because the condition must be a boolean expression. Instead, you need to explicitly check whether the string is empty or not. Rust provides methods like `is_empty` to perform this check.

### Example of Invalid Condition

Attempting to use a string directly in an `if` statement will result in an error:

```rust
fn main() {
    let text = "";

    // Error: expected `bool`, found `&str`
    if text {
        println!("This will not compile.");
    }
}
```

Here is the error from the previous code:

```text
[E0308] Error: mismatched types
    ╭─[command_7:1:1]
    │
 5  │     if text {
    │        ──┬─  
    │          ╰─── expected `bool`, found `&str`
────╯
```

To properly check if a string is empty, you can use the `is_empty` method, which returns `true` if the string is empty and `false` otherwise:


```Rust
// correct code
fn main() {
    let text = "";

    if text.is_empty() {
        println!("The text is empty.");
    } else {
        println!("The text is not empty.");
    }
}
main();
```

    The text is empty.


In this example, `text.is_empty()` checks if text is empty and returns a boolean value that can be used in the if statement.

### Correct Example: Checking if a String is Not Empty
If you want to check if a string is not empty, you can use the ! operator with the is_empty method:


```Rust
fn main() {
    let text = "Hello, world!";

    if !text.is_empty() {
        println!("The text is not empty.");
    } else {
        println!("The text is empty.");
    }
}
main();
```

    The text is not empty.


In this example, `!text.is_empty()` checks if text is not empty and returns a boolean value that can be used in the if statement.

## The else Statements

The `else` statement in Rust provides a way to specify a block of code that will execute if the condition in the preceding if statement evaluates to false. This allows your program to handle alternative cases and execute different code paths based on the evaluation of conditions.

**Basic Syntax**
The basic syntax for using an else statement is as follows:

```rust
if condition {
    // Code to execute if condition is true
} else {
    // Code to execute if condition is false
}
```

**Example** 
Here's a simple example to demonstrate the usage of else:


```Rust
fn main() {
    let number = -42;

    if number > 0 {
        println!("The number is positive.");
    } else {
        println!("The number is zero or negative.");
    }
}
main();
```

    The number is zero or negative.


In this example:
 - The if statement checks if number is greater than 0.
 - If the condition is `false`, so the condition is not executed
 - The second condition is `true`, so the else block executes and prints "The number is zero or negative."

### Summary

- if Statement: Executes a block of code if a specified condition is true.
- The curly braces are required regardless of the number of statements in the if code block.
- The parentheses are optional and not preferred around the conditions.
- else Statement: Specifies a block of code to execute if the if condition is false.


