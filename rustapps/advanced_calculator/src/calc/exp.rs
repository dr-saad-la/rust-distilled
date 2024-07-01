/// Calculates the exponential of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The exponent.
///
/// # Returns
///
/// The exponential of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::exp::exp;
/// let result = exp(2.0);
/// assert_eq!(result, (2.0 as f64).exp());
/// ```
pub fn exp(a: f64) -> f64 {
    a.exp()
}
