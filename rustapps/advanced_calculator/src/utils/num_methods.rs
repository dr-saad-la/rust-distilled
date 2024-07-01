/// Calculates the derivative of a function at a given point using central difference.
///
/// # Arguments
///
/// * `f` - The function to differentiate.
/// * `x` - The point at which to calculate the derivative.
/// * `h` - A small step size.
///
/// # Returns
///
/// The derivative of the function at the given point.
///
/// # Examples
///
/// ```
/// use advanced_calculator::utils::numerical_methods::derivative;
/// let f = |x: f64| x.powi(2);
/// let result = derivative(f, 2.0, 1e-5);
/// assert_eq!(result, 4.0);
/// ```
pub fn derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}
