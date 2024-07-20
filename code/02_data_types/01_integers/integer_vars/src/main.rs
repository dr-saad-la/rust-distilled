//================================================================================
//      Author:         Dr. Saad Laouadi
//      Course:         Rust Distilled
//      Chapter 02:     Primitive Data Types
//      Section:        Numeric Data
//      lesson:         Integers
//
//      © copy-right: Dr. Saad Laouadi
//================================================================================

mod int_operations;
mod ints_info;
mod rnd_integers;
mod unsigned_ints;

fn main() {
    // Invoke the print_signed_integers() function
    banner("*", 52, "Integers in Rust");
    print_signed_integers();
    make_separator(52);
    signed_integers_size();
    make_separator(52);

    // Invoke unsigned functions
    print_unsigned_integers();
    make_separator(52);
    unsigned_integers_size();

    banner("*", 52, "Generate Random Integers");
    // Generate a random signed integers
    rnd_integers::print_rnd_integers();

    // Integer Operations
    make_separator(52);
    int_operations::add_ints();
    int_operations::sub_ints();
    int_operations::mul_ints();
    int_operations::divide_ints();
    int_operations::remainder_ints();
    banner("*", 52, "Detailed Information about Integers in Rust;;;");
    ints_info::ints_types();

    ints_info::ints_info();

    ints_info::min_max_ints();

    unsigned_ints::get_unsigned_ints_info();
}

// a function that prints signed integers
fn print_signed_integers() {
    let i8_val: i8 = -123;
    let i16_val: i16 = -12345;
    let i32_val: i32 = -12345678;
    let i64_val: i64 = -1234567890;

    println!("i8 : {}", i8_val);
    println!("i16: {}", i16_val);
    println!("i32: {}", i32_val);
    println!("i64: {}", i64_val);
}

fn signed_integers_size() {
    let i8_val: i8 = -123;
    let i16_val: i16 = -12345;
    let i32_val: i32 = -12345678;
    let i64_val: i64 = -1234567890;

    println!(
        "i8 : {:<12} (Size: {} bytes)",
        i8_val,
        std::mem::size_of::<i8>()
    );
    println!(
        "i16: {:<12} (Size: {} bytes)",
        i16_val,
        std::mem::size_of::<i16>()
    );
    println!(
        "i32: {:<12} (Size: {} bytes)",
        i32_val,
        std::mem::size_of::<i32>()
    );
    println!(
        "i64: {:<12} (Size: {} bytes)",
        i64_val,
        std::mem::size_of::<i64>()
    );
}

// A function that print unsigned integers
fn print_unsigned_integers() {
    let u8_val: u8 = 123;
    let u16_val: u16 = 12345;
    let u32_val: u32 = 12345678;
    let u64_val: u64 = 1234567890;

    println!("u8: {}", u8_val);
    println!("u16: {}", u16_val);
    println!("u32: {}", u32_val);
    println!("u64: {}", u64_val);
}

fn unsigned_integers_size() {
    let u8_val: u8 = 123;
    let u16_val: u16 = 12345;
    let u32_val: u32 = 12345678;
    let u64_val: u64 = 1234567890;

    println!(
        "u8: {:<12} (Size: {} bytes)",
        u8_val,
        std::mem::size_of::<u8>()
    );
    println!(
        "u16: {:<12} (Size: {} bytes)",
        u16_val,
        std::mem::size_of::<u16>()
    );
    println!(
        "u32: {:<12} (Size: {} bytes)",
        u32_val,
        std::mem::size_of::<u32>()
    );
    println!(
        "u64: {:<12} (Size: {} bytes)",
        u64_val,
        std::mem::size_of::<u64>()
    );
}

// Function to create a formatted banner
fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}

// make separator function
fn make_separator(nchar: usize) {
    println!("{}", "*".repeat(nchar))
}
