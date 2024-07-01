use advanced_calculator::calc::{add, divide, multiply, power, sqrt, subtract};
use advanced_calculator::calc::{complex, exp};
use advanced_calculator::calc::{factorial, gcd_lcm, hyperbolic, log, trig};
use advanced_calculator::plots::histogram;
use advanced_calculator::stats::{kurtosis, mean, median, mode, skewness, std, variance};
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
    assert_eq!(sqrt::sqrt(4.0), 2.0);
    assert_eq!(sqrt::sqrt(9.0), 3.0);
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

#[test]
fn test_mean() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(mean::mean(&data), 3.0);
}

#[test]
fn test_median() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(median::median(&data), 3.0);
}

#[test]
fn test_mode() {
    let data = [1.0, 2.0, 2.0, 3.0, 4.0];
    assert_eq!(mode::mode(&data), vec![2.0]);
}

#[test]
fn test_std() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert!((std::std(&data) - 1.5811388300841898).abs() < 1e-10); // approximate value
}

#[test]
fn test_variance() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert!((variance::variance(&data) - 2.5).abs() < 1e-10); // approximate value
}

#[test]
fn test_skewness() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert!((skewness::skewness(&data) - 0.0).abs() < 1e-10); // approximately 0
}

#[test]
fn test_kurtosis() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let expected_kurtosis = -1.3;
    assert!((kurtosis::kurtosis(&data) - expected_kurtosis).abs() < 0.1);
}

#[test]
fn test_histogram() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let result = histogram::histogram(&data, 5);
    assert_eq!(result.len(), 5);
}
