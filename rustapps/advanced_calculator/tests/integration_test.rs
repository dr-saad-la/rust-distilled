use advanced_calculator::calc::{add, divide, multiply, subtract};

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
