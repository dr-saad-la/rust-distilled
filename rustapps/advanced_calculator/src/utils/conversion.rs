/// Converts degrees to radians.
///
/// # Arguments
///
/// * `degrees` - The angle in degrees.
///
/// # Returns
///
/// The angle in radians.
///
/// # Examples
///
/// ```
/// use advanced_calculator::utils::conversions::deg_to_rad;
/// let result = deg_to_rad(180.0);
/// assert_eq!(result, std::f64::consts::PI);
/// ```
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

/// Converts radians to degrees.
///
/// # Arguments
///
/// * `radians` - The angle in radians.
///
/// # Returns
///
/// The angle in degrees.
///
/// # Examples
///
/// ```
/// use advanced_calculator::utils::conversions::rad_to_deg;
/// let result = rad_to_deg(std::f64::consts::PI);
/// assert_eq!(result, 180.0);
/// ```
pub fn rad_to_deg(radians: f64) -> f64 {
    radians * (180.0 / std::f64::consts::PI)
}
