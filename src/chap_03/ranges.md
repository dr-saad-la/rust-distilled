<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Ranges in Rust
</div>

## Ranges in Rust

In Rust, ranges help represent sequences of values, making it easy to loop over numbers or define slices of arrays. They offer a clear and straightforward way to specify both the start and end points of a sequence, as well as the steps between values. Ranges are commonly used in loops, iteration, and pattern matching. This feature is essential in Rust, allowing developers to write cleaner and more efficient code.


## Types of Ranges in Rust

In Rust, ranges can be categorized into three distinct types:

1. **Exclusive Ranges**
2. **Inclusive Ranges**
3. **Ranges with Step Values**

### Exclusive Range

An exclusive range includes the **start** value but **excludes the end value**, making it ideal for iterating up to, but not including, a specific endpoint. You can create an exclusive range using the `..` syntax.

```rust
for i in 0..5 {
    println!("{}", i);
}
```
Here is an example: 


```Rust
fn main(){
    for i in 0..5 {
        println!("{}", i)
    }
}
main();
```

    0
    1
    2


### Inclusive Range

An inclusive range includes both the start and end values. It is created using the `..=` syntax.

```rust
for i in 0..=5 {
    println!("{}", i)
}
```

Here is an example:


```Rust
fn main(){
    for i in 0..=5 {
    println!("{}", i);
}
}
main();
```

    3
    4
    0
    1
    2


### Range with a Step Value

You can create a range with a specific step value using the `step_by` method. This allows you to define the increment between each value in the range, providing greater control over iteration.

```rust
for i in (0..5).step_by(2) {
    println!("{}", i);
}
```

**Example**:


```Rust
fn main(){
    for i in (0..5).step_by(2) {
        println!("{}", i)
    }
    
    // another example
    println!("{}", "=".repeat(10));
    for j in (0..10).step_by(3){
        println!("{}", j)
    }
}
main();
```

    3
    4
    5
    0
    2
    4


### Range of Characters in Rust

In Rust, you can create ranges of characters just like numerical ranges. This is useful for iterating over sequences of characters in the ASCII or Unicode ranges.

#### Example of Character Range

You can use the `..` and `..=` syntax to create ranges of characters:

```rust
for c in 'a'..='e' {
    println!("{}", c);
}
```


```Rust
fn main(){
    for c in 'a'..='e' { // inclusive the `e` character
        println!("{}", c)
    }
}
main();
```

    ==========
    0
    3
    6
    9
    a
    b
    c


This example iterates over the characters from 'a' to 'e', inclusive.

### Using Ranges in Patterns
Character ranges can also be used in pattern matching:

rust
Copy code



```Rust
fn main(){
    let ch = 'c';

    match ch {
        'a'..='z' => println!("Lowercase letter"),
        'A'..='Z' => println!("Uppercase letter"),
        _ => println!("Other character"),
    }
}

main();
```

    d
    e
    Lowercase letter


This match statement checks if the character falls within a range of lowercase or uppercase letters.

### Creating a Range from an Array, Vector or Slice:

- You can create a range from an array, vector or slice using the iter method.

```rust
let v = vec![1, 2, 3, 4, 5];
for i in v.iter() {
    println!("{}", i);
}
```

**Example**:


```Rust
fn main(){
    let v = vec![1, 2, 3, 4, 5];
    for i in v.iter() {
        println!("{}", i);
    }
}
main();
```

    1
    2
    3
    4
    5


### Range of Reversed Values Using the `rev` method

- The `rev` method can be used to create a range that iterates in reverse order:

```rust
for i in (0..5).rev() {
    println!("{}", i);
}
```

**Example**


```Rust
// Example of using .rev method
fn main(){
    for i in (0..5).rev() {
    println!("{}", i);
    }
}
main();
```

    4
    3
    2
    1
    0


## Creating a Range from an Array or a Vector:
You can create a range from an array or a vector using the `iter` method.


```Rust
fn main(){
    let vec = vec![1, 2, 3, 4, 5];
    for i in vec.iter() {
        println!("{}", i);
    }
}
main();
```

    1
    2
    3
    4
    5


### Using the `enumerate` method:
- The `enumerate` method can be used to create a range that also includes the index of each item:

```rust
let v = vec![1, 2, 3, 4, 5];
for (i, item) in v.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

**Example**


```Rust
// Example of using iter
fn main(){
    let v = vec![1, 2, 3, 4, 5];
    for (i, item) in v.iter().enumerate() {
        println!("{}: {}", i, item);
    }
}
main();
```

    0: 1
    1: 2
    2: 3
    3: 4
    4: 5


### Using the `filter` method:

The `filter` method can be used to create a range that only includes items that meet a certain condition.

```rust
let v = vec![1, 2, 3, 4, 5];
for item in v.iter().filter(|&x| x % 2 == 0) {
    println!("{}", item);
}
```


```Rust
fn main(){
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for item in v.iter().filter(|&x| x % 2 == 0) {
        println!("{}", item);
    }
}
main();
```

    2
    4
    6
    8
    10


### Using the `map` method:

The `map` method can be used to create a range that includes the results of applying a function to each item.

```rust
let v = vec![1, 2, 3, 4, 5];
let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
println!("{:?}", v2);
```


```Rust
fn main(){
    let v = vec![1, 2, 3, 4, 5];
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}
main();
```

    [2, 3, 4, 5, 6]


### Infinite Range:
You can create an infinite range using the `std::iter::repeat` function or the `cycle` method:


```Rust
// Using std::iter::repeat

fn main(){
    for i in std::iter::repeat(1).take(5) {
    println!("{}", i);
    }
    
    println!("{}", "*".repeat(7));
    
    // Using cycle
    let vec = vec![1, 2];
    let mut iter = vec.iter().cycle();
    for _ in 0..5 {
        println!("{}", iter.next().unwrap());
    }
}
main();
```

    1
    1
    1
    1
    1
    *******
    1
    2
    1
    2
    1


## Range of Floating Point Numbers:
Rust does not support creating a range of floating point numbers directly. However, you can use the `std::iter::successors` function to achieve this:


```Rust
fn main(){
    let f64_iter = std::iter::successors(Some(0.0), |x| Some(x + 0.1));
    for i in f64_iter.take(5) {
        println!("{}", i);
    }
}
main();
```

    0
    0.1
    0.2
    0.30000000000000004
    0.4

## Summary
- In Rust, ranges provide a powerful and flexible way to work with sequences of values. Whether iterating over numbers, characters, or applying complex operations with methods like `step_by`, `rev`, `enumerate`, `filter`, and `map`, ranges make the code more readable and efficient. 
- Understanding and utilizing these different types of ranges will enhance the ability to write robust and concise Rust programs.
