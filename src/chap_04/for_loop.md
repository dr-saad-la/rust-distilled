<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Control flow with `for` loop
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar); 
    let message = format!("{:^width$}", message, width = nchar); 
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

## Iterating with `for .. in`

The `for` loop is another powerful and versatile control flow construct in Rust that allows for iterating over collections or ranges of values in a concise and readable manner. It is the most common and idiomatic way to perform iterations in Rust, providing a robust mechanism for traversing arrays, vectors, slices, and other iterable data structures.

### Introduction to `for` Loops

The `for` loop simplifies the process of iterating over collections or ranges by automatically handling the iteration logic. This not only makes the code more readable and less error-prone but also leverages Rust's strong type system and ownership model to ensure safety and performance. The `for` loop is preferred for most iteration tasks due to its clear syntax and ability to work seamlessly with Rust's iterators.

### Syntax of `for` Loop

The general syntax of a `for` loop in Rust is as follows:

```rust
for item in collection {
    // Code to execute for each item in the collection
}
```

## The Mechanism of `for` Loop in Rust

1. **Initialization**: The loop initializes an iterator for the specified collection or range.
2. **Iteration**: For each iteration, the loop retrieves the next item from the iterator and executes the loop body with this item.
3. **Termination**: The loop continues until the iterator is exhausted, meaning there are no more items to retrieve.

## Advantages of `for` Loops

- **Readability**: The syntax of for loops is clean and expressive, making it easy to understand the intent of the iteration.
- **Safety**: Rust's ownership and borrowing rules ensure that `for` loops do not cause data races or invalid memory access.
- **Efficiency**: The Rust compiler optimizes `for` loops to minimize overhead, making them highly efficient for iterating over large collections.
- **Flexibility**: for loops can be used with a wide variety of iterable structures, including arrays, vectors, slices, and custom iterators.


### Example: Iterating Over a Range (Inclusive)

The `for` loop in Rust can be effectively used to iterate over a range of values. When specifying a range, you can use the inclusive range syntax (`..=`)  to ensure that the upper bound is included in the iteration This is particularly useful when you need to include both the start and end values in your loop. Or you can use  the non-inclusive range using this syntax (`..`), remember that when using the non-exlusive syntax, the last item won't be included. 

#### Example

Let us consider the following example where we iterate over an inclusive range of integers from 1 to 5:


```Rust
fn main() {
    banner("*", 62, "Iterating over a range of integer values using for loop");
    
    // Use an inclusive range to iterate from 1 to 5
    for number in 1..=5 {
        println!("The number is: {}", number);
    }
    
    println!("Loop ended here");
    println!("{}", "*".repeat(62));
}
main();
```

    
    **************************************************************
       Iterating over a range of integer values using for loop    
    **************************************************************
    The number is: 1
    The number is: 2
    The number is: 3
    The number is: 4
    The number is: 5
    Loop ended here
    **************************************************************


### Code in Details

#### Initialization:

- The `for` loop initializes an iterator over the inclusive range `1..=5`. This range includes all integers from 1 to 5, both endpoints inclusive.

#### Iteration:

- The loop begins iterating over the range. In each iteration, the `number` variable takes on the value of the current item from the range.

#### Loop Body:

- The code inside the loop body (`println!("The number is: {}", number);`) executes for each value in the range. This statement prints the current value of `number` to the console.

#### Termination:

- The loop continues to iterate until all values in the range have been processed. In this example, it iterates five times, corresponding to the values 1, 2, 3, 4, and 5.

### Detailed Walkthrough

1. **First Iteration**:
   - `number = 1`
   - The loop body prints: `The number is: 1`

2. **Second Iteration**:
   - `number = 2`
   - The loop body prints: `The number is: 2`

3. **Third Iteration**:
   - `number = 3`
   - The loop body prints: `The number is: 3`

4. **Fourth Iteration**:
   - `number = 4`
   - The loop body prints: `The number is: 4`

5. **Fifth Iteration**:
   - `number = 5`
   - The loop body prints: `The number is: 5`

After the fifth iteration, the range is exhausted, and the loop terminates.

If we were to iterate over a non-inclusive range, the end point would not be included in the iteration. Here is the same example as above, but with non-inclusive syntax:


```Rust
fn main() {
    banner("*", 62, "Iterating over a range of integer values using for loop");
    
    // Use an inclusive range to iterate from 1 to 5
    for number in 1..5 {
        println!("The number is: {}", number);
    }
    
    println!("Loop ended here");
    println!("{}", "*".repeat(62));
}
main();
```

    
    **************************************************************
       Iterating over a range of integer values using for loop    
    **************************************************************
    The number is: 1
    The number is: 2
    The number is: 3
    The number is: 4
    Loop ended here
    **************************************************************


As we can observe, the value `5` is not included in the output.


### Example


```Rust
fn main() {
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("The value is: {}", element);
    }
}
main();
```

    The value is: 10
    The value is: 20
    The value is: 30
    The value is: 40
    The value is: 50


### Example: Iterating Over a Range of Characters

We can also iterate over a range of characters using the `for` loop. This is particularly useful when you need to perform operations on a sequence of characters in a specific range. The syntax for character ranges is similar to that of numerical ranges.

#### Example

Consider the following example where we iterate over an inclusive range of characters from `'a'` to `'e'`:


```Rust
fn main() {
    banner("*", 52, "Iterating over a range of characters example");
    for c in 'a'..='e' { // inclusive e
        println!("Char: {}", c);
    }
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
        Iterating over a range of characters example    
    ****************************************************
    Char: a
    Char: b
    Char: c
    Char: d
    Char: e
    ****************************************************


### Example: Iterating Over an Array

The `for` loop can be used to iterate over the elements of an array. This is a common use case for `for` loops, as it allows for concise and readable traversal of array elements.

#### Example

Consider the following example where we iterate over an array of integers:


```Rust
fn main() {
    banner("*", 52, "Iterating over an array");
    let numbers = [10, 20, 30, 40, 50];

    for number in numbers {
        println!("The number is: {}", number);
    }
    
    println!("{}", "+".repeat(52));
}
main();
```

    
    ****************************************************
                  Iterating over an array               
    ****************************************************
    The number is: 10
    The number is: 20
    The number is: 30
    The number is: 40
    The number is: 50
    ++++++++++++++++++++++++++++++++++++++++++++++++++++


### Example: Iterating Over a Vector

Another collection the `for` loop can also be used with to iterate over the elements of a vector. Vectors are similar to arrays but are dynamically sized, making them a common choice for collections where the number of elements may change.

#### Example

Consider the following example where we iterate over a vector of integers:


```Rust
fn main() {
    banner("*", 52, "Iterating over a vector");
    let numbers = vec![10, 20, 30, 40, 50];

    for number in &numbers {
        println!("The number is: {}", number);
    }
    println!("{}", "+".repeat(52));
    // println!("{:?}", numbers);             if we didn't use `&numbers`, this would cause a compile error.
}
main();
```

    
    ****************************************************
                  Iterating over a vector               
    ****************************************************
    The number is: 10
    The number is: 20
    The number is: 30
    The number is: 40
    The number is: 50
    ++++++++++++++++++++++++++++++++++++++++++++++++++++


Notice that with vectors, the `&numbers` syntax borrows the vector, allowing us to iterate over its elements without taking ownership. Ownership will be discussed in a different chapter in more detial.

## Nested for Loops
In Rust, nested for loops allow you to iterate over multiple collections or ranges simultaneously. This is useful when you need to perform operations that involve combinations of elements from different collections or when working with multidimensional data structures.

**Syntax of Nested for Loops**

The general syntax for nested for loops is straightforward, with one for loop placed inside another. Each loop can iterate over its respective collection or range.

```rust
for item1 in collection1 {
    for item2 in collection2 {
        // code to execute for each combination of item1 and item2
    }
}
```

### Example: Nested for Loop with Ranges
Consider the following example where we use nested for loops to iterate over two ranges:


```Rust
fn main() {
    banner("*", 52, "Nested for loop");
    for i in 1..=3 {
        for j in 1..=3 {
            println!("i: {}, j: {}", i, j);
        }
    }
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                      Nested for loop                   
    ****************************************************
    i: 1, j: 1
    i: 1, j: 2
    i: 1, j: 3
    i: 2, j: 1
    i: 2, j: 2
    i: 2, j: 3
    i: 3, j: 1
    i: 3, j: 2
    i: 3, j: 3
    ****************************************************


- In this example:
    - The outer loop initializes an iterator over the inclusive range 1..=3, iterating over the values 1, 2, and 3.
    - For each iteration of the outer loop, the inner loop initializes its own iterator over the inclusive range 1..=3.
    - The outer loop continues until all values in its range have been processed. For each value of the outer loop, the inner loop runs to completion before the outer loop proceeds to the next value.




### Conclusion

In this section, we explored the various ways to use the `for` loop in Rust to iterate over different types of collections. The `for` loop is a powerful and flexible control flow construct that allows for concise and efficient iteration over arrays, vectors, ranges, characters, and other data structures.

- We began with a detailed introduction to the `for` loop, highlighting its syntax and advantages. We then provided examples of using `for` loops with inclusive and non-inclusive ranges, demonstrating how to iterate over sequences of numbers and characters. 

- We also covered the use of `for` loops with arrays and vectors, which are commonly used collections in Rust. By iterating over these collections, we can perform operations on each element in a straightforward and readable manner.



