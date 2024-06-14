<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Char Type in Rust
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

The `char` type represents a single character. Unlike some other languages where characters are limited to ASCII, Rust's char type is a 32-bit value, allowing it to represent any Unicode character. This makes it a powerful type for handling text in a globalized context.

## Declaring and Using char

- A `char`  in Rust is denoted with **single quotes `''`** and can store any valid Unicode scalar value.



```Rust
fn main() {
    banner("=", 62, "Char Type in Rust");
    let c1: char = 'a';                   // ASCII character
    let c2: char = '中';                  // Chinese character
    let c3: char = '🎛';                  // Emoji
    let c4: char = '⍵';

    println!("c1: {}, c2: {}, c3: {}, c4: {}", c1, c2, c3, c4);
}
main();
```

    
    ==============================================================
                          Char Type in Rust                       
    ==============================================================
    c1: a, c2: 中, c3: 🎛, c4: ⍵


- The `char` type allows only one character, if you try to pass more than one character, you will get a compile error:

```rust
let cc: char = 'ab';                // This will issue an error
```

- Here is the error: 

```text
let cc: char = 'ab';
                   ^^^^ error: character literal may only contain one codepoint
character literal may only contain one codepoint
help: if you meant to write a `str` literal, use double quotes

"ab"
```

### Properties of char

- **32-bit Unicode**: The char type in Rust is capable of holding any valid Unicode scalar value, which means it can represent characters from many different languages and symbol sets.
- **Size**: A char is always 4 bytes (32 bits) in size, regardless of the character it represents.

#### Example:


```Rust
fn main() {
    banner("=", 62, "Char Type Properties");
    let c1: char = 'a';
    let c2: char = '中';                  // Chinese character
    let c3: char = '🎛';                  // Emoji
    let c4: char = '⍵';
    println!("Character: {}, Size: {} bytes", c1, std::mem::size_of_val(&c1));
    println!("Character: {}, Size: {} bytes", c2, std::mem::size_of_val(&c2));
    println!("Character: {}, Size: {} bytes", c3, std::mem::size_of_val(&c3));
    println!("Character: {}, Size: {} bytes", c4, std::mem::size_of_val(&c4));
}
main();
```

    
    ==============================================================
                         Char Type Properties                     
    ==============================================================
    Character: a, Size: 4 bytes
    Character: 中, Size: 4 bytes
    Character: 🎛, Size: 4 bytes
    Character: ⍵, Size: 4 bytes


## Conversion to ASCII Codes of Characters

To convert a char to its numeric representation, the ASCII codes (or Unicode scalar values), you can cast each `char` to a `u32` using the `as` keyword (since `char` in Rust is a Unicode scalar value and u32 can represent these values).

Here is an example demonstrating this:


```Rust
fn main() {
    // Print a banner for the example
    banner("=", 62, "Example of Chars ASCII Codes");

    // Vector of various characters, including letters, a special character, a space
    let vec_char: Vec<char> = vec!['a', 'A', 'b', '#', ' ', '1', '{'];

    // Iterate over the characters and print each character along with its Unicode scalar value
    for c in vec_char.iter() {
        // Print the character and its corresponding Unicode value (ASCII value for ASCII characters)
        println!("Character '{}' has Unicode scalar value: {}", c, *c as u32);
    }
}

// Call the main function to execute the program
main();
```

    
    ==============================================================
                     Example of Chars ASCII Codes                 
    ==============================================================
    Character 'a' has Unicode scalar value: 97
    Character 'A' has Unicode scalar value: 65
    Character 'b' has Unicode scalar value: 98
    Character '#' has Unicode scalar value: 35
    Character ' ' has Unicode scalar value: 32
    Character '1' has Unicode scalar value: 49
    Character '{' has Unicode scalar value: 123


- In this example, we use a vector of characters and print the Unicode scalar value (ASCII code for ASCII characters) for each character in the vector. Vectors and loops will be covered in detail in a later chapter.

- Here is the code explained:
    - `vec_char: Vec<char>`: A vector containing characters:
        - Lowercase and uppercase letters
        - Special characters
        - Space
    - `for c in vec_char.iter()`: Iterates over each character in the vector.
    - `println!("{} ==> {}", c, *c as u32)`: Prints the character and its Unicode scalar value.
        - `*c`: The dereference operator, used here to access the value of the character.
        - `as`: Used to perform the cast.
        - `u32`: The type representing the Unicode scalar value.


```Rust
Converting back from ASCII code to characters is possible, here is an example showing that:
```


```Rust
fn main() {
    banner("=", 62, "converting an ASCII code to a character");

    let ascii_code = 65;                       // ASCII code for 'A'
    let character = ascii_code as u8 as char; // Convert ASCII code to char

    println!("The character for ASCII code {} is '{}'", ascii_code, character);

    // Additional example with a range of ASCII codes
    let ascii_codes = vec![97, 98, 99, 100, 101]; 
    for code in ascii_codes.iter() {
        let char = *code as u8 as char;         // Convert ASCII code to char
        println!("The character for ASCII code {} is '{}'", code, char);
    }
}

main();
```

    
    ==============================================================
               converting an ASCII code to a character            
    ==============================================================
    The character for ASCII code 65 is 'A'
    The character for ASCII code 97 is 'a'
    The character for ASCII code 98 is 'b'
    The character for ASCII code 99 is 'c'
    The character for ASCII code 100 is 'd'
    The character for ASCII code 101 is 'e'


### Working with Unicode Characters

- Because Rust's char type is Unicode, you can work with a wide range of characters beyond the basic ASCII set.


```Rust
fn main() {
    banner("=", 62, "Working with Unicode chars");
    
    let chars = vec!['ß', '中', '📚'];

    for ch in chars {
        println!("Character: '{}', Unicode: U+{:04X}", ch, ch as u32);
    }
}
main();
```

    Character: 'ß', Unicode: U+00DF
    Character: '中', Unicode: U+4E2D
    Character: '📚', Unicode: U+1F4DA


Converting from unicode to chars is also possible:


```Rust
fn main() {
    banner("=", 62, "converting unicode to a character");
    let num =  0x1F4DA;                             
    let book = std::char::from_u32(num).unwrap();
    println!("The character for code point {} is '{}'", num, book);
}
main();
```

    
    ==============================================================
                  converting unicode to a character               
    ==============================================================
    The character for code point 128218 is '📚'


## Char Type Operations

### Comparisons

Characters can be compared using standard comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`). This allows for checking equality, ordering, and range operations.

Characters in Rust are represented by their Unicode scalar values (u32), therefoere the comparison is done based on these numeric values.


```Rust
fn main() {
    banner("=", 62, "Character Comparisons in Rust");

    // Declare two characters
    let ch1: char = 'A';
    let ch2: char = 'a';

    // print the unicode of character
    println!(
        "'{}' has Unicode scalar value {}",
        ch1,
        ch1 as u32
    );
    println!(
        "'{}' has Unicode scalar value {}",
        ch2,
        ch2 as u32
    );
    // Compare the characters using standard comparison operators
    if ch1 < ch2 {
        println!("'{}' is less than '{}' because: {} is less than {} ", 
            ch1, ch2, ch1 as u32, ch2 as u32);
    } else {
        println!("'{}' is not less than '{}'", ch1, ch2);
    }

    // Additional comparisons
    if ch1 == ch2 {
        println!("'{}' is equal to '{}'", ch1, ch2);
    } else {
        println!("'{}' is not equal to '{}'", ch1, ch2);
    }

    if ch1 > ch2 {
        println!("'{}' is greater than '{}'", ch1, ch2);
    } else {
        println!("'{}' is not greater than '{}'", ch1, ch2);
    }
}

main();
```

    
    ==============================================================
                    Character Comparisons in Rust                 
    ==============================================================
    'A' has Unicode scalar value 65
    'a' has Unicode scalar value 97
    'A' is less than 'a' because: 65 is less than 97 
    'A' is not equal to 'a'
    'A' is not greater than 'a'


### Char Type Methods:

The `char` type in Rust has several useful methods, such as:
 - `is_alphabetic()`: Checks if the character is alphabetic letter.
 - `is_numeric()`: Checks if the character is numeric.
 - `to_uppercase()`: Converts the character to uppercase.
 - `to_lowercase()`: Converts the character to lowercase.

Here is an example


```Rust
fn main() {
    let ch: char = 'a';

    // Check if the character is alphabetic
    if ch.is_alphabetic() {
        println!("'{}' is an alphabetic character", ch);
    }

    // Check if the character is numeric
    if !ch.is_numeric() {
        println!("'{}' is not a numeric character", ch);
    }

    // Convert to uppercase
    let upper_ch = ch.to_uppercase().next().unwrap();
    println!("Uppercase of '{}' is '{}'", ch, upper_ch);

    // Convert to lowercase
    let lower_ch = upper_ch.to_lowercase().next().unwrap();
    println!("Lowercase of '{}' is '{}'", upper_ch, lower_ch);
}
main();
```

    'a' is an alphabetic character
    'a' is not a numeric character
    Uppercase of 'a' is 'A'
    Lowercase of 'A' is 'a'


## Summary

- The `char` type in Rust is a 32-bit value that can represent any Unicode scalar value.
- A `char` is always 4 bytes in size.
- Rust provides many useful methods for working with characters, including checking character properties and converting between cases.
- You can compare characters, convert them to their numeric Unicode code points, and vice versa.
- Rust’s `char` type allows for handling a wide variety of characters from different languages and symbol sets, making it very versatile for text processing.
