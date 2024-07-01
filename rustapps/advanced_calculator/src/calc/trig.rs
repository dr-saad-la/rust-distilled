/// Computes the sine of `a` (in radians) and returns the result.
///
/// # Arguments
///
/// * `a` - The angle in radians.
///
/// # Returns
///
/// The sine of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::trig::sin;
/// let result = sin(0.0);
/// assert_eq!(result, 0.0);
/// ```
pub fn sin(a: f64) -> f64 {
    a.sin()
}

/// Computes the cosine of `a` (in radians) and returns the result.
///
/// # Arguments
///
/// * `a` - The angle in radians.
///
/// # Returns
///
/// The cosine of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::trig::cos;
/// let result = cos(0.0);
/// assert_eq!(result, 1.0);
/// ```
pub fn cos(a: f64) -> f64 {
    a.cos()
}

/// Computes the tangent of `a` (in radians) and returns the result.
///
/// # Arguments
///
/// * `a` - The angle in radians.
///
/// # Returns
///
/// The tangent of `a`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::trig::tan;
/// let result = tan(0.0);
/// assert_eq!(result, 0.0);
/// ```
pub fn tan(a: f64) -> f64 {
    a.tan()
}
