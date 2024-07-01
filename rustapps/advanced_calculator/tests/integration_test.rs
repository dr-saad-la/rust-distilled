use advanced_calculator::calc::{add, divide, multiply, subtract};
use advanced_calculator::calc::{complex, exp, hyperbolic};
use advanced_calculator::calc::{factorial, gcd_lcm};
use advanced_calculator::calc::{log, power, sqrt, trig};
use num_complex::Complex;

#[test]
fn test_add() {
    assert_eq!(add::add(10, 5), 15);
    assert_eq!(add::add(0, 0), 0);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract::subtract(10, 5), 5);
    assert_eq!(subtract::subtract(5, 10), -5);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply::multiply(10, 5), 50);
    assert_eq!(multiply::multiply(0, 5), 0);
}

#[test]
fn test_divide() {
    assert_eq!(divide::divide(10, 5), Some(2));
    assert_eq!(divide::divide(10, 0), None); // Tests division by zero
}

#[test]
fn test_power() {
    assert_eq!(power::power(2.0, 3.0), 8.0);
}

#[test]
fn test_sqrt() {
    assert_eq!(sqrt::sqrt(16.0), 4.0);
}

#[test]
fn test_trig() {
    assert_eq!(trig::sin(0.0), 0.0);
    assert_eq!(trig::cos(0.0), 1.0);
    assert_eq!(trig::tan(0.0), 0.0);
}

#[test]
fn test_log() {
    assert_eq!(log::ln(1.0), 0.0);
    assert_eq!(log::log10(100.0), 2.0);
    assert_eq!(log::log_base(8.0, 2.0), 3.0);
}

#[test]
fn test_exp() {
    assert_eq!(exp::exp(1.0), (1.0 as f64).exp());
}

#[test]
fn test_hyperbolic() {
    assert_eq!(hyperbolic::sinh(1.0), (1.0 as f64).sinh());
    assert_eq!(hyperbolic::cosh(1.0), (1.0 as f64).cosh());
    assert_eq!(hyperbolic::tanh(1.0), (1.0 as f64).tanh());
}

#[test]
fn test_complex() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let result = complex::add_complex(a, b);
    assert_eq!(result, Complex::new(4.0, 6.0));
}

#[test]
fn test_factorial() {
    assert_eq!(factorial::factorial(5), 120);
}

#[test]
fn test_gcd_lcm() {
    assert_eq!(gcd_lcm::gcd(48, 18), 6);
    assert_eq!(gcd_lcm::lcm(12, 15), 60);
}
