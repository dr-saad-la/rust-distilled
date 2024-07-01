/// Computes the square root of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The number to compute the square root of.
///
/// # Returns
///
/// The square root of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::sqrt::sqrt;
/// let result = sqrt(16.0);
/// assert_eq!(result, 4.0);
/// ```
pub fn sqrt(a: f64) -> f64 {
    a.sqrt()
}
