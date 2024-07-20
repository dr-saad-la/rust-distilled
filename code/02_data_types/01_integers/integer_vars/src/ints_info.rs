#![allow(dead_code)]
use crate::banner;
use std::any::type_name;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn ints_types() {
    let int8: i8 = -42;
    let int16: i16 = -32000;
    let int32: i32 = -2_000_000;
    let int64: i64 = -9_000_000_000;
    let int128: i128 = -9_000_000_000_000;

    banner("=", 62, "Int8 Integer Types in Rust");
    println!(
        "Decimal int8: {},\nBinary int8: {:b},\nOctal int8: {},\nHexadecimal int8: {:x}",
        int8, int8, int8, int8
    );

    banner("=", 62, "Int16 Integer Types in Rust");
    println!(
        "Decimal int16: {},\nBinary int16: {:b},\nOctal int16: {},\nHexadecimal int16: {:x}",
        int16, int16, int16, int16
    );

    banner("=", 62, "Int32 Integer Types in Rust");
    println!(
        "Decimal Int32: {},\nBinary int32: {:b},\nOctal int32: {},\nHexadecimal int32: {:x}",
        int32, int32, int32, int32
    );

    banner("=", 62, "Int64 Integer Types in Rust");
    println!(
        "Decimal int64: {},\nBinary int64: {:b},\nOctal int64: {},\nHexadecimal int64: {:x}",
        int64, int64, int64, int64
    );

    banner("=", 62, "Int128 Integer Types in Rust");
    println!(
        "Decimal int128: {},\nBinary int128: {:b},\nOctal int128: {},\nHexadecimal int128: {:x}",
        int128, int128, int128, int128
    );
}

pub fn ints_info() {
    let int8: i8 = -42;
    let int16: i16 = -32000;
    let int32: i32 = -2_000_000;
    let int64: i64 = -9_000_000_000;
    let int128: i128 = -9_000_000_000_000;

    banner("=", 62, "Integer Types in Rust, Types and Size in Memory");

    println!(
        "int8:    {:<15}, type: {:<10}, size: {} bytes",
        int8,
        type_of(int8),
        std::mem::size_of::<i8>()
    );
    println!(
        "int16:   {:<15}, type: {:<10}, size: {} bytes",
        int16,
        type_of(int16),
        std::mem::size_of::<i16>()
    );
    println!(
        "int32:   {:<15}, type: {:<10}, size: {} bytes",
        int32,
        type_of(int32),
        std::mem::size_of::<i32>()
    );
    println!(
        "int64:   {:<15}, type: {:<10}, size: {} bytes",
        int64,
        type_of(int64),
        std::mem::size_of::<i64>()
    );
    println!(
        "int128:  {:<15}, type: {:<10}, size: {} bytes",
        int128,
        type_of(int128),
        std::mem::size_of::<i128>()
    );
}

pub fn min_max_ints() {
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
