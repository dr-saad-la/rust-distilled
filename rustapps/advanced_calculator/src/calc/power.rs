/// Raises `a` to the power of `b` and returns the result.
///
/// # Arguments
///
/// * `a` - The base.
/// * `b` - The exponent.
///
/// # Returns
///
/// The result of raising `a` to the power of `b`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::power::power;
/// let result = power(2.0, 3.0);
/// assert_eq!(result, 8.0);
/// ```
pub fn power(a: f64, b: f64) -> f64 {
    a.powf(b)
}
