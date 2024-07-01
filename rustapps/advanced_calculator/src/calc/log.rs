/// Computes the natural logarithm of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The number to compute the natural logarithm of.
///
/// # Returns
///
/// The natural logarithm of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::log::ln;
/// let result = ln(1.0);
/// assert_eq!(result, 0.0);
/// ```
pub fn ln(a: f64) -> f64 {
    a.ln()
}

/// Computes the base-10 logarithm of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The number to compute the base-10 logarithm of.
///
/// # Returns
///
/// The base-10 logarithm of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::log::log10;
/// let result = log10(100.0);
/// assert_eq!(result, 2.0);
/// ```
pub fn log10(a: f64) -> f64 {
    a.log10()
}

/// Calculates the logarithm of `a` with base `b` and returns the result.
///
/// # Arguments
///
/// * `a` - The number to calculate the logarithm for.
/// * `b` - The base of the logarithm.
///
/// # Returns
///
/// The logarithm of `a` with base `b`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::log::log_base;
/// let result = log_base(8.0, 2.0);
/// assert_eq!(result, 3.0);
/// ```
pub fn log_base(a: f64, b: f64) -> f64 {
    a.log(b)
}
