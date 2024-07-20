use crate::banner;

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
