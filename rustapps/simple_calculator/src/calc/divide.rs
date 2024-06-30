pub fn divide(a: i32, b: i32) -> i32 {
    if b != 0 {
        a / b
    } else {
        println!("Error: Division by zero");
        0
    }
}
