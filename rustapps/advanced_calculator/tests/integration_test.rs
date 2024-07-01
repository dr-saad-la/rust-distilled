use advanced_calculator::calc::{add, divide, multiply, subtract};
use advanced_calculator::calc::{log, power, sqrt, trig};

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
}
