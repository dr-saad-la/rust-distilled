<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Creating Enums
</div>


```Rust
// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
```

Enums, short for "enumerations," are a fundamental construct that allows you to represent a value that can be one of several different variants, each potentially carrying its own data. 

## What Are Enums?

Enums in Rust allow you to define a type that can have multiple distinct variants. Each variant can optionally carry additional data, which can vary in type and structure. Enums are particularly useful for modeling data that can take on a fixed number of states, each with different associated data. This makes enums an ideal choice for representing state machines, error handling, and other scenarios where a value must be one of a discrete set of possibilities.

Enums are self-describing data types, making them a preferred choice for clearly representing a set of related values.

Enums are extensively utilized in both the Rust standard library and user-defined code, highlighting their importance as a fundamental feature in Rust.


### Key Characteristics of Enums

- **Versatility**: Enums can represent complex data structures by associating different types of data with each variant.
- **Safety**: Rust's pattern matching ensures that all possible variants are accounted for, providing compile-time safety and reducing runtime errors.
- **Clarity**: Enums improve code readability by explicitly defining the possible states a value can be in.

### Use Cases for Enums

Enums are useful in many scenarios, including:
 - Representing different states of a finite state machine.
 - Handling various types of messages or commands in a system.
 - Modeling different error types in error handling.
 - Encoding different types of responses or results.

## Enum Syntax

To define an enum in Rust, you use the `enum` keyword followed by the name of the enum and a set of variants separated with commas and enclosed in braces `{...}`. Each variant can be a simple identifier or a tuple-like or struct-like variant carrying additional data.

In other programming languages are called mnemonics, states or fields, but in Rust, they are called variants. 

**Basic Enum Syntax**

The simplest form of an enum is one with only identifier variants, which do not carry any additional data.

```rust
enum EnumName {
    VariantOne,
    VariantTwo,
    // other variants
}
```


#### Examples

**Status Example**

Consider the following example, which defines an enum named `Status` with two variants:

```rust
enum Status {
    Success,
    Error,
}
```

In this example, the `Status` enum has two variants: `Success` and `Error`. This enum can be used to represent the outcome of an operation, indicating either a successful result or an error condition.

**Representing Cardinal Directions Example**

Enums are particularly useful for representing a fixed set of related values. Here is another example of an enum that represents the four cardinal directions:

```rust
enum Direction {
    North,
    East,
    South,
    West,
}
```
The `Direction` enum has four variants: `North`, `East`, `South`, and `West`. These variants can be used to represent the four cardinal directions. This enum is particularly useful in applications that involve navigation, such as mapping software, games, or robotic movement.

## Naming Conventions for Enums in Rust

Naming conventions in Rust help ensure that code is readable and maintainable. For enums and their variants, Rust follows specific conventions that align with general best practices for Rust code. 

### Enum Names

- **Single-Word Names**: If the enum name is a single word, it should start with uppercase letter.
- **UpperCamelCase**: Enum names with multiple words should be written in UpperCamelCase. This means that each word in the name starts with an uppercase letter and there are no underscores between words.
- **Descriptive**: Enum names should be descriptive and convey the purpose or meaning of the enum.

#### Examples

```rust
// Correct: Single-word enum name
enum Status {
    Active,
    Inactive,
    Pending,
}

// Correct: UpperCamelCase
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


// Incorrect: snake_case or lowercase
// enum traffic_light {
//     red,
//     yellow,
//     green,
// }
```

### Enum Variant Names

- **UpperCamelCase**: Enum variant names should also be written in UpperCamelCase, following the same convention as enum names.
- Descriptive: Variant names should clearly describe the value or state they represent.
- Single-Word Names: If the variant name is a single word, it should still be in UpperCamelCase.

### Examples

```rust
enum FileAccessMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

// Correct: Single-word variant names
enum Status {
    Success,
    Error,
}

// Incorrect: snake_case or lowercase
// enum FileAccessMode {
//     read_only,
//     write_only,
//     read_write,
// }
```

Note that the Rust compiler will issue warnings about not following the naming conventions of enums, therefore it is always considered good practice to follow the convensions. 

### Additional Guidelines

1. **Consistency**: Maintain consistency in naming conventions across the entire codebase. Consistency helps improve readability and maintainability.
2. **Clarity**: Choose names that clearly communicate the purpose and meaning of the enum and its variants. Avoid abbreviations and short forms unless they are widely understood.
3. **Avoid Prefixes**: Avoid using prefixes that repeat the enum name in the variants. Since variants are accessed through the enum, the prefix is redundant.

### Namespacing

Enum variants are namespaced by their enum, which means you can use concise variant names without worrying about name clashes. This namespacing allows you to define variants that have simple and intuitive names without fear of conflict with variants from other enums or other items in your code.

#### How Namespacing Works

When you define an enum, each variant of that enum is associated with the enum's namespace. To access a variant, you use the syntax `EnumName::VariantName`. This fully qualified name ensures that the variant is clearly associated with its enum, eliminating any potential ambiguity.

#### Example

Consider the following enums:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Status {
    Red,
    Green,
    Blue,
}
```

Notice that in both `TrafficLight` and `Status` enums have variants named `Red` and `Green`. Because the variants are namespaced by their respective enums, there is no conflict between the Red and Green variants in TrafficLight and Status. So the following code is a valid code that can be compiled without errors:

```rust
// Run code
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Status {
    Red,
    Green,
    Blue,
}

fn main() {
    banner("*", 52, "Enums in Rust");
    let light = TrafficLight::Red;
    let state = Status::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready..."),
        TrafficLight::Green => println!("Go!"),
    }

    match state {
        Status::Red => println!("Error: Red status"),
        Status::Green => println!("System is operational"),
        Status::Blue => println!("System is in standby"),
    }
    println!("{}", "*".repeat(52));
}
```

## Using Enums Across Modules

In larger Rust projects, it is common practice to define enums in separate modules (or files) to keep the codebase organized and modular. Let us suppose that the previous two enums, TrafficLight and Status, were defined in a separate module named `myenums.rs`. To use these enums in your main code, you first need to import them and then use them with their fully qualified namespace.

### Step-by-Step Guide

1. **Define Enums in a Module:** Create a file named myenums.rs and define the enums:

```rust
// myenums.rs
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub enum Status {
    Red,
    Green,
    Blue,
}
```

2. **Import Enums in the Main Module:** In your main file (e.g., main.rs), import the enums using the mod and use keywords:

```rust
// main.rs
mod myenums;

use myenums::{TrafficLight, Status};

// use the enums using the fully qualified name
fn main() {
    let light = TrafficLight::Red;
    let state = Status::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready..."),
        TrafficLight::Green => println!("Go!"),
    }

    match state {
        Status::Red => println!("Error: Red status"),
        Status::Green => println!("System is operational"),
        Status::Blue => println!("System is in standby"),
    }
}
```

#### Code in Details

1. Defining Enums in a Module: The enums TrafficLight and Status are defined in a separate file myenums.rs. The pub keyword is used to make these enums public so that they can be accessed from other modules.

2. Importing Enums:
    - The mod myenums; statement declares the module.
    - The use myenums::{TrafficLight, Status}; statement brings the enums into scope, allowing you to use them with their fully qualified names.

3. Using Fully Qualified Names: The variants are accessed using their fully qualified names, such as `TrafficLight::Red` and `Status::Green`.


```Rust
// Run code
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Status {
    Red,
    Green,
    Blue,
}

fn main() {
    banner("*", 52, "Enums in Rust");
    let light = TrafficLight::Red;
    let state = Status::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready..."),
        TrafficLight::Green => println!("Go!"),
    }

    match state {
        Status::Red => println!("Error: Red status"),
        Status::Green => println!("System is operational"),
        Status::Blue => println!("System is in standby"),
    }
    println!("{}", "*".repeat(52));
}
main();
```

    
    ****************************************************
                       Enums in Rust                    
    ****************************************************
    Stop!
    System is operational
    ****************************************************


### Documentation

Code are read more otfen then written, therefore adding comments or documentation to explain the purpose of the enum and its variants is essential, especially if the names are not self-explanatory.

#### Example
```rust
/// Represents the different states of a traffic light.
enum TrafficLight {
    /// The traffic light is red, indicating that vehicles must stop.
    Red,
    /// The traffic light is yellow, indicating that vehicles should prepare to stop.
    Yellow,
    /// The traffic light is green, indicating that vehicles may go.
    Green,
}
```

While this example is inherently self-descriptive, it serves as a demonstration of how to effectively document or comment your code. Proper documentation enhances code readability and maintainability, making it easier for others (and yourself) to understand the code's purpose and functionality.

Let us consider a example in physics which might not be immediately clear to those outside the field. Here is the example without documentation:

```rust
enum ElementaryParticle {
    Quark {
        flavor: String,
        color: String,
    },
    Lepton {
        kind: String,
        charge: i32,
    },
    Boson {
        kind: String,
        mass: f64,
    },
}
```

While this is the same example with proper comments

```rust
/// Represents different types of elementary particles in physics.
enum ElementaryParticle {
    /// Quarks are fundamental particles, like up and down quarks.
    Quark {
        flavor: String,
        color: String,
    },
    /// Leptons are particles like electrons and muons.
    Lepton {
        kind: String,
        charge: i32,
    },
    /// Bosons are force-carrying particles, such as photons.
    Boson {
        kind: String,
        mass: f64,
    },
}
```

This shows the importance of adopting documentation at an early stage of learning coding.

### Summary

In this section, we have explored the essential aspects of enums in Rust, highlighting their versatility and importance in creating robust and maintainable code. Here are the key points we covered:

- **Definition and Syntax**:
  - Enums are defined using the `enum` keyword followed by a set of variants.
  - Variants can be simple identifiers or carry additional data in tuple-like or struct-like forms.
  
- **Naming conventions**:
    - UpperCamelCase: Use UpperCamelCase for both enum names and variant names, regardless of whether they are single-word or multi-word.
    - Descriptive Names: Choose names that clearly describe the purpose and meaning.
    - Consistency: Maintain consistent naming conventions across the codebase.
    - Avoid Redundant Prefixes: Enum variants are namespaced by their enum, so avoid repeating the enum name in the variants.
    - Documentation: Add comments or documentation to explain enums and variants when necessary.
- **Namespacing**:
  - Enum variants are namespaced by their enum, allowing for concise and intuitive variant names without risk of name clashes.
