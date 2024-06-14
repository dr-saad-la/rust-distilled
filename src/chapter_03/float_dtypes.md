<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Floating Point Data Types in Rust
</div>

```Rust
use std::any::type_name;

// Function to create a formatted banner
pub fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
```

Floating point data types in Rust are used to represent numbers that have fractional parts. They are especially useful for scientific computations, graphics programming, and any application where you need to perform calculations with real numbers. Rust provides two primary floating point types:

  - **f32**: A 32-bit floating point number.
  - **f64**: A 64-bit floating point number (double precision).

In this lesson, we will cover the basics of floating point types in Rust, how to declare them, their ranges, and some common operations.

### The f32: 32-bit Floating Point Number

The **f32** type represents a single-precision floating-point number. It occupies 32 bits (4 bytes) of memory and provides a balance between range and precision, suitable for many applications where performance and memory usage are considerations.

### Range and Precision:

 - The approximate range is from 1.2E-38 to 3.4E+38.
 - It has about 7 decimal digits of precision.
 
### Recommendation

- Use `f32` only if you want to save some space
- Use `f64` for maximum precision generally. (Most APIs in Rust use `f64`)


```Rust
/// This program demonstrates the use of the 32-bit floating-point type (`f32`) in Rust.
/// It showcases how to define and print the value, type, and size in memory.

fn main() {
    banner("=", 62, "32-bit floating-point variable with 4 Decimal Points");
    // Define a 32-bit floating-point variable with a precise value for Pi
    let float32: f32 = 3.1415927;
    println!(
        "Value: {:.4}, Type: {}, Size: {} bytes",
        float32,
        type_of(float32),
        std::mem::size_of::<f32>()
    );
}

main();
```

    
    ==============================================================
         32-bit floating-point variable with 4 Decimal Points     
    ==============================================================
    Value: 3.1416, Type: f32, Size: 4 bytes


## The f64: 64-bit Floating Point Number

The **f64** type represents a double-precision floating-point number. It occupies 64 bits (8 bytes) of memory and offers higher precision and range compared to f32, making it suitable for more precise scientific computations.

**Range and Precision:**
 - The approximate range is from 2.2E-308 to 1.8E+308.
 - It has about 15 decimal digits of precision.


```Rust
/// This program demonstrates the use of the 64-bit floating-point type (`f64`) in Rust.
/// It showcases how to define and print the value, type, and size in memory.

fn main() {
    banner("=", 62, "64-bit floating-point variable with 7 Decimal Points");
    // Define a 64-bit floating-point variable with a precise value for Pi
    let float64: f64 = 3.141592653589793;
    println!(
        "Value: {:.7}, Type: {}, Size: {} bytes",
        float64,
        type_of(float64),
        std::mem::size_of::<f64>()
    );
}

main();
```

    
    ==============================================================
         64-bit floating-point variable with 7 Decimal Points     
    ==============================================================
    Value: 3.1415927, Type: f64, Size: 8 bytes


## Floats in Details

The following example will show some details about the floating point number, such as the memory size, and the precision. The `f64` is really a big number, you'll see!


```Rust
// program demonstrating the float data types in rust
fn main() {
    banner("=", 62, "The 32-bit floating point type");

    // f32: The 32-bit floating point type
    let float32: f32 = 3.14;
    println!("Value: {:<15} \nType: {:<10} \nSize: {} bytes \nMin: {},\nMax: {}", 
        float32, 
        type_of(float32), 
        std::mem::size_of::<f32>(), 
        f32::MIN, 
        f32::MAX
    );

     banner("=", 62, "The 64-bit floating point t");
    // f64: The 64-bit floating point type
    let float64: f64 = 3.14;
    println!("Value: {:<15} \nType: {:<10}\nSize: {} bytes \nMin: {} \nMax: {}", 
        float64, 
        type_of(float64), 
        std::mem::size_of::<f64>(), 
        f64::MIN, 
        f64::MAX
    );
}
main();
```

    
    ==============================================================
                    The 32-bit floating point type                
    ==============================================================
    Value: 3.14            
    Type: f32        
    Size: 4 bytes 
    Min: -340282350000000000000000000000000000000,
    Max: 340282350000000000000000000000000000000
    
    ==============================================================
                     The 64-bit floating point t                  
    ==============================================================
    Value: 3.14            
    Type: f64       
    Size: 8 bytes 
    Min: -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000 
    Max: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000


## Special Values

Floating point numbers can represent several special values, including:

1. **Infinity:** Positive and negative infinity.
2. **NaN:** Not a Number, used to represent undefined or unrepresentable values.

#### Examples:


```Rust
fn main() {
    banner("=", 62, "Special Values");
    let positive_infinity = f32::INFINITY;
    let negative_infinity = f32::NEG_INFINITY;
    let nan = f32::NAN;

    println!("Positive Infinity: {}", positive_infinity);
    println!("Negative Infinity: {}", negative_infinity);
    println!("NaN: {}", nan);
}

main();
```

    
    ==============================================================
                            Special Values                        
    ==============================================================
    Positive Infinity: inf
    Negative Infinity: -inf
    NaN: NaN


---

## Formatting Float Variable

The `{..}` of `println!` macro can used to format float variables along with other data types. 


### Specifying the Field Width

The `println!` macro can be used to format strings in various ways. One common formatting need is to specify the field width for printed values. This can be done using the format specifications within the placeholder `{}`. This section explains how to specify the field width and how to use fill characters in formatted output.

#### Specifying the Field Width

To specify the field width in the `println!` macro, you can use the format specifications inside the curly braces `{}`. The general syntax is `{:width}`, where `width` is the minimum number of characters to be printed. If the value to be printed is shorter than the specified width, it will be padded with spaces by default.


### The `println!` formatting Behavior
By default, the `println!` macro right-aligns integers and left-aligns strings when a field width is specified. Here 

  - **Right Alignment for Integers**: When formatting integers with a specified field width, Rust pads the output with spaces to the left, effectively right-aligning the integer within the specified width.
  - **Left Alignment for Strings**: When formatting strings with a specified field width, Rust pads the output with spaces to the right, effectively left-aligning the string within the specified width.
  


```Rust
fn main() {
    // Right-aligned integer with a field width of 5
    println!("{:5}", 42); // Prints "   42"

    // Left-aligned string with a field width of 5
    println!("{:5}", "Hi"); // Prints "Hi   "

    // Additional examples for clarity
    println!("{:5}", 7); // Prints "    7"
    println!("{:5}", "Rust"); // Prints "Rust "
}

main();
```

       42
    Hi   
        7
    Rust 


In this example, the number 42 and 7 are right-aligned with spaces added to the left, while the string "Hi" and "Rust" are left-aligned within a field of width 5, padded with spaces to the right.

## Specifying Alignment

- By default, values are right-aligned. You can change the alignment using:
    - `<` for left alignment
    - `>` for right alignment (the default)
    - `^` for center alignment.


```Rust
fn main() {
    banner("=", 62, "Formatting Alignment");
    // Right-aligned integer with a field width of 5 (default)
    println!("{:>7}", 42); // Prints "   42"

    // Left-aligned integer with a field width of 7
    println!("{:<7}", 42); // Prints "42   "

    // Center-aligned integer with a field width of 7
    println!("{:^7}", 42); // Prints " 42  "

    // Right-aligned string with a field width of 7
    println!("{:>7}", "Rust"); // Prints "   Rust"

    // Left-aligned string with a field width of 7 (default)
    println!("{:<7}", "Rust"); // Prints "Rust   "

    // Center-aligned string with a field width of 7
    println!("{:^7}", "Rust"); // Prints " Rust  "
}

main();
```

    
    ==============================================================
                         Formatting Alignment                     
    ==============================================================
         42
    42     
      42   
       Rust
    Rust   
     Rust  


### Specifying the Fill Character

- You can specify a fill character to pad the output instead of spaces. This is done by placing the fill character and the alignment specifier before the width:


```Rust
fn main() {
    banner("=", 62, "Example of right-alignment with zero padding");
    // Prints "0000042" with zeros as the fill character to make the total width 7
    println!("{:0>7}", 42); 
    
    // Example of left-alignment with asterisks as the fill character
    // Prints "Hi*****" with asterisks added to the right to make the total width 7
    println!("{:*<7}", "Hi"); 
    
    // Example of center-alignment within a field width of 10
    // Prints "   Rust   " with spaces added to both sides to center the text
    println!("{:^10}", "Rust"); 
}

// Call the main function to execute the program
main();

```

    
    ==============================================================
             Example of right-alignment with zero padding         
    ==============================================================
    0000042
    Hi*****
       Rust   


## Specifying Precision for Floating-Point Numbers


You can control the number of decimal places displayed for floating-point numbers by specifying the precision after a period (.) within the formatting string. This feature is particularly useful when you need to format numbers for display purposes, such as in scientific calculations, financial applications, or any context where the precision of floating-point representation matters.

#### Example: Formatting Floating-Point Numbers with Precision
The following example demonstrates how to specify the number of decimal places for a floating-point number:


```Rust
fn main() {
    banner("=", 62, "Formatting Floating-Point Numbers with Precision");
    let pi = 3.141592653589793;

    // Print with 2 decimal places
    println!("{:.2}", pi); // Prints "3.14"

    // Print with 4 decimal places
    println!("{:.4}", pi); // Prints "3.1416"
}

main();
```

    
    ==============================================================
           Formatting Floating-Point Numbers with Precision       
    ==============================================================
    3.14
    3.1416


Here is another example showing how to format floats:


```Rust
fn main() {
    banner("=", 62, "Extended example of formatting floats");
    let e = 2.718281828459045;

    // Print with no decimal places
    println!("{:.0}", e); // Prints "3"

    // Print with 1 decimal place
    println!("{:.1}", e); // Prints "2.7"

    // Print with 5 decimal places
    println!("{:.5}", e); // Prints "2.71828"

    // Print with 10 decimal places
    println!("{:.10}", e); // Prints "2.7182818285"
}

main();
```

    
    ==============================================================
                Extended example of formatting floats             
    ==============================================================
    3
    2.7
    2.71828
    2.7182818285


## Combining Width, Alignment, and Precision
- You can combine field width, alignment, fill character, and precision for more complex formatting:


```Rust
fn main() {
    banner("=", 62, "Combining different alignment operators");
    
    // Right-align with custom width and padding character
    println!("{:*>10}", 123); // Prints "*******123"

    // Left-align with custom width and padding character
    println!("{:-<10}", "Hello"); // Prints "Hello-----"

    // Center-align with custom width and padding character
    println!("{:=^12}", "World"); // Prints "===World==="

    // Default right-align for integers
    println!("{:>8}", 45); // Prints "      45"

    // Default left-align for strings
    println!("{:<8}", "Hi"); // Prints "Hi      "
    
    let pi = 3.141592653589793;
    println!("{:>10.2}", pi); // Prints "      3.14" with right alignment, width 10, and 2 decimal places
    println!("{:*^10.4}", pi); /*
                    Prints "**3.1416**" with center alignment, width 10, 4 decimal places, 
                    and asterisks as the fill character
                    */
    
    let float64: f64 = 3.141592653589793;
    
    println!("L-aligned: {:~<10.4}", float64);
    println!("R-aligned: {:~>10.4}", float64);
}

main();
```

    
    ==============================================================
               Combining different alignment operators            
    ==============================================================
    *******123
    Hello-----
    ===World====
          45
    Hi      
          3.14
    **3.1416**
    L-aligned: 3.1416~~~~
    R-aligned: ~~~~3.1416


## Formatting with Named Arguments
You can use named arguments in the `println!` macro for clearer formatting:


```Rust
fn main() {
    banner("=", 62, "Formatting Named Arguments");
    
    let name = "Alice";
    let age = 30;

    // Use named arguments in the println! macro to print a formatted string
    println!("{name} is {age} years old", name = name, age = age);
}

main();
```

    
    ==============================================================
                      Formatting Named Arguments                  
    ==============================================================
    Alice is 30 years old


## Formatting Numbers with Thousands Separator

To include a thousands separator in numeric values, you can use the num.to_formatted_string(&Locale::en) from the `num-format` crate. 


```Rust
:dep num-format

use num_format::{Locale, ToFormattedString};

fn main() {
    let num = 1000000;
    println!("{}", num.to_formatted_string(&Locale::en));
}
main();
```

Here is a customized function to do that; 


```Rust
fn format_with_commas(num: u64) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for (i, c) in num_str.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    result.chars().rev().collect()
}

fn main() {
    let num = 1_000_000;                     // Using underscores for readability in the code
    println!("{}", format_with_commas(num)); // Prints "1,000,000"
}
main();
```

    1,000,000


### Scientific Notation

Scientific notation is a way of representing numbers that are too large or too small to be conveniently written in decimal form. Rust's `println!` macro allows you to format numbers in scientific notation using either lowercase `e` or uppercase `E`.

#### The Lowercase `e`

Using the lowercase `e` in the format specifier will output the number in scientific notation with a lowercase `e` to denote the exponent.

**Example**:


```Rust
fn main() {
    let num = 12345.6789;
    println!("{:e}", num); // Prints "1.234568e4"
}
main();
```

    1.23456789e4


## The Uppercase E
- Using the uppercase E in the format specifier will output the number in scientific notation with an uppercase E to denote the exponent.

Example:


```Rust
fn main() {
    let num = 12345.6789;
    println!("{:E}", num); // Prints "1.234568E4"
}
main();
```

    1.23456789E4



```Rust
fn main() {
    let num = 0.00012345;
    println!("{:e}", num); // Prints "1.234500e-4"
    println!("{:E}", num); // Prints "1.234500e-4"
}
main();
```

    1.2345e-4
    1.2345E-4


Here is a practical example where to format float in scientific notation:


```Rust
fn main() {
    let pi = 3.141592653589793;
    let euler_number = 2.718281828459045;
    let golden_ratio = 1.618033988749895;
    let avogadro_number = 6.02214076e23;
    let speed_of_light = 299792458.0;
    let gravitational_constant = 6.67430e-11;

    println!("Pi (π) in scientific notation (e):                     {:e}", pi);
    println!("Pi (π) in scientific notation (E):                     {:E}", pi);

    println!("Euler's Number (e) in scientific notation (e):         {:e}", euler_number);
    println!("Euler's Number (e) in scientific notation (E):         {:E}", euler_number);

    println!("Golden Ratio (φ) in scientific notation (e):           {:e}", golden_ratio);
    println!("Golden Ratio (φ) in scientific notation (E):           {:E}", golden_ratio);

    println!("Avogadro's Number in scientific notation (e):          {:e}", avogadro_number);
    println!("Avogadro's Number in scientific notation (E):          {:E}", avogadro_number);

    println!("Speed of Light (c) in scientific notation (e):         {:e}", speed_of_light);
    println!("Speed of Light (c) in scientific notation (E):         {:E}", speed_of_light);

    println!("Gravitational Constant (G) in scientific notation (e): {:e}", gravitational_constant);
    println!("Gravitational Constant (G) in scientific notation (E): {:E}", gravitational_constant);
}

main();
```

    Pi (π) in scientific notation (e):                     3.141592653589793e0
    Pi (π) in scientific notation (E):                     3.141592653589793E0
    Euler's Number (e) in scientific notation (e):         2.718281828459045e0
    Euler's Number (e) in scientific notation (E):         2.718281828459045E0
    Golden Ratio (φ) in scientific notation (e):           1.618033988749895e0
    Golden Ratio (φ) in scientific notation (E):           1.618033988749895E0
    Avogadro's Number in scientific notation (e):          6.02214076e23
    Avogadro's Number in scientific notation (E):          6.02214076E23
    Speed of Light (c) in scientific notation (e):         2.99792458e8
    Speed of Light (c) in scientific notation (E):         2.99792458E8
    Gravitational Constant (G) in scientific notation (e): 6.6743e-11
    Gravitational Constant (G) in scientific notation (E): 6.6743E-11


---

### Common Operations

- Floating point numbers support a variety of operations, including addition, subtraction, multiplication, division, and more.

Example of Basic Operations:

___


```Rust
fn main() {
    banner("=", 62, "Basic Arithmetic operations with Floats");

    let a: f32 = 5.5;
    let b: f32 = 2.2;

    // Perform basic arithmetic operations
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;

    // Print the results of the arithmetic operations
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}

// Call the main function to execute the program
main();

```

    
    ==============================================================
               Basic Arithmetic operations with Floats            
    ==============================================================
    Sum: 7.7
    Difference: 3.3
    Product: 12.1
    Quotient: 2.5


---

### Methods on Floating Point Types

- Rust provides several useful methods for floating point types. Here are some common ones:

    * `abs()`: Returns the absolute value.
    * `sqrt()`: Returns the square root.
    * `powi()`: Raises the number to an integer power.
    * `sin()`, `cos()`, `tan()`: Trigonometric functions.

#### Examples:


```Rust
fn main() {
    banner("=", 62, "Few Methods of float types");
    let x: f64 = -3.14;
    let y: f64 = 2.0;

    println!("Absolute value of {}: {}", x, x.abs());
    println!("Square root of {}: {}", y, y.sqrt());
    println!("{} raised to the power 3: {}", y, y.powi(3));
    println!("Sine of {}: {}", x, x.sin());
}

main();
```

    
    ==============================================================
                      Few Methods of float types                  
    ==============================================================
    Absolute value of -3.14: 3.14
    Square root of 2: 1.4142135623730951
    2 raised to the power 3: 8
    Sine of -3.14: -0.0015926529164868282


## Summary

- Rust provides two floating point types: `f32` (32-bit) and `f64` (64-bit).
- `f32` is single-precision, suitable for applications where memory is constrained.
- `f64` is double-precision, suitable for applications requiring higher precision.
- Floating point types support a wide range of mathematical operations and special values.
- Rust’s standard library provides many useful methods for working with floating point numbers.
