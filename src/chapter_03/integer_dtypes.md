<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
   Integer Data Types in Rust
</div>

Throughout the chapters of this, function or tools to friendly format the output on the console, we define a function at the top the chapter, and this full will be used throughout the chapter or throughout the book.

```Rust
use std::any::type_name;
// Function to create a formatted banner
pub fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}

// function to display the data type of a variable
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
```

## Introduction

In Rust, integer types are fundamental data types used to represent whole numbers. Rust provides several integer types with different sizes and characteristics. This lesson will cover how to declare integer variables with explicit typing, the various integer types available, and some important properties of these types.

## Integers Types in Rust

-   Rust's integer types are divided into two categories:
    1. **Signed integers:** These can represent both positive and negative numbers.
    2. **Unsigned integers:** These can only represent positive numbers.

### Signed Integer Types

#### An 8-bit signed integer: i8

-   **Range:** -128 to 127.
-   **Usage:** `i8` is used when you need to store small integer values and want to minimize memory usage. It’s commonly used in applications where memory is limited or for specific algorithms that require small-range integer arithmetic.

#### A 16-bit signed integer: i16

-   **Range:** -32,768 to 32,767.
-   **Usage:** `i16` is useful for storing medium-range integer values. It is often used in applications such as signal processing or where you need a larger range than `i8` but still want to keep memory usage relatively low.

#### A 32-bit signed integer: i32

**Range:** -2,147,483,648 to 2,147,483,647.  
**Usage:** `i32` is one of the most commonly used integer types because it provides a good balance between range and memory usage. It’s suitable for most arithmetic operations and is typically the default integer type in many applications unless there is a specific need for a different size.

#### A 64-bit signed integer: i64

-   **Range:** -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.
-   **Usage:** `i64` is used when you need to store very large integer values. It is commonly used in applications involving large datasets, high-precision calculations, or when dealing with timestamps and file sizes.

#### A 128-bit signed integer: i128

**Range:** -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727.  
**Usage:** `i128` is used for applications that require extremely large integer values beyond the range of `i64`. It’s suitable for high-precision scientific calculations, cryptographic applications, and financial computations where very large numbers are involved.

**Here is the list of signed integer types in Rust**:

<table>
    <thead>
        <tr>
            <th>Type</th>
            <th>Description</th>
            <th>Range</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>i8</code></td>
            <td>8-bit signed integer</td>
            <td>-128 to 127</td>
        </tr>
        <tr>
            <td><code>i16</code></td>
            <td>16-bit signed integer</td>
            <td>-32,768 to 32,767</td>
        </tr>
        <tr>
            <td><code>i32</code></td>
            <td>32-bit signed integer</td>
            <td>-2,147,483,648 to 2,147,483,647</td>
        </tr>
        <tr>
            <td><code>i64</code></td>
            <td>64-bit signed integer</td>
            <td>-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807</td>
        </tr>
        <tr>
            <td><code>i128</code></td>
            <td>128-bit signed integer</td>
            <td>-170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727</td>
        </tr>
        <tr>
            <td><code>isize</code></td>
            <td>Pointer-sized signed integer (architecture dependent)</td>
            <td><code>-2^(N-1)</code> to <code>2^(N-1) - 1</code></td>
        </tr>
    </tbody>
</table>

### Unsigned Integer Types

#### An 8-bit unsigned integer: u8

-   **Range**: 0 to 255.
-   **Usage**: `u8` is used when you need to store small non-negative integer values and want to minimize memory usage. It’s commonly used in applications such as byte manipulation, image processing, and data serialization where values are guaranteed to be within this range.

#### A 16-bit unsigned integer: u16

-   **Range**: 0 to 65,535.
-   **Usage**: `u16` is useful for storing medium-range non-negative integer values. It is often used in applications such as graphics, network protocols, and systems programming where a larger range than `u8` is needed but still want to keep memory usage relatively low.

#### A 32-bit unsigned integer: u32

-   **Range**: 0 to 4,294,967,295.
-   **Usage**: `u32` is one of the most commonly used unsigned integer types because it provides a good balance between range and memory usage. It’s suitable for most non-negative arithmetic operations and is typically the default unsigned integer type in many applications unless there is a specific need for a different size.

#### A 64-bit unsigned integer: u64

-   **Range**: 0 to 18,446,744,073,709,551,615.
-   **Usage**: `u64` is used when you need to store very large non-negative integer values. It is commonly used in applications involving large datasets, high-precision calculations, or when dealing with timestamps, large file sizes, and large ranges of IDs.

#### A 128-bit unsigned integer: u128

-   **Range**: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455.
-   **Usage**: `u128` is used for applications that require extremely large non-negative integer values beyond the range of `u64`. It’s suitable for high-precision scientific calculations, cryptographic applications, and financial computations where very large numbers are involved.

**Here is a table of unsigned integer Types in Rust**:

<table>
    <thead>
        <tr>
            <th>Type</th>
            <th>Description</th>
            <th>Range</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>u8</code></td>
            <td>8-bit unsigned integer</td>
            <td>0 to 255</td>
        </tr>
        <tr>
            <td><code>u16</code></td>
            <td>16-bit unsigned integer</td>
            <td>0 to 65,535</td>
        </tr>
        <tr>
            <td><code>u32</code></td>
            <td>32-bit unsigned integer</td>
            <td>0 to 4,294,967,295</td>
        </tr>
        <tr>
            <td><code>u64</code></td>
            <td>64-bit unsigned integer</td>
            <td>0 to 18,446,744,073,709,551,615</td>
        </tr>
        <tr>
            <td><code>u128</code></td>
            <td>128-bit unsigned integer</td>
            <td>0 to 340,282,366,920,938,463,463,374,607,431,768,211,455</td>
        </tr>
        <tr>
            <td><code>usize</code></td>
            <td>Pointer-sized unsigned integer (architecture dependent)</td>
            <td><code>0</code> to <code>2^N - 1</code></td>
        </tr>
    </tbody>
</table>

---

## Important Properties of Integer Types

### Size

The size of each integer type in memory is fixed. For example, `i8` and `u8` are always 1 byte, while `i64` and `u64` are always 8 bytes. The `isize` and `usize` types are architecture-dependent, meaning they can be either 4 bytes (32-bit) or 8 bytes (64-bit) depending on the target architecture.

### Range

Each integer type has a specific range of values it can represent. Signed integers can represent both negative and positive numbers, while unsigned integers can only represent positive numbers.

### Overflow

Rust provides safety checks for integer overflow in debug mode. If an arithmetic operation overflows, Rust will panic in debug builds. In release builds, Rust performs two's complement wrapping.

### Literals

Integer literals can be written with type suffixes to specify their type explicitly. For example, `42i8`, `42u32`, or `42isize`.

### Underscores in Numbers

For readability, underscores can be used in numeric literals. For example, `1_000_000` is the same as `1000000`.

---

## Declaring Integer Variables with Explicit Typing

-   In Rust, you can declare variables using the let keyword. To explicitly specify the type of an integer variable, you can use a colon followed by the type after the variable name. Here are some examples:

```Rust
fn main() {
    let int8: i8 = -42;
    let int16: i16 = -32000;
    let int32: i32 = -2_000_000;
    let int64: i64 = -9_000_000_000;
    let int128: i128 = -9_000_000_000_000;


    banner("=", 62, "Int8 Integer Types in Rust" );
    println!("Decimal int8: {},\nBinary int8: {:b},\nOctal int8: {},\nHexadecimal int8: {:x}",
        int8, int8, int8, int8);

    banner("=", 62, "Int16 Integer Types in Rust" );
    println!("Decimal int16: {},\nBinary int16: {:b},\nOctal int16: {},\nHexadecimal int16: {:x}",
        int16, int16, int16, int16);

    banner("=", 62, "Int32 Integer Types in Rust" );
    println!("Decimal Int32: {},\nBinary int32: {:b},\nOctal int32: {},\nHexadecimal int32: {:x}",
        int32, int32, int32, int32);

    banner("=", 62, "Int64 Integer Types in Rust" );
    println!("Decimal int64: {},\nBinary int64: {:b},\nOctal int64: {},\nHexadecimal int64: {:x}",
        int64, int64, int64, int64);

    banner("=", 62, "Int128 Integer Types in Rust" );
    println!("Decimal int128: {},\nBinary int128: {:b},\nOctal int128: {},\nHexadecimal int128: {:x}",
        int128, int128, int128, int128);
}
main();
```

    ==============================================================
                      Int8 Integer Types in Rust
    ==============================================================
    Decimal int8: -42,
    Binary int8: 11010110,
    Octal int8: -42,
    Hexadecimal int8: d6

    ==============================================================
                     Int16 Integer Types in Rust
    ==============================================================
    Decimal int16: -32000,
    Binary int16: 1000001100000000,
    Octal int16: -32000,
    Hexadecimal int16: 8300

    ==============================================================
                     Int32 Integer Types in Rust
    ==============================================================
    Decimal Int32: -2000000,
    Binary int32: 11111111111000010111101110000000,
    Octal int32: -2000000,
    Hexadecimal int32: ffe17b80

    ==============================================================
                     Int64 Integer Types in Rust
    ==============================================================
    Decimal int64: -9000000000,
    Binary int64: 1111111111111111111111111111110111100111100011101110011000000000,
    Octal int64: -9000000000,
    Hexadecimal int64: fffffffde78ee600

    ==============================================================
                     Int128 Integer Types in Rust
    ==============================================================
    Decimal int128: -9000000000000,
    Binary int128: 11111111111111111111111111111111111111111111111111111111111111111111111111111111111101111101000010000110001100100111000000000000,
    Octal int128: -9000000000000,
    Hexadecimal int128: fffffffffffffffffffff7d086327000

```Rust
fn main() {
    let int8: i8 = -42;
    let int16: i16 = -32000;
    let int32: i32 = -2_000_000;
    let int64: i64 = -9_000_000_000;
    let int128: i128 = -9_000_000_000_000;

    banner("=", 62, "Integer Types in Rust, Types and Size in Memory" );

    println!("int8:    {:<15}, type: {:<10}, size: {} bytes", int8, type_of(int8), std::mem::size_of::<i8>());
    println!("int16:   {:<15}, type: {:<10}, size: {} bytes", int16, type_of(int16), std::mem::size_of::<i16>());
    println!("int32:   {:<15}, type: {:<10}, size: {} bytes", int32, type_of(int32), std::mem::size_of::<i32>());
    println!("int64:   {:<15}, type: {:<10}, size: {} bytes", int64, type_of(int64), std::mem::size_of::<i64>());
    println!("int128:  {:<15}, type: {:<10}, size: {} bytes", int128, type_of(int128), std::mem::size_of::<i128>());
}
main();
```

    ==============================================================
           Integer Types in Rust, Types and Size in Memory
    ==============================================================
    int8:    -42            , type: i8        , size: 1 bytes
    int16:   -32000         , type: i16       , size: 2 bytes
    int32:   -2000000       , type: i32       , size: 4 bytes
    int64:   -9000000000    , type: i64       , size: 8 bytes
    int128:  -9000000000000 , type: i128      , size: 16 bytes

### Minimum and Maximum Values of Integer Types in Rust

```Rust
fn main() {
    banner("*", 72, "The MIN and Max Values for Each Integer Data Type");
    // Print min and max values for each type
    print_data_type_info::<i8>("i8", i8::MIN, i8::MAX);
    print_data_type_info::<i16>("i16", i16::MIN, i16::MAX);
    print_data_type_info::<i32>("i32", i32::MIN, i32::MAX);
    print_data_type_info::<i64>("i64", i64::MIN, i64::MAX);
    print_data_type_info::<i128>("i128", i128::MIN, i128::MAX);
}

fn print_data_type_info<T>(type_name: &str, min: T, max: T)
where
    T: std::fmt::Display,
{
    println!(
        "Type: {:<8}, Size: {:<8} bytes, Min: {:<25}, Max: {}",
        type_name,
        std::mem::size_of::<T>(),
        min,
        max
    );
}
main();
```

    ************************************************************************
               The MIN and Max Values for Each Integer Data Type
    ************************************************************************
    Type: i8      , Size: 1        bytes, Min: -128                     , Max: 127
    Type: i16     , Size: 2        bytes, Min: -32768                   , Max: 32767
    Type: i32     , Size: 4        bytes, Min: -2147483648              , Max: 2147483647
    Type: i64     , Size: 8        bytes, Min: -9223372036854775808     , Max: 9223372036854775807
    Type: i128    , Size: 16       bytes, Min: -170141183460469231731687303715884105728, Max: 170141183460469231731687303715884105727

### Unsigned Integer Types in Rust

```Rust
fn main() {
    let uint8: u8 = 42;
    let uint16: u16 = 32000;
    let uint32: u32 = 2_000_000;
    let uint64: u64 = 9_000_000_000;
    let uint128: u128 = 9_000_000_000_000;

    banner("=", 62, "uint8 Integer Types in Rust" );
    println!("Decimal uint8: {},\nBinary uint8: {:b},\nOctal uint8: {},\nHexadecimal: {:x}",
    uint8, uint8, uint8, uint8);

    banner("=", 62, "uint16 Integer Types in Rust" );
    println!("Decimal uint16: {},\nBinary uint16: {:b},\nOctal uint16: {},\nHexadecimal uint16: {:x}",
        uint16, uint16, uint16, uint16);

    banner("=", 62, "uint32 Integer Types in Rust" );
    println!("Decimal uint32: {},\nBinary uint32: {:b},\nOctal uint32: {},\nHexadecimal uint32: {:x}",
        uint32, uint32, uint32, uint32);

    banner("=", 62, "uint64 Integer Types in Rust" );
    println!("Decimal uint64: {},\nBinary uint64: {:b},\nOctal uint64: {},\nHexadecimaluint64: {:x}",
        uint64, uint64, uint64, uint64);

    banner("=", 62, "Int128 Integer Types in Rust" );
    println!("Decimal int128: {},\nBinary uint128: {:b},\nOctal uint128: {},\nHexadecimal uint128: {:x}",
        uint128, uint128, uint128, uint128);
}
main();
```

    ==============================================================
                     uint8 Integer Types in Rust
    ==============================================================
    Decimal uint8: 42,
    Binary uint8: 101010,
    Octal uint8: 42,
    Hexadecimal: 2a

    ==============================================================
                     uint16 Integer Types in Rust
    ==============================================================
    Decimal uint16: 32000,
    Binary uint16: 111110100000000,
    Octal uint16: 32000,
    Hexadecimal uint16: 7d00

    ==============================================================
                     uint32 Integer Types in Rust
    ==============================================================
    Decimal uint32: 2000000,
    Binary uint32: 111101000010010000000,
    Octal uint32: 2000000,
    Hexadecimal uint32: 1e8480

    ==============================================================
                     uint64 Integer Types in Rust
    ==============================================================
    Decimal uint64: 9000000000,
    Binary uint64: 1000011000011100010001101000000000,
    Octal uint64: 9000000000,
    Hexadecimaluint64: 218711a00

    ==============================================================
                     Int128 Integer Types in Rust
    ==============================================================
    Decimal int128: 9000000000000,
    Binary uint128: 10000010111101111001110011011001000000000000,
    Octal uint128: 9000000000000,
    Hexadecimal uint128: 82f79cd9000

### Type and Memory Size of Unsigned Integers

```Rust
fn main() {
    banner("=", 62, "Unsigned Integer Types in Rust" );
    let uint8: u8 = 42;
    let uint16: u16 = 32000;
    let uint32: u32 = 2_000_000;
    let uint64: u64 = 9_000_000_000;
    let uint128: u128 = 9_000_000_000_000;


    println!("uuint8:    {:<15}, type: u8,    size: {:<10} bytes", uint8, std::mem::size_of::<u8>());
    println!("uuint16:   {:<15}, type: u16,   size: {:<10} bytes", uint16, std::mem::size_of::<u16>());
    println!("uuint32:   {:<15}, type: u32,   size: {:<10} bytes", uint32, std::mem::size_of::<u32>());
    println!("uuint64:   {:<15}, type: u64,   size: {:<10} bytes", uint64, std::mem::size_of::<u64>());
    println!("uint128:   {:<15}, type: u128,  size: {:<10} bytes", uint128, std::mem::size_of::<u128>());
}
main();
```

    ==============================================================
                    Unsigned Integer Types in Rust
    ==============================================================
    uuint8:    42             , type: u8,    size: 1          bytes
    uuint16:   32000          , type: u16,   size: 2          bytes
    uuint32:   2000000        , type: u32,   size: 4          bytes
    uuint64:   9000000000     , type: u64,   size: 8          bytes
    uint128:   9000000000000  , type: u128,  size: 16         bytes

```Rust
// Print Data Type Information

fn main() {
    banner("*", 72, "The MIN and Max Values for Each Integer Data Type");
    print_data_type_info::<u8>("u8", u8::MIN, u8::MAX);
    print_data_type_info::<u16>("u16", u16::MIN, u16::MAX);
    print_data_type_info::<u32>("u32", u32::MIN, u32::MAX);
    print_data_type_info::<u64>("u64", u64::MIN, u64::MAX);
    print_data_type_info::<u128>("usize", u128::MIN, u128::MAX);
}
fn print_data_type_info<T>(type_name: &str, min: T, max: T)
where
    T: std::fmt::Display,
{
    println!(
        "Type: {:<8}, Size: {:<8} bytes, Min: {:<5}, Max: {}",
        type_name,
        std::mem::size_of::<T>(),
        min,
        max
    );
}
main();
```

    ************************************************************************
               The MIN and Max Values for Each Integer Data Type
    ************************************************************************
    Type: u8      , Size: 1        bytes, Min: 0    , Max: 255
    Type: u16     , Size: 2        bytes, Min: 0    , Max: 65535
    Type: u32     , Size: 4        bytes, Min: 0    , Max: 4294967295
    Type: u64     , Size: 8        bytes, Min: 0    , Max: 18446744073709551615
    Type: usize   , Size: 16       bytes, Min: 0    , Max: 340282366920938463463374607431768211455

---

### isize and usize in Rust

1. **isize**:

    - **Description**: A pointer-sized signed integer.
    - **Range**: The range of `isize` depends on the architecture of the machine:
        - On a 32-bit system: -2,147,483,648 to 2,147,483,647
        - On a 64-bit system: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    - **Usage**: `isize` is used when you need an integer type that can hold any pointer or array index on the target architecture. It is particularly useful for pointer arithmetic and when working with collections that require indexing, ensuring that the indices are correctly sized for the platform.

2. **usize**:
    - **Description**: A pointer-sized unsigned integer.
    - **Range**: The range of `usize` depends on the architecture of the machine:
        - On a 32-bit system: 0 to 4,294,967,295
        - On a 64-bit system: 0 to 18,446,744,073,709,551,615
    - **Usage**: `usize` is used for indexing and pointer arithmetic. It is the primary type for sizes and indices in collections and memory-related operations. Since it matches the architecture's pointer size, it ensures that operations involving memory addresses and sizes are performed efficiently and correctly.

### Example Usage

Here is an example demonstrating how `isize` and `usize` can be used in Rust:

```Rust
fn main() {

    banner("=", 62, "Int Size Integer Types in Rust, Types and Size in Memory" );

    let signed_index: isize = -10;
    println!("signed_index: {}, type: isize, size: {} bytes", signed_index, std::mem::size_of::<isize>());

    // Example of usize
    let unsigned_index: usize = 10;
    println!("unsigned_index: {}, type: usize, size: {} bytes", unsigned_index, std::mem::size_of::<usize>());

    banner("=", 62, "Int Size Integer Types in Rust, Types and Size in Memory" );
    let intsize_1: isize = -9; // Size depends on the architecture (32-bit or 64-bit)
    let intsize_2: isize = -9_000;
    let intsize_3: isize = -9_000_000;
    let intsize_4: isize = -9_000_000_000;
    let intsize_5: isize = -9_000_000_000_000_000_000;

    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", intsize_1, type_of(intsize_1), std::mem::size_of::<isize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", intsize_2, type_of(intsize_2), std::mem::size_of::<isize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", intsize_3, type_of(intsize_3), std::mem::size_of::<isize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", intsize_4, type_of(intsize_4), std::mem::size_of::<isize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", intsize_5, type_of(intsize_5), std::mem::size_of::<isize>());

    banner("=", 62, "Int Size Integer Types in Rust, Types and Size in Memory" );
    let untsize_1: usize = 9; // Size depends on the architecture (32-bit or 64-bit)
    let untsize_2: usize = 9_000;
    let untsize_3: usize = 9_000_000;
    let untsize_4: usize = 9_000_000_000;
    let untsize_5: usize = 9_000_000_000_000_000_000;

    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", untsize_1, type_of(intsize_1), std::mem::size_of::<usize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", untsize_2, type_of(intsize_2), std::mem::size_of::<usize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", untsize_3, type_of(intsize_3), std::mem::size_of::<usize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", untsize_4, type_of(intsize_4), std::mem::size_of::<usize>());
    println!("intsize: {:<15}, type: {:<10}, size: {} bytes", untsize_5, type_of(intsize_5), std::mem::size_of::<usize>());


    banner("=", 62, "Case usage of Usize Type" );
    // Using usize for array indexing
    let array = [1, 2, 3, 4, 5];
    let index: usize = 2;
    println!("Element at index {}: {}", index, array[index]);
}
main();
```

    ==============================================================
       Int Size Integer Types in Rust, Types and Size in Memory
    ==============================================================
    signed_index: -10, type: isize, size: 8 bytes
    unsigned_index: 10, type: usize, size: 8 bytes

    ==============================================================
       Int Size Integer Types in Rust, Types and Size in Memory
    ==============================================================
    intsize: -9             , type: isize     , size: 8 bytes
    intsize: -9000          , type: isize     , size: 8 bytes
    intsize: -9000000       , type: isize     , size: 8 bytes
    intsize: -9000000000    , type: isize     , size: 8 bytes
    intsize: -9000000000000000000, type: isize     , size: 8 bytes

    ==============================================================
       Int Size Integer Types in Rust, Types and Size in Memory
    ==============================================================
    intsize: 9              , type: isize     , size: 8 bytes
    intsize: 9000           , type: isize     , size: 8 bytes
    intsize: 9000000        , type: isize     , size: 8 bytes
    intsize: 9000000000     , type: isize     , size: 8 bytes
    intsize: 9000000000000000000, type: isize     , size: 8 bytes

    ==============================================================
                       Case usage of Usize Type
    ==============================================================
    Element at index 2: 3

```Rust

```

```Rust
fn main() {
    banner("*", 72, "The MIN and Max Values for Isize and Usize on My machine");
    print_data_type_info::<isize>("isize", isize::MIN, isize::MAX);
    print_data_type_info::<usize>("usize", usize::MIN, usize::MAX);
}

fn print_data_type_info<T>(type_name: &str, min: T, max: T)
where
    T: std::fmt::Display,
{
    println!(
        "Type: {:<8}, Size: {:<8} bytes, Min: {:<25}, Max: {}",
        type_name,
        std::mem::size_of::<T>(),
        min,
        max
    );
}
main();
```

    ************************************************************************
            The MIN and Max Values for Isize and Usize on My machine
    ************************************************************************
    Type: isize   , Size: 8        bytes, Min: -9223372036854775808     , Max: 9223372036854775807
    Type: usize   , Size: 8        bytes, Min: 0                        , Max: 18446744073709551615

## Full Example

```Rust
fn main() {
    banner("=", 62, "Integer Data Types in Rust" );
    // i8: The 8-bit signed integer type
    let uint8: i8 = -42;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uint8,
        type_of(uint8), std::mem::size_of::<i8>());

    // i16: The 16-bit signed integer type
    let uint16: i16 = -32000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uint16,
        type_of(uint16), std::mem::size_of::<i16>());

    // i32: The 32-bit signed integer type
    let uint32: i32 = -2_000_000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uint32,
        type_of(uint32), std::mem::size_of::<i32>());

    // i64: The 64-bit signed integer type
    let uint64: i64 = -9_000_000_000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uint64,
        type_of(uint64), std::mem::size_of::<i64>());

    // isize: The pointer-sized signed integer type
    let intsize: isize = -9_000_000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", intsize,
        type_of(intsize), std::mem::size_of::<isize>());

    // u8: The 8-bit unsigned integer type
    let uuint8: u8 = 42;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uuint8,
        type_of(uuint8), std::mem::size_of::<u8>());

    // u16: The 16-bit unsigned integer type
    let uuint16: u16 = 32000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uuint8,
        type_of(uuint16), std::mem::size_of::<u16>());

    // u32: The 32-bit unsigned integer type
    let uuint32: u32 = 2_000_000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uuint32,
        type_of(uuint32), std::mem::size_of::<u32>());

    // u64: The 64-bit unsigned integer type
    let uuint64: u64 = 9_000_000_000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uuint64,
        type_of(uuint64), std::mem::size_of::<u64>());

    // usize: The pointer-sized unsigned integer type
    let uintsize: usize = 9_000_000;
    println!("Value: {:<15}, Type: {:<10}, Size: {} bytes", uintsize,
        type_of(uintsize), std::mem::size_of::<usize>());
}
main();
```

    ==============================================================
                      Integer Data Types in Rust
    ==============================================================
    Value: -42            , Type: i8        , Size: 1 bytes
    Value: -32000         , Type: i16       , Size: 2 bytes
    Value: -2000000       , Type: i32       , Size: 4 bytes
    Value: -9000000000    , Type: i64       , Size: 8 bytes
    Value: -9000000       , Type: isize     , Size: 8 bytes
    Value: 42             , Type: u8        , Size: 1 bytes
    Value: 42             , Type: u16       , Size: 2 bytes
    Value: 2000000        , Type: u32       , Size: 4 bytes
    Value: 9000000000     , Type: u64       , Size: 8 bytes
    Value: 9000000        , Type: usize     , Size: 8 bytes

```Rust
fn main() {
    banner("*", 72, "The MIN and Max Values for Each Integer Data Type");
    // Print min and max values for each type
    print_data_type_info::<i8>("i8", i8::MIN, i8::MAX);
    print_data_type_info::<i16>("i16", i16::MIN, i16::MAX);
    print_data_type_info::<i32>("i32", i32::MIN, i32::MAX);
    print_data_type_info::<i64>("i64", i64::MIN, i64::MAX);
    print_data_type_info::<isize>("isize", isize::MIN, isize::MAX);
    print_data_type_info::<u8>("u8", u8::MIN, u8::MAX);
    print_data_type_info::<u16>("u16", u16::MIN, u16::MAX);
    print_data_type_info::<u32>("u32", u32::MIN, u32::MAX);
    print_data_type_info::<u64>("u64", u64::MIN, u64::MAX);
    print_data_type_info::<usize>("usize", usize::MIN, usize::MAX);
}

fn print_data_type_info<T>(type_name: &str, min: T, max: T)
where
    T: std::fmt::Display,
{
    println!(
        "Type: {:<8}, Size: {:<8} bytes, Min: {:<25}, Max: {}",
        type_name,
        std::mem::size_of::<T>(),
        min,
        max
    );
}
main();
```

    ************************************************************************
               The MIN and Max Values for Each Integer Data Type
    ************************************************************************
    Type: i8      , Size: 1        bytes, Min: -128                     , Max: 127
    Type: i16     , Size: 2        bytes, Min: -32768                   , Max: 32767
    Type: i32     , Size: 4        bytes, Min: -2147483648              , Max: 2147483647
    Type: i64     , Size: 8        bytes, Min: -9223372036854775808     , Max: 9223372036854775807
    Type: isize   , Size: 8        bytes, Min: -9223372036854775808     , Max: 9223372036854775807
    Type: u8      , Size: 1        bytes, Min: 0                        , Max: 255
    Type: u16     , Size: 2        bytes, Min: 0                        , Max: 65535
    Type: u32     , Size: 4        bytes, Min: 0                        , Max: 4294967295
    Type: u64     , Size: 8        bytes, Min: 0                        , Max: 18446744073709551615
    Type: usize   , Size: 8        bytes, Min: 0                        , Max: 18446744073709551615
