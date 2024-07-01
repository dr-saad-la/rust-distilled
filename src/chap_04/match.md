<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   The match Statement in Rust
</div>

The `match` statement in Rust is a powerful control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches. It is similar to **switch** statements in other languages but much more powerful and flexible.

- The `match` statement is used to match:
    - **Integers**.
    - **Booleans**.
    - **Enums**.
    - **Arrays**.
    - **Tuples**.
    - **Structs** (Like classes in other programming languages).

## Basic Syntax

- The basic syntax of a `match` statement involves a value to match against:
    - A series of match statements called arms enclosed in curly braces `{...}`.  
    - Patterns followed by the code to execute if the pattern matches.
    - The arms must be exhaustive


```Rust
fn main() {
    let num = 3;

    match num {
        1 => println!("One!"),                // This is called an arm
        2 => println!("Two!"),                // This is called an arm
        3 => println!("Three!"),              
        _ => println!("Something else!"),    // This is the catch-All statement (or the default value)
    }
}
main();
```

    Three!

- **Details of the match components**

1. `match num { ... }`:
    - This is the match statement itself. It takes the value num and compares it against a series of patterns enclosed in curly braces {}.
2. Arms:
    - Each comparison in a match statement is called an arm. Each arm consists of a pattern, the => symbol, and the code to execute if the pattern matches.

3. **Patterns (1, 2, 3, _):**
    * 1: This is a pattern that matches if num is 1.
    * 2: This is a pattern that matches if num is 2.
    * 3: This is a pattern that matches if num is 3.
    * _: This is a wildcard pattern that matches any value not matched by the previous patterns. It is often used as a catch-all or default case.

4. `=>:`
    - The => symbol separates the pattern from the code that should be executed if the pattern matches. It can be read as "if this pattern matches, then do this".
5. The **Expressions**: The expressions that get executed when a pattern matches.
    * println!("One!"): Executes if num is 1.
    * println!("Two!"): Executes if num is 2.
    * println!("Three!"): Executes if num is 3.
    * println!("Something else!"): Executes if num is any value not covered by the previous patterns.

### Match Arms

- **Patterns:** Patterns can be literals, variables, wildcards, and more.
- **Arms:** Each arm consists of a pattern and an expression to evaluate if the pattern matches.
- **Wildcard:** The `_` pattern matches any value not matched by previous arms.

### Matching Literals
- You can match literal values like integers, characters, and strings.


```Rust
fn main() {
    let x = 5;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"),
    }
}
main();
```

    Five


### Matching Ranges
- You can match ranges of values using the `..=` syntax.
- In this case the **upper limit must be inclusive**.


```Rust
fn main() {
    let x = 7;
    
    match x {
        1..=5 => println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        _ => println!("Something else"),
    }
}
main();
```

    Between 6 and 10

// This code will not compile because the upper limit in the range is not inclusive
fn main() {
    let x = 7;
    
    match x {
        1..5 => println!("Between 1 and 5"),
        6..10 => println!("Between 6 and 10"),
        _ => println!("Something else"),
    }
}
main();// here is the error
[E0658] Error: exclusive range pattern syntax is experimental
   ╭─[command_20:1:1]
   │
 6 │         1..5 => println!("Between 1 and 5"),
   │         ──┬─  
   │           ╰─── error: exclusive range pattern syntax is experimental
───╯
[E0658] Error: exclusive range pattern syntax is experimental
   ╭─[command_20:1:1]
   │
 7 │         6..10 => println!("Between 6 and 10"),
   │         ──┬──  
   │           ╰──── error: exclusive range pattern syntax is experimental
───╯
[non_contiguous_range_endpoints] Error: multiple ranges are one apart
   ╭─[command_20:1:1]
   │
 6 │         1..5 => println!("Between 1 and 5"),
   │         ──┬─  
   │           ╰─── help: use an inclusive range instead: `1_i32..=5_i32
## Matching Multiple Patterns

In Rust, you can match multiple patterns in a single match arm using the `|` (pipe) operator. This allows you to simplify your `match` statements by combining several patterns that should be handled in the same way.

### Syntax

```rust
match value {
    pattern1 | pattern2 | pattern2 => {
        // code to execute if value matches pattern1 or pattern2 or pattern3
    },
    _ => {
        // code to execute if value matches none of the above patterns
    },
}
```

### Example
- Here's an example that matches multiple patterns using the | operator:


```Rust
fn main() {
    let x = 1;

    match x {
        1 | 2 | 3 => println!("One, two or three"),
        4 => println!("Four"),
        _ => println!("Something else"),
    }
}
main();
```

    One, two or three


## Combining Patterns with Ranges
- You can combine multiple patterns including ranges:


```Rust
fn main() {
    let x = 5;

    match x {
        1 | 2 | 3 => println!("One, two, or three"),
        4..=6 => println!("Four, five, or six"),
        _ => println!("Something else"),
    }
}
main();
```

    Four, five, or six


## Matching with Conditions: (Pattern Guards)

- It is allowable to add extra conditions to the  `match` arms using **pattern guards**. A pattern guard is an `if` condition that goes after the pattern and before the `=>`. This allows you to match a pattern only if an additional condition is true.

### Syntax

```rust
match value {
    pattern if condition => {  // This if condition is called the match guard
        // code to execute if pattern matches and condition is true
    },
    _ => {
        // code to execute if value matches none of the above patterns
    },
}
```

### Example
Here's an example using a pattern guard:


```Rust
fn main() {
    let x = Some(4);

    match x {
        Some(n) if n < 5 => println!("Less than five: {}", n),
        Some(n) => println!("n is: {}", n),
        None => println!("No value"),
    }
}
main();
```

    Less than five: 4


- In this example:
    - If x is Some(n) and n < 5, it prints "Less than five: n".
    - If x is Some(n) and n is not less than 5, it prints "n is: n".
    - If x is None, it prints "No value".

### Example with Multiple Conditions

- You can also combine multiple conditions in a pattern guard:


```Rust
fn main() {
    let y = 10;

    match y {
        n if n % 2 == 0 && n > 5 => println!("Even and greater than five: {}", n),
        n if n % 2 == 0 => println!("Even: {}", n),
        n => println!("Odd: {}", n),
    }
}
main();
```

    Even and greater than five: 10



```Rust
fn main() {
    let y = 10;

    match y {
        n if n % 2 == 0 || n < 5 => println!("Even and greater than five: {}", n),
        n if n % 2 == 0 => println!("Even: {}", n),
        n => println!("Odd: {}", n),
    }
}
main();
```

    Even and greater than five: 10


- In this example:
    - If y is even and greater than 5, it prints "Even and greater than five: y".
    - If y is even but not greater than 5, it prints "Even: y".
    - For all other values of y (i.e., odd numbers), it prints "Odd: y".

### Example with Enums and Conditions

- Pattern guards are particularly useful with enums:


```Rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id } if id % 2 == 0 => println!("Even id: {}", id),
        Message::Hello { id } => println!("Odd id: {}", id),
    }
}
main();
```

    Odd id: 5


- In this example:
    - If the id is even, it prints "Even id: id".
    - If the id is odd, it prints "Odd id: id".

## Using `match` as an Expression

In Rust, `match` can be used as an expression, meaning it can return a value that can be assigned to a variable. This allows you to make decisions and compute values based on patterns in a concise and expressive way.

### Syntax

The value resulting from a `match` expression can be assigned directly to a variable.

```rust
let variable = match value {
    pattern1 => result1,
    pattern2 => result2,
    _ => default_result,
};
```

### Example
- Here's an example where match is used to determine a value based on different conditions:


```Rust
fn main() {
    let number = 5;

    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };

    println!("The number is:==> {}", description);
}
main();
```

    The number is:==> five


- In this example:
    * The match expression evaluates number and returns a string based on the pattern it matches.
    * The resulting string is assigned to the description variable.

### Using match with Enums
You can also use match with enums to return values based on different enum variants:


```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("The value of the coin is {} cents.", value);
}
main();
```

    The value of the coin is 10 cents.


- In this example:
    - The value_in_cents function uses a match expression to return the value of a coin in cents.
    - The match expression evaluates the variant of the Coin enum and returns the corresponding value.

### Using match to Compute Complex Values
You can use match expressions to compute more complex values based on patterns:


```Rust
fn main() {
    let num = Some(7);

    let doubled = match num {
        Some(n) if n > 0 => n * 2,
        Some(n) => n,
        None => 0,
    };

    println!("The doubled value is {}", doubled);
}
main();
```

    The doubled value is 14


- In this example:
    - The match expression checks if num is Some(n) and n is greater than 0, then doubles the value.
    - If n is not greater than 0, it returns the value as is.
    - If num is None, it returns 0.

### Chaining match Expressions
You can chain match expressions together to build more complex logic:


```Rust
fn main() {
    let number = 5;

    let result = match number {
        n if n % 2 == 0 => "even",
        _ => match number {
            1 => "one",
            3 => "three",
            5 => "five",
            _ => "odd",
        }
    };

    println!("The number is {}", result);
}
main();
```

    The number is five


- In this example:
    - The outer match checks if number is even.
    - If number is not even, the inner match provides specific names for some odd numbers and a generic "odd" for others.

## Destructuring Structs, Enums, and Tuples

- You can match and destructure complex data types like structs, enums, and tuples.

### `Structs`

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

### `Enums`


```Rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}
```

### Example with Enums Multiple Patterns
- You can also match multiple enum variants in a single match arm:


```Rust
enum Pet {
    Dog,
    Cat,
    Bird,
}

fn main() {
    let pet = Pet::Dog;

    match pet {
        Pet::Dog | Pet::Cat => println!("It's a common pet."),
        Pet::Bird => println!("It's a bird."),
    }
}
main();
```

    It's a common pet.


### `Tuples`


```Rust
fn main() {
    let pair = (0, -2);

    match pair {
        (0, y) => println!("First is zero and y is {}", y),
        (x, 0) => println!("x is {} and second is zero", x),
        _ => println!("It doesn't matter what they are"),
    }
}

main();
```

    First is zero and y is -2


### Binding Values
- You can bind matched values to variables using the `@` operator.


```Rust
fn main() {
    let x = 18;

    match x {
        n @ 1..=12 => println!("Got a number in the range 1 to 12: {}", n),
        n @ 13..=19 => println!("A teen of age: {}", n),
        _ => println!("Other age"),
    }
}
main();
```

    A teen of age: 18


## Using match with Enums

- The match statement is particularly powerful when used with enums, allowing you to handle each variant of the enum.


```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Value in cents: {}", value_in_cents(coin));
}
main();
```

    Value in cents: 10


## Exhaustiveness

- The match statement in Rust must be exhaustive, meaning all possible values must be covered. 
- If you do not cover all possibilities, the code will not compile. This ensures that you handle all potential cases.

### Example of Non-Exhaustive Match (Will Not Compile)

```rust
fn main() {
    let number = 5;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
    }
    // Error: non-exhaustive patterns: `3i32..=2147483647i32 | -2147483648i32..=0i32` not covered
}
main();
```

## Adding the Wildcard Arm


```Rust
fn main() {
    let number = 5;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Something else"),
    }
}

main();
```

    Something else


## Summary

- match Statement: Used for pattern matching, similar to switch statements in other languages but more powerful.
- Patterns: Can be literals, ranges, structs, enums, tuples, and more.
- Pattern Guards: Use if conditions in match arms to add extra conditions to patterns.
- Syntax: Place the if condition after the pattern and before the =>.
- Combining Conditions: You can combine multiple conditions in a single pattern guard.
- Use Cases: Pattern guards are useful for refining matches, especially with enums and complex data structures.
- Binding Values: Use @ to bind matched values to variables.
- Exhaustiveness: All possible values must be covered, ensuring that you handle all cases.
- Use the `|` operator to match multiple patterns in a single match arm.
- Combine different patterns, including literals, ranges, and enum variants, to simplify your match statements.
- Matching multiple patterns helps in handling similar cases together, making the code more concise and readable.


