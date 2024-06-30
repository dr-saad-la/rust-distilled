mod calc;
use calc::{add, divide, multiply, subtract};

fn main() {
    let a = 10;
    let b = 5;

    let sum = add::add(a, b);
    let difference = subtract::subtract(a, b);
    let product = multiply::multiply(a, b);
    let quotient = divide::divide(a, b);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);

    match quotient {
        Some(q) => println!("Quotient: {}", q),
        None => println!("Cannot divide by zero"),
    }
}
