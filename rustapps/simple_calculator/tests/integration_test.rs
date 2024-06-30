use simple_calculator::calc;

#[test]
fn test_add() {
    assert_eq!(calc::add::add(10, 5), 15);
    assert_eq!(calc::add::add(0, 0), 0);
}

#[test]
fn test_subtract() {
    assert_eq!(calc::subtract::subtract(10, 5), 5);
    assert_eq!(calc::subtract::subtract(5, 10), -5);
}

#[test]
fn test_multiply() {
    assert_eq!(calc::multiply::multiply(10, 5), 50);
    assert_eq!(calc::multiply::multiply(0, 5), 0);
}

#[test]
fn test_divide() {
    assert_eq!(calc::divide::divide(10, 5), 2);
    assert_eq!(calc::divide::divide(10, 0), 0); // Tests division by zero
}
