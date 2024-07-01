/// Calculates the arcsine (inverse sine) of `a` and returns the result in radians.
///
/// # Arguments
///
/// * `a` - The value to calculate the arcsine for.
///
/// # Returns
///
/// The arcsine of `a` in radians.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::inverse_trig::asin;
/// let result = asin(1.0);
/// assert_eq!(result, 1.0_f64.asin());
/// ```
pub fn asin(a: f64) -> f64 {
    a.asin()
}

/// Calculates the arccosine (inverse cosine) of `a` and returns the result in radians.
///
/// # Arguments
///
/// * `a` - The value to calculate the arccosine for.
///
/// # Returns
///
/// The arccosine of `a` in radians.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::inverse_trig::acos;
/// let result = acos(1.0);
/// assert_eq!(result, 1.0_f64.acos());
/// ```
pub fn acos(a: f64) -> f64 {
    a.acos()
}

/// Calculates the arctangent (inverse tangent) of `a` and returns the result in radians.
///
/// # Arguments
///
/// * `a` - The value to calculate the arctangent for.
///
/// # Returns
///
/// The arctangent of `a` in radians.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::inverse_trig::atan;
/// let result = atan(1.0);
/// assert_eq!(result, 1.0_f64.atan());
/// ```
pub fn atan(a: f64) -> f64 {
    a.atan()
}
