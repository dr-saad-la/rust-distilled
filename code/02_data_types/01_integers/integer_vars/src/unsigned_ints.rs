use crate::banner;
use crate::ints_info::type_of;

pub fn get_unsigned_ints_info() {
    let uint8: u8 = 42;
    let uint16: u16 = 32000;
    let uint32: u32 = 2_000_000;
    let uint64: u64 = 9_000_000_000;
    let uint128: u128 = 9_000_000_000_000;

    banner("=", 62, "uint8 Integer Types in Rust");
    println!(
        "Decimal uint8: {},\nBinary uint8: {:b},\nOctal uint8: {},\nHexadecimal: {:x}",
        uint8, uint8, uint8, uint8
    );

    banner("=", 62, "uint16 Integer Types in Rust");
    println!(
        "Decimal uint16: {},\nBinary uint16: {:b},\nOctal uint16: {},\nHexadecimal uint16: {:x}",
        uint16, uint16, uint16, uint16
    );

    banner("=", 62, "uint32 Integer Types in Rust");
    println!(
        "Decimal uint32: {},\nBinary uint32: {:b},\nOctal uint32: {},\nHexadecimal uint32: {:x}",
        uint32, uint32, uint32, uint32
    );

    banner("=", 62, "uint64 Integer Types in Rust");
    println!(
        "Decimal uint64: {},\nBinary uint64: {:b},\nOctal uint64: {},\nHexadecimaluint64: {:x}",
        uint64, uint64, uint64, uint64
    );

    banner("=", 62, "Int128 Integer Types in Rust");
    println!(
        "Decimal int128: {},\nBinary uint128: {:b},\nOctal uint128: {},\nHexadecimal uint128: {:x}",
        uint128, uint128, uint128, uint128
    );
}

pub fn unsigned_type_and_memsize() {
    banner(
        "=",
        62,
        "Ints ISize Integer Types in Rust, Types and Size in Memory",
    );

    let signed_index: isize = -10;
    println!(
        "signed_index: {}, type: isize, size: {} bytes",
        signed_index,
        std::mem::size_of::<isize>()
    );

    // Example of usize
    let unsigned_index: usize = 10;
    println!(
        "unsigned_index: {}, type: usize, size: {} bytes",
        unsigned_index,
        std::mem::size_of::<usize>()
    );

    banner(
        "=",
        62,
        "Int Size Integer Types in Rust, Types and Size in Memory",
    );
    let intsize_1: isize = -9; // Size depends on the architecture (32-bit or 64-bit)
    let intsize_2: isize = -9_000;
    let intsize_3: isize = -9_000_000;
    let intsize_4: isize = -9_000_000_000;
    let intsize_5: isize = -9_000_000_000_000_000_000;

    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        intsize_1,
        type_of(intsize_1),
        std::mem::size_of::<isize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        intsize_2,
        type_of(intsize_2),
        std::mem::size_of::<isize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        intsize_3,
        type_of(intsize_3),
        std::mem::size_of::<isize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        intsize_4,
        type_of(intsize_4),
        std::mem::size_of::<isize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        intsize_5,
        type_of(intsize_5),
        std::mem::size_of::<isize>()
    );

    banner(
        "=",
        62,
        "Int Size Integer Types in Rust, Types and Size in Memory",
    );
    let untsize_1: usize = 9; // Size depends on the architecture (32-bit or 64-bit)
    let untsize_2: usize = 9_000;
    let untsize_3: usize = 9_000_000;
    let untsize_4: usize = 9_000_000_000;
    let untsize_5: usize = 9_000_000_000_000_000_000;

    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        untsize_1,
        type_of(intsize_1),
        std::mem::size_of::<usize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        untsize_2,
        type_of(intsize_2),
        std::mem::size_of::<usize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        untsize_3,
        type_of(intsize_3),
        std::mem::size_of::<usize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        untsize_4,
        type_of(intsize_4),
        std::mem::size_of::<usize>()
    );
    println!(
        "intsize: {:<15}, type: {:<10}, size: {} bytes",
        untsize_5,
        type_of(intsize_5),
        std::mem::size_of::<usize>()
    );

    banner("=", 62, "Case usage of Usize Type");
    // Using usize for array indexing
    let array = [1, 2, 3, 4, 5];
    let index: usize = 2;
    println!("Element at index {}: {}", index, array[index]);
}
