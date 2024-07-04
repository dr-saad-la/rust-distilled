<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Control Flow in Rust: If-Else Expression Rust
</div>

## Introduction to Statements and Expressions

In Rust, understanding the distinction between statements and expressions is crucial for mastering the language's control flow constructs. 

### Statements vs Expressions

- **Expressions** evaluate to a value and can be used wherever a value is expected.
- **Statements** perform actions but do not return values.

Examples:
- **Expression**: `5 + 3`
- **Statement**: `let x = 5;`

### Rust's Approach to Conditionals

Rust's handling of conditionals emphasizes its expression-oriented nature. Unlike some languages where `if` constructs are purely statements, in Rust, they are expressions and can be used in places where a value is expected.

#### Example:

```rust
let x = if true { 1 } else { 0 };
println!("{}", x); // Outputs: 1
```

### Similarity to LISP

Rust’s treatment of conditionals as expressions is similar to LISP, where almost everything is an expression and can return a value.

Example in LISP:

```lisp
(setq x (if (> 3 2) 1 0))
(print x) ; Outputs: 1
```

In both Rust and LISP, if constructs return values, allowing them to be used in variable assignments and other expressions.

## Difference Between Rust and C
In contrast, C treats conditionals as statements. The if construct in C does not return a value, and thus, cannot be used directly in assignments.

Example in C:
```c
int x;
if (true) {
    x = 1;
} else {
    x = 0;
}
printf("%d\n", x); // Outputs: 1
```

In C, the conditional logic requires separate assignment statements within the if and else blocks, unlike Rust, where the if-else construct itself returns a value.

## If Else Expression in Rust

In Rust, `if-else` expressions provide a powerful mechanism for implementing conditional logic in a concise and expressive manner. Unlike many other programming languages where `if` constructs are solely statements that do not return values, Rust treats them as expressions that evaluate to a value. This unique feature allows `if-else` constructs to be seamlessly integrated into variable assignments and other expressions, leading to cleaner and more readable code. 

When the `if-else`  is used in assignments, these are sometimes referred to as "let bindings."

By leveraging `if-else` expressions, developers can perform conditional checks and immediately use the results within their code. This not only reduces the need for additional variable assignments but also makes the code flow more intuitive and easy to follow. The ability to combine `if`, `else if`, and `else` branches within a single expression further enhances the flexibility and power of Rust's conditional logic capabilities.

The following sections will explore how to effectively use `if-else` expressions in Rust, highlighting their syntax and demonstrating their practical applications. Through these examples, you will gain a deeper understanding of how to use Rust's conditional expressions to write robust and efficient code.

## General Syntax of If-Else Expressions


Here is the general syntax of `if-else` expression in Rust:

```rust
let var = if condition {
        // code to execute if condition is true
    } else {
        // code to execute if condition is false
}
```

and the extended syntax with `else-if` branch:

```rust
let var = if condition1 {
        // code to execute if condition1 is true
    } else if condition2 {
        // code to execute if condition2 is true
    } else {
        // code to execute if none of the above conditions are true
}
```

### Using Conditions with `let Bindings`

Rust allows you to use `if`, `else`, and `else if` conditions in combination with `let` bindings for more concise and expressive code.


```rust
let my_var = if cond {result} else {the_other_result} 
```

## Example of Using If-Else Expression

Here is a simple example demonstrating the use of an `if-else` expression in Rust:


```Rust
fn main() {
    let cond = true;

    let my_var = if cond { 
        "Condition is true" 
    } else { 
        "Condition is false" 
    };

    println!("{}", my_var);
}
main();
```

    Condition is true


This example shows how you can directly assign the result of an `if-else` expression to a variable using let bindings. This approach makes the code more concise and readable.

Here is another example showing how to use condition with let bindings:


```Rust
fn main() {
    let temperature = 30;

    let status = if temperature > 25 {
        "Hot"
    } else {
        "Cold"
    };

    println!("The weather is {}", status); 
}
main();
```

    The weather is Hot


The `if-else` expression evaluates whether the temperature is greater than 25. If the condition is `true`, the expression evaluates to "Hot". Otherwise, it evaluates to "Cold". The result of this expression is then assigned to the variable status, which is printed out to display the weather status.

## Advanced Example with `let` Binding

The entire code block of a conditional expression, including various types of `if` condition arms such as `if`, `else`, and `else if`, can be used within a `let` binding. This allows for more concise and expressive code by directly assigning the result of the conditional expression to a variable.

 Here's an example to illustrate how to use this full syntax within a let binding:


```Rust
fn main() {
    let temperature = 10;

    let weather_status = if temperature > 30 {
        "Hot"
    } else if temperature >= 20 {
        "Warm"
    } else if temperature >= 10 {
        "Cool"
    } else {
        "Cold"
    };

    println!("The weather is {}", weather_status); 
}
main();
```

    The weather is Cool


This example demonstrates how you can use `if, else if, and else` conditions within a let binding to handle multiple conditions concisely and expressively.

The example is self-explanatory, however, here is the details of how it works

- Condition: The `if expression` checks if temperature is greater than 30. If true, it assigns "Hot" to weather_status. This condition is not `true`, so it will not execute
- `Else If`: If the first condition is false, it checks if temperature is greater than or equal to 20. If true, it assigns "Warm" to weather_status. The first condition was indeed `false`, but this condition is also `false`, so it won't execute.
- `Another Else If`: If the previous conditions are false, it checks if temperature is greater than or equal to 10. If true, it assigns "Cool" to weather_status. All previous conditions were `false` but this one is `true`, so the value will be assigned to `weather_status` variable and the next conditions won't be checked.
- Else: If none of the above conditions are true, it assigns "Cold" to weather_status. This condition is not checked since the previous condition was `true` and program won't reach this point.

The program prints the values and terminates.

## Summary
- **Statements**: Perform actions, do not return values.
- **Expressions**: Evaluate to values and can be used wherever values are expected.
- Rust's if-else constructs are expressions, allowing them to return values and be used in assignments, similar to LISP.
- This differs from C, where conditionals are statements and do not return values.
- Using conditions with let bindings in Rust allows for more concise and expressive code, enhancing readability and maintainability.

