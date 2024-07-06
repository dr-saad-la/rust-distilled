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


```Rust

```

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

### Iterating Over Other Collections
In Rust, the `for` loop can be used to iterate over various types of collections. Here are some additional examples:

### Iterating Over a Tuple

We can use the `for` loop to iterate over the elements of a tuple. However, since tuples do not implement the `IntoIterator` trait by default, you need to use a different approach. One common method is to convert the tuple into an array or a vector before iterating.

#### Example

Consider the following example where we iterate over a tuple of different types by converting it to an array:


```Rust
fn main() {
    banner("*", 52, "Iterating over a tuple");
    let tuple = (1, 2, 3, 4, 5);
    let array = [tuple.0, tuple.1, tuple.2, tuple.3, tuple.4];

    for value in &array {
        println!("Value: {}", value);
    }
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                   Iterating over a tuple               
    ****************************************************
    Value: 1
    Value: 2
    Value: 3
    Value: 4
    Value: 5
    ****************************************************


- **Iterating Over a HashMap**:


```Rust
use std::collections::HashMap;

fn main() {
    banner("*", 52, "Iterating over a hashmap");
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                  Iterating over a hashmap              
    ****************************************************
    Key: b, Value: 2
    Key: c, Value: 3
    Key: a, Value: 1
    ****************************************************


- **Iterating Over a HashSet:**


```Rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    for value in &set {
        println!("Value: {}", value);
    }
}
main();
```

    Value: 2
    Value: 1
    Value: 3


- **Iterating Over a LinkedList:**


```Rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    for value in &list {
        println!("Value: {}", value);
    }
}
main();
```

    Value: 1
    Value: 2
    Value: 3


- **Iterating Over a BinaryHeap:**


```Rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(2);
    heap.push(3);

    for value in heap {
        println!("Value: {}", value);
    }
}
main();
```

    Value: 3
    Value: 1
    Value: 2


## Advanced for loop Usage

### Enumerate and Zip in Rust
In Rust, the enumerate and zip methods provide powerful and flexible ways to iterate over collections. These methods enhance the capabilities of the for loop by allowing you to access additional information during iteration.

#### Using the enumerate Method
The enumerate method creates an iterator that yields pairs of indices and values. This is particularly useful when you need to keep track of the position of each element while iterating over a collection.

Example: Iterating with enumerate
Consider the following example where we iterate over a vector of integers and print each element along with its index:


```Rust
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}
main();
```

    Index: 0, Value: 10
    Index: 1, Value: 20
    Index: 2, Value: 30
    Index: 3, Value: 40
    Index: 4, Value: 50


### Using the zip Method
The zip method creates an iterator that yields pairs of elements from two collections. This is useful when you need to iterate over two collections in parallel.

Example: Iterating with zip
Consider the following example where we iterate over two vectors of integers simultaneously:


```Rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        println!("a: {}, b: {}", a, b);
    }
}
main();
```

    a: 1, b: 4
    a: 2, b: 5
    a: 3, b: 6


## Destructuring in for Loops
Destructuring allows you to conveniently unpack elements of tuples and structs directly within the for loop pattern. This technique simplifies access to multiple elements and can make your code more readable and expressive.

### Destructuring Tuples in for Loops
When iterating over a collection of tuples, you can destructure the tuples directly in the for loop to access each element individually.

### Example: Destructuring Tuples
Consider the following example where we iterate over a vector of tuples, each containing a pair of integers:


```Rust
fn main() {
    let pairs = vec![(1, 2), (3, 4), (5, 6)];

    for (x, y) in pairs {
        println!("x: {}, y: {}", x, y);
    }
}
main();
```

    x: 1, y: 2
    x: 3, y: 4
    x: 5, y: 6


### Example: Destructuring Structs
Consider the following example where we iterate over a vector of structs, each representing a point in 2D space:


```Rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];

    for Point { x, y } in points {
        println!("x: {}, y: {}", x, y);
    }
}
main();
```

    x: 1, y: 2
    x: 3, y: 4
    x: 5, y: 6


## Infinite Iterators in Rust

Rust provides powerful iterator capabilities that include the use of infinite iterators. These iterators can generate an endless sequence of values. To manage and control these infinite sequences, Rust offers methods like take, which allows you to limit the number of iterations to a finite number.

### Using Infinite Iterators
Infinite iterators can be created using the std::iter module. One common method is iter::repeat, which repeats a value infinitely.

### Example: Creating an Infinite Iterator
Consider the following example where we use iter::repeat to create an infinite iterator:


```Rust
fn main() {
    let infinite_ones = std::iter::repeat(1);

    for number in infinite_ones.take(5) {
        println!("Number: {}", number);
    }
}
main();
```

    Number: 1
    Number: 1
    Number: 1
    Number: 1
    Number: 1


### Conclusion

- In this section, we explored the various ways to use the `for` loop in Rust to iterate over different types of collections. The `for` loop is a powerful and flexible control flow construct that allows for concise and efficient iteration over arrays, vectors, ranges, characters, and other data structures.

- We began with a detailed introduction to the `for` loop, highlighting its syntax and advantages. We then provided examples of using `for` loops with inclusive and non-inclusive ranges, demonstrating how to iterate over sequences of numbers and characters. 

- We also covered the use of `for` loops with arrays and vectors, which are commonly used collections in Rust. By iterating over these collections, we can perform operations on each element in a straightforward and readable manner.

- Additionally, we showed how to iterate over more complex collections such as `HashMap`, `HashSet`, `LinkedList`, and `BinaryHeap`. These examples illustrated the versatility of the `for` loop and its ability to handle various data structures efficiently.

- Finally, we presented an example of iterating over a tuple by converting it into an array, showcasing the adaptability of the `for` loop to different types of collections.  Furthermore, we explored nested for loops, which allow for complex iteration patterns by iterating over multiple collections or ranges simultaneously.

