<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Homogeneous Data Structures in Rust: <code>Vectors</code>
</div>

```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

Vectors are one of the most fundamental and versatile collection types in Rust. Unlike arrays, which have a fixed size, vectors provide a dynamically sized array, enabling the storage and manipulation of a variable number of elements of the same type. This flexibility makes vectors a crucial part of Rust's standard library, offering a wide array of methods for efficiently managing and interacting with data.

## Key Characteristics of Vectors

Vectors, denoted as `Vec<T>`, are a generic, sequential, and resizable collection in Rust. They are defined in the standard library (`std::vec`) and are automatically available for use without the need for explicit import statements. Below are some key features that make vectors indispensable in Rust programming:

- **Generic and Sequential Collection**: Vectors are generic over the type of elements they hold, meaning they can store any type of data, as long as it is specified at the time of vector creation. This allows for great flexibility in handling different kinds of data within a single collection type.
  
- **Resizable and Dynamic**: Unlike fixed-size arrays, vectors can dynamically grow and shrink in size as elements are added or removed. This dynamic resizing is managed efficiently by Rust, making vectors suitable for situations where the number of elements is not known in advance or may change over time.

- **Standard Library Integration**: Vectors are a part of Rust's standard library (`std::vec`) and are available by default. This means you can start using vectors immediately without any additional setup, simplifying the development process.

- **Comprehensive Methods**: The `Vec<T>` type comes with a rich set of methods that facilitate various operations such as adding, removing, and accessing elements, iterating over the collection, sorting, and more. These methods are designed to be efficient and ergonomic, adhering to Rust’s principles of safety and performance.

## Vector Syntax and Initialization

To utilize vectors, the standard library's `Vec` type is included by default, eliminating the need for additional imports. In this section, we will explore various methods for creating and initializing vectors in Rust.

### Different Ways of Initializing Vectors

There are several ways to create and initialize vectors in Rust, each suited to different scenarios and preferences. Below, we outline the primary methods:

#### 1. Explicit Type Annotation

Explicit type annotation is useful when the type of elements in the vector cannot be inferred from the context. This method ensures clarity and type safety.

To initialize an empty vector with explicit type annotation, use the following syntax:

```rust
// Creating an empty vector with explicit type annotation
let v: Vec<i32> = Vec::new();
```

In this example, `v` is explicitly declared as a vector of i32 elements.

#### 2. Using the Turbofish Operator

The **turbofish** operator (`::<Type>`) provides an alternative way to specify the type when creating a vector. This method is particularly useful for its conciseness and readability.

```rust
// Creating an empty vector using the turbofish operator
let v = Vec::<i32>::new();
```
    
Here, `v` is created as an empty vector of i32 elements using the turbofish operator.

#### 3. Using the vec! Macro
The `vec!` macro is a convenient way to create a vector with initial values. It simplifies vector initialization by inferring the type from the provided values.

```rust
// Creating a vector with initial values using the vec! macro
let v = vec![1, 2, 3, 4, 5];
```

In this example, v is initialized with five i32 elements: 1, 2, 3, 4, and 5.

### 4. Implicit Type Inference
In Rust, the type of elements in a vector must be known at compile time. If the type can be inferred from the context, you can create a vector without explicitly specifying the type. However, it is important to ensure that the type can be inferred correctly by the compiler based on subsequent usage or assignments.

```rust
// Creating an empty vector with implicit type inference
let v = Vec::new();
```

When creating an empty vector with `Vec::new()`, ensure that the type can be inferred from the context in which the vector is used. This approach relies on subsequent operations or variable assignments to determine the vector's type.

### Practical Considerations

When initializing vectors, it is essential to choose the method that best fits the context and clarity of your code. Explicit type annotation and the turbofish operator provide clear type definitions, while the vec! macro offers convenience and readability for initializing vectors with values. Implicit type inference can be useful but requires careful consideration to ensure type correctness.

### `Vec<T>` Implementing Debug Trait

Vectors in Rust implement the `Debug` trait, which allows them to be formatted using the `fmt` method. This trait is crucial for outputting vectors in a readable and user-friendly format, especially for debugging purposes. When you want to print the contents of a vector, you can use the `{:?}` formatter to leverage the Debug trait.

**Using the Debug Trait**: The `Debug` trait enables the fmt method to provide a human-readable representation of the vector, making it easier to inspect and understand the contents of the vector during development. This is particularly useful when troubleshooting or logging the state of your program.

```rust
fn main() {
    // Creating a vector with initial values
    let some_vec = vec![1, 2, 3, 4, 5];
    
    // Printing the vector using the Debug trait
    println!("{:?}", some_vec);
}
```

### Creating Empty Vectors


```Rust
fn main(){
    banner("*", 52, "Creating Empty Vectors");
    
    // Explicitly specifying the type of the vector
    let v1: Vec<i32> = Vec::new();
    
    // Using the turbofish operator to specify the type
    let v2 = Vec::<i32>::new();
    
    // Explicitly specifying the type of the vector to ensure type inference
    let v3: Vec<u64> = Vec::new();
    
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    
    println!("{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
                   Creating Empty Vectors               
    ****************************************************
    []
    []
    []
    ****************************************************


### Examples

### Example 01: Creating Vectors with Explicit Type annotation


```Rust
fn main() {
    banner("*", 52, "Creating vectors with explict type annotation");
    // Creating an empty vector with explicit type annotation
    let v: Vec<i32> = Vec::new();
    
    println!("An empty vector is initialized: {:?}", v);
    
    // Adding elements to the vector
    let mut v = v;
    v.push(10);
    v.push(20);
    v.push(30);
    
    println!("Vector with explicit type annotation: {:?}", v);
    
    println!("\n{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
       Creating vectors with explict type annotation    
    ****************************************************
    An empty vector is initialized: []
    Vector with explicit type annotation: [10, 20, 30]
    
    ****************************************************


### Example 02: Creating Vectors Using the Turbofish Operator


```Rust
fn main() {
    banner("*", 52, "Creating vectors Using Turbofish Operator");
    
    // Creating an empty vector using the turbofish operator
    let mut v = Vec::<i32>::new();
    
    // Adding elements to the vector
    v.push(40);
    v.push(50);
    v.push(60);
    
    println!("Vector using the turbofish operator: {:?}", v);
    
    println!("\n{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
         Creating vectors Using Turbofish Operator      
    ****************************************************
    Vector using the turbofish operator: [40, 50, 60]
    
    ****************************************************


### Example 03: Creating Vectors Using `vec!` Macro


```Rust
fn main() {
    banner("*", 52, "Creating vectors Using Vec! Macro");
    // Creating a vector with initial values using the vec! macro
    let v = vec![70, 80, 90, 100];
    
    println!("Vector using the vec! macro: {:?}", v);
    
    println!("{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
             Creating vectors Using Vec! Macro          
    ****************************************************
    Vector using the vec! macro: [70, 80, 90, 100]
    ****************************************************


#### Implicit Type Inference Example


```Rust
fn main() {
    banner("*", 52, "Creating Vectors with Implicit Type Inference");
    // Creating an empty vector with implicit type inference
    let mut v = Vec::new();
    
    // The type of the vector is inferred from the context when elements are added
    v.push(10);
    v.push(20);
    v.push(30);
    
    println!("\nVector with implicit type inference: {:?}", v);
    
    println!("\n{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
       Creating Vectors with Implicit Type Inference    
    ****************************************************
    
    Vector with implicit type inference: [10, 20, 30]
    
    ****************************************************


When you create an empty vector with `Vec::new()`  without subsequent assignments, the compiler needs to know the type of the elements that the vector will hold. If the type cannot be inferred from the context, the compiler will throw an error,  for example

```rust
fn main(){
    let v = Vec::new();
}
```

Trying to run this program will give this error

```text

[E0282] Error: type annotations needed for `Vec<_>`
   ╭─[command_16:1:1]
   │
 2 │     let v = Vec::new();
   │         ┬│  ─────┬────  
   │         ╰─────────────── error: type annotations needed for `Vec<_>`
   │          │       │      
   │          ╰────────────── help: consider giving `v` an explicit type, where the type for type parameter `T` is specified: `: Vec<T>`
   │                  │      
   │                  ╰────── type must be known at this point
```

So, you must initialize the vector with at least one element so the compiler will decide the type.

## Mutable and Immutable Vectors

Vectors in Rust can be either mutable or immutable, depending on whether you need to modify their contents after creation. Understanding the difference between mutable and immutable vectors is crucial for writing efficient and safe Rust code.

### Immutable Vectors

An **immutable** vector is one that cannot be changed after it is created. This immutability ensures that the contents of the vector remain constant throughout its usage, providing safety and predictability. Immutable vectors are particularly useful when you want to ensure that the data structure remains unchanged, preventing accidental modifications.

When a vector is declared as immutable, any attempt to modify its contents will result in a compile-time error. This guarantees that the integrity of the data is maintained.


### Practical Use Cases for Immutable Vectors

- Constant Data: When working with data that should not change throughout the program's execution, such as configuration settings or static lookup tables, immutable vectors provide safety and clarity.
- Thread Safety: In concurrent programming, immutable data structures eliminate the need for synchronization mechanisms, as they cannot be altered by any thread.

#### Example: Using an Immutable Vector


```Rust
// example of immuatable vector
fn main() {
    banner("*", 52, "Creating Immutable Vectors");
    let v = vec![10, 20, 30, 40, 50];
    println!("The immutable vec {:?}", v);
    
    // Attempting to modify the vector will result in a compile-time error
    // v.push(6); // Uncommenting this line will cause a compilation error
    
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                 Creating Immutable Vectors             
    ****************************************************
    The immutable vec [10, 20, 30, 40, 50]
    ****************************************************


### Code in Details

- Vector Initialization:
    - `let v = vec![1, 2, 3, 4, 5];`: An immutable vector numbers is created and initialized with the values 1, 2, 3, 4, and 5.

- Immutable Nature:
    - The vector `v` is immutable, meaning its contents cannot be modified. Attempting to do so, as shown in the commented - line `v.push(6)`;, will result in a compile-time error.

- Printing the Vector:
    - `println!("Immutable vector: {:?}", v)`;: The contents of the immutable vector are printed, demonstrating its integrity and stability.

### Mutable Vectors
On the other hand, mutable vectors allow for modifications after their creation. This mutability is essential when you need to add, remove, or update elements dynamically during the program's execution.

To declare a mutable vector use the `mut` keyword. 

### Practical Use Cases for Mutable Vectors

- Dynamic Data: When dealing with data that changes over time, such as user input, sensor readings, or real-time computations, mutable vectors provide the necessary flexibility.
- Complex Data Structures: In scenarios where complex data structures (e.g., graphs, trees) are built and modified dynamically, mutable vectors are indispensable.

#### Example: Using an Mutable Vector


```Rust
fn main() {
    banner("*", 52, "Mutable Vectors");
    // Creating a mutable vector
    let mut nums = vec![1, 2, 3, 4, 5];
    
    println!("The original vector: {:?}", nums);
    
    // Modifying the contents of the vector
    nums.push(6);
    nums[0] = 0;
    
    // Printing the contents of the mutable vector
    println!("Modified vector:     {:?}", nums);
    println!("{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
                      Mutable Vectors                   
    ****************************************************
    The original vector: [1, 2, 3, 4, 5]
    Modified vector:     [0, 2, 3, 4, 5, 6]
    ****************************************************


### Code in Details

- Vector Initialization:
    `let mut numbers = vec![1, 2, 3, 4, 5];`: A mutable vector numbers is created and initialized with the values 1, 2, 3, 4, and 5. The mut keyword indicates that the vector is mutable.

- Modifying the Vector:
    - `nums.push(6);`: The push method adds the value 6 to the end of the vector.
    - `nums[0] = 0;`: The first element of the vector is updated to 0.

- Printing the Vector:
    - `println!("Mutable vector: {:?}", nums);`: The contents of the mutable vector are printed, showing the modifications.

## Accessing Vector Elements in Rust

Accessing elements in a vector is a fundamental operation in Rust, allowing you to retrieve and manipulate individual items stored within the vector. Rust provides several methods to access vector elements safely and efficiently, ensuring that common errors such as out-of-bounds access are avoided. It is important to note that vectors in Rust use zero-based indexing, meaning the first element is at index `0`.


### Indexing

The simplest way to access an element in a vector is by using the indexing syntax. This method retrieves a reference to the element at the specified index.

**Example**


```Rust
fn main() {
    banner("*", 52, "Accessing Vector Elements");
    let numbers = vec![10, 20, 30, 40, 50];
    
    // Accessing elements using indexing
    let first = numbers[0];
    let second = numbers[1];
    
    let last = numbers[numbers.len() - 1];
    
    println!("First element:  {}", first);
    println!("Second element: {}", second);
    println!("Last element:   {}", last);
    
    println!("{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
                 Accessing Vector Elements              
    ****************************************************
    First element:  10
    Second element: 20
    Last element:   50
    ****************************************************


### Using the get Method
The get method provides a safer way to access elements in a vector. It returns an Option<&T>, which is Some(&T) if the index is within bounds, and None otherwise. This method helps prevent runtime panics due to out-of-bounds access.


```Rust
fn main() {
    banner("*", 52, "Accessing Vector Elements Using get method");
    let numbers = vec![10, 20, 30, 40, 50];
    
    // Safely accessing elements using the get method
    match numbers.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("No element found at index 0"),
    }
    
    match numbers.get(10) {
        Some(element) => println!("Element at index 10: {}", element),
        None => println!("No element found at index 10"),
    }
    
    println!("{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
         Accessing Vector Elements Using get method     
    ****************************************************
    First element: 10
    No element found at index 10
    ****************************************************


### Code in Details

- Vector Initialization:
    - let numbers = vec![10, 20, 30, 40, 50];: A vector numbers is initialized with values.

- Using the get Method:
    - numbers.get(0): Safely attempts to access the first element. Since the index is within bounds, it returns Some(&10).
    - numbers.get(10): Attempts to access an element at index 10, which is out of bounds. It returns None.

- Pattern Matching:
    - match numbers.get(0): Matches the result of numbers.get(0), printing the element if it exists.
    - match numbers.get(10): Matches the result of numbers.get(10), printing a message indicating no element is found at the specified index.

### Mutable Access
When you need to modify the elements of a vector, you can access them mutably using indexing or the get_mut method.


```Rust
fn main() {
    banner("*", 52, "Accessing Vector Elements Using get_mut method");
    let mut numbers = vec![10, 20, 30, 40, 50];
    
    // Mutable access using indexing
    numbers[0] = 15;
    numbers[2] = 35;
    
    // Mutable access using the get_mut method
    if let Some(third) = numbers.get_mut(2) {
        *third = 33;
    }
    
    println!("Modified vector: {:?}", numbers);
    
    println!("{}", "*".repeat(52));
}

main();
```

    
    ****************************************************
       Accessing Vector Elements Using get_mut method   
    ****************************************************
    Modified vector: [15, 20, 33, 40, 50]
    ****************************************************


### Code in Details

- Vector Initialization:
    - `let mut numbers = vec![10, 20, 30, 40, 50];`: A mutable vector numbers is created and initialized with values.

- Mutable Indexing:
    - `numbers[0] = 15;`: Modifies the first element to 15.
    - `numbers[2] = 35;`: Modifies the third element to 35.

- Using get_mut Method:
    - `if let Some(third) = numbers.get_mut(2)`: Safely obtains a mutable reference to the third element.
    - `*third = 33;`: Modifies the third element to 33.
        - The `*` operator is the dereference operator which is used to dereference a pointer or reference, allowing us to access or modify the value that the pointer or reference points to.

- Printing the Modified Vector:
    - `println!("Modified vector: {:?}", numbers);`: Prints the modified contents of the vector.

## Basic Operations with Vectors

Vectors in Rust are highly versatile and support a wide range of operations, similar to arrays. However, in this section, we will focus on the most fundamental operations to provide a solid foundation. More advanced operations will be discussed in subsequent chapters.

### Iterating over Vector Elements
Iteration is one of the most common operations performed on vectors. Rust provides several methods to iterate over the elements of a vector, each suited to different use cases. Below, we explore these methods in detail.

### Using a for Loop
The for loop is the most straightforward and widely used method for iterating over the elements of a vector. It is simple and concise, making it ideal for most use cases.

Vectors in Rust do not implement the `Copy` trait. Therefore, when iterating over their elements, we need to use the `&` character before their name to take a reference to the vector rather than the vector itself. This process is known as borrowing, and it allows us to access the elements without taking ownership of the vector. Borrowing is a fundamental concept in Rust's ownership system, which we will discuss in detail in a later chapter.


```Rust
fn main() {
    banner("*", 72, "Iterating over vector elements");
    let nums = vec![10, 20, 30, 40, 50];
    println!("The initial vector is: {:?}\n", nums);
    
    // Iterating over vector elements 
    for number in &nums {
        println!("Number: {}", number);
    }
    
    // Loop through the vector elements and indexes
    // we chain two methods, iter() and enumerate() (These will be discussed later)
    for (index, value) in nums.iter().enumerate() {
        println!("The index is:\t {} and the value is: \t{}", index, value);
    }
    
    println!("\nUsing vector is allowable after iterations: {:?}", nums);
    println!("{}", "*".repeat(72));
}

main();
```

    
    ************************************************************************
                         Iterating over vector elements                     
    ************************************************************************
    The initial vector is: [10, 20, 30, 40, 50]
    
    Number: 10
    Number: 20
    Number: 30
    Number: 40
    Number: 50
    The index is:	 0 and the value is: 	10
    The index is:	 1 and the value is: 	20
    The index is:	 2 and the value is: 	30
    The index is:	 3 and the value is: 	40
    The index is:	 4 and the value is: 	50
    
    Using vector is allowable after iterations: [10, 20, 30, 40, 50]
    ************************************************************************


### Summary

In this section, we have covered the essential concepts and operations related to vectors in Rust, emphasizing their flexibility and versatility in managing dynamic collections of data. Here are the main points we have highlighted:

- **Vector Basics**: Vectors are dynamic arrays provided by Rust's standard library, allowing for the storage of a variable number of elements of the same type.

- **Initialization Methods**:
  - Explicit Type Annotation: Ensures clarity and type safety.
  - Turbofish Operator: Provides a concise way to specify the type.
  - `vec!` Macro: Convenient for creating vectors with initial values.
  - Implicit Type Inference: Utilizes context to infer the type.

- **Immutable Vectors**:
  - Once created, the contents cannot be modified.
  - Useful for ensuring data integrity and thread safety.
  - Attempting to modify an immutable vector results in a compile-time error.

- **Mutable Vectors**:
  - Allow for modifications after creation.
  - Essential for dynamic data that changes over time.
  - Examples include adding elements with `push` and updating elements by index.

- **Iteration Techniques**:
  - `for` Loop: Simple and concise.
  - `iter` Method: Returns an iterator for more complex operations.
  - `enumerate` Method: Provides both indices and element references.

- **Accessing Vector Elements**:
  - **Indexing**: Simple and direct method using zero-based indexing.
  - **Using the `get` Method**: Provides safe access by returning an `Option`, preventing out-of-bounds access.
  - **Mutable Access**: Allows modification of elements using both indexing and the `get_mut` method.
  - **Dereferencing**: The `*` operator is used to dereference a mutable reference, enabling the modification of the value it points to.


- **Debug Trait**:
  - Vectors implement the `Debug` trait, allowing for user-friendly output using the `{:?}` formatter.

- **Borrowing and Ownership**:
  - Vectors do not implement the `Copy` trait.
  - Iterating over vector elements requires borrowing (`&`), which allows access without taking ownership.

In subsequent chapters, we will delve into more advanced vector operations, further enhancing our understanding of vectors in Rust.



