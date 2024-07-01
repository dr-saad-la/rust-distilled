#![allow(unused_imports)]

mod calc;
mod linalg;
mod plots;
mod stats;
mod tools;
mod utils;

use calc::{add, divide, log, multiply, power, sqrt, subtract, trig};
use calc::{complex, exp, hyperbolic, hyperbolic_inverse, inverse_trig};
use calc::{factorial, gcd_lcm};
use calc::{fourier, polynomial};
use linalg::{matrix, vector};
use num_complex::Complex;
use tools::banner;

use plots::histogram;
use stats::{correlation, mean, median, mode, std};
use stats::{kurtosis, skewness, variance};
use utils::constants::{E, PI};
use utils::conversion::{deg_to_rad, rad_to_deg};
use utils::num_methods::derivative;

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

    let radians = deg_to_rad(180.0);
    let degrees = rad_to_deg(PI);
    let derivative_result = derivative(|x| x.powi(2), 2.0, 1e-5);

    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mean_result = mean::mean(&data);
    let median_result = median::median(&data);
    let mode_result = mode::mode(&data);
    let stddev_result = std::std(&data);

    let x = [1.0, 2.0, 3.0];
    let y = [1.0, 2.0, 3.0];
    let correlation_result = correlation::corr(&x, &y);

    // Vector
    let vector_a = vector::Vector::new(vec![1.0, 2.0, 3.0]);
    let vector_b = vector::Vector::new(vec![4.0, 5.0, 6.0]);
    let vector_cross = vector_a.cross(&vector_b);
    println!("Vector Cross Product: {:?}", vector_cross);

    let polynomial_a = polynomial::Polynomial::new(vec![1.0, -3.0, 2.0]);
    let polynomial_value = polynomial_a.evaluate(2.0);
    println!("Polynomial Evaluation at x=2: {}", polynomial_value);

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

    banner("*", 62, "Conversion Operations");
    println!("Radians: {}", radians);
    println!("Degrees: {}", degrees);
    println!("Derivative: {}", derivative_result);

    banner("*", 62, "Statistical Operations");
    println!("Mean: {}", mean_result);
    println!("Median: {}", median_result);
    println!("Mode: {:?}", mode_result);
    println!("Standard Deviation: {}", stddev_result);
    println!("Correlation: {}", correlation_result);

    // Print a welcome banner
    banner("*", 30, "Advanced Calculator");

    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mean_result = mean::mean(&data);
    let median_result = median::median(&data);
    let mode_result = mode::mode(&data);
    let stddev_result = std::std(&data);
    let variance_result = variance::variance(&data);
    let skewness_result = skewness::skewness(&data);
    let kurtosis_result = kurtosis::kurtosis(&data);
    let histogram_result = histogram::histogram(&data, 5);

    println!("Mean: {}", mean_result);
    println!("Median: {}", median_result);
    println!("Mode: {:?}", mode_result);
    println!("Standard Deviation: {}", stddev_result);
    println!("Variance: {}", variance_result);
    println!("Skewness: {}", skewness_result);
    println!("Kurtosis: {}", kurtosis_result);
    println!("Histogram: {:?}", histogram_result);

    // Example usage of new features
    let matrix_a = matrix::Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let matrix_b = matrix::Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    let matrix_sum = matrix_a.add(&matrix_b);
    println!("Matrix Sum: {:?}", matrix_sum);

    let vector_a = vector::Vector::new(vec![1.0, 2.0, 3.0]);
    let vector_b = vector::Vector::new(vec![4.0, 5.0, 6.0]);
    let vector_cross = vector_a.cross(&vector_b);
    println!("Vector Cross Product: {:?}", vector_cross);

    let polynomial_a = polynomial::Polynomial::new(vec![1.0, -3.0, 2.0]);
    let polynomial_value = polynomial_a.evaluate(2.0);
    println!("Polynomial Evaluation at x=2: {}", polynomial_value);

    let complex_input = vec![
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(-1.0, 0.0),
    ];
    let dft_output = fourier::dft(&complex_input);
    println!("DFT Output: {:?}", dft_output);
}
