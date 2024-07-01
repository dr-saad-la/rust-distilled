/// Divides the first integer by the second and returns the result as an `Option`.
///
/// # Arguments
///
/// * `a` - The dividend.
/// * `b` - The divisor.
///
/// # Returns
///
/// An `Option<i32>` which is:
/// - `Some(quotient)` if `b` is not zero.
/// - `None` if `b` is zero, to prevent division by zero.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::divide::divide;
/// let result = divide(10, 2);
/// assert_eq!(result, Some(5));
///
/// let result = divide(10, 0);
/// assert_eq!(result, None);
/// ```
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
