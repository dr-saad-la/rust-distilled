mod calc;
mod tools;

use calc::{add, divide, log, multiply, power, sqrt, subtract, trig};
use calc::{complex, exp, hyperbolic, hyperbolic_inverse, inverse_trig};
use calc::{factorial, gcd_lcm};
use num_complex::Complex;
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
    let exp_result = exp::exp(a);
    let sinh_result = hyperbolic::sinh(a);
    let cosh_result = hyperbolic::cosh(a);
    let tanh_result = hyperbolic::tanh(a);

    let asin_result = inverse_trig::asin(a);
    let acos_result = inverse_trig::acos(a);
    let atan_result = inverse_trig::atan(a);

    let asinh_result = hyperbolic_inverse::asinh(a);
    let acosh_result = hyperbolic_inverse::acosh(a);
    let atanh_result = hyperbolic_inverse::atanh(a);

    let log_base_result = log::log_base(a, b);
    let factorial_result = factorial::factorial(5);
    let gcd_result = gcd_lcm::gcd(48, 18);
    let lcm_result = gcd_lcm::lcm(12, 15);

    let complex_a = Complex::new(1.0, 2.0);
    let complex_b = Complex::new(3.0, 4.0);
    let complex_sum = complex::add_complex(complex_a, complex_b);

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

    println!("Exponential: {}", exp_result);
    println!("Hyperbolic Sine: {}", sinh_result);
    println!("Hyperbolic Cosine: {}", cosh_result);
    println!("Hyperbolic Tangent: {}", tanh_result);
    println!("Complex Sum: {}", complex_sum);

    println!("Inverse Sine: {}", asin_result);
    println!("Inverse Cosine: {}", acos_result);
    println!("Inverse Tangent: {}", atan_result);

    println!("Hyperbolic Inverse Sine: {}", asinh_result);
    println!("Hyperbolic Inverse Cosine: {}", acosh_result);
    println!("Hyperbolic Inverse Tangent: {}", atanh_result);
    println!("Log Base-n: {}", log_base_result);
    println!("Factorial: {}", factorial_result);
    println!("GCD: {}", gcd_result);
    println!("LCM: {}", lcm_result);
    println!("Complex Sum: {}", complex_sum);
}
