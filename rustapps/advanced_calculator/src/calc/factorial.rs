/// Calculates the factorial of `n` and returns the result.
///
/// # Arguments
///
/// * `n` - The number to calculate the factorial for.
///
/// # Returns
///
/// The factorial of `n`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::factorial::factorial;
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
