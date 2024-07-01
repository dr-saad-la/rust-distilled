/// Calculates the hyperbolic sine of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The angle in radians.
///
/// # Returns
///
/// The hyperbolic sine of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::hyperbolic::sinh;
/// let result = sinh(1.0);
/// assert_eq!(result, (1.0 as f64).sinh());
/// ```
pub fn sinh(a: f64) -> f64 {
    a.sinh()
}

/// Calculates the hyperbolic cosine of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The angle in radians.
///
/// # Returns
///
/// The hyperbolic cosine of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::hyperbolic::cosh;
/// let result = cosh(1.0);
/// assert_eq!(result, (1.0 as f64).cosh());
/// ```
pub fn cosh(a: f64) -> f64 {
    a.cosh()
}

/// Calculates the hyperbolic tangent of `a` and returns the result.
///
/// # Arguments
///
/// * `a` - The angle in radians.
///
/// # Returns
///
/// The hyperbolic tangent of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::hyperbolic::tanh;
/// let result = tanh(1.0);
/// assert_eq!(result, (1.0 as f64).tanh());
/// ```
pub fn tanh(a: f64) -> f64 {
    a.tanh()
}
