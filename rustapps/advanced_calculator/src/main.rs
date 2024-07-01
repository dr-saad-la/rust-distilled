mod calc;
mod tools;

use calc::{add, divide, log, multiply, power, sqrt, subtract, trig};
use tools::banner;

fn main() {
    let a = 10.0;
    let b = 5.0;

    // Print a welcome banner
    banner("*", 30, "Advanced Calculator");

    let sum = add::add(a as i32, b as i32);
    let difference = subtract::subtract(a as i32, b as i32);
    let product = multiply::multiply(a as i32, b as i32);
    let quotient = divide::divide(a as i32, b as i32);

    let power_result = power::power(a, b);
    let sqrt_result = sqrt::sqrt(a);
    let sine = trig::sin(a);
    let cosine = trig::cos(a);
    let tangent = trig::tan(a);
    let natural_log = log::ln(a);
    let base10_log = log::log10(a);

    println!("{:<20} {}", "sum:", sum);
    println!("{:<20} {}", "Difference:", difference);
    println!("{:<20} {}", "Product:", product);

    match quotient {
        Some(q) => println!("{:<20} {}", "Quotient:", q),
        None => println!("Cannot divide by zero"),
    }

    banner("*", 32, "Advanced operations");

    println!("Power: {:<20}", power_result);
    println!("Square Root: {:<20}", sqrt_result);
    println!("Sine: {:<20}", sine);
    println!("Cosine: {:<20}", cosine);
    println!("Tangent: {:<20}", tangent);
    println!("Natural Log: {:<20}", natural_log);
    println!("Base-10 Log: {:<20}", base10_log);
}
