/// Calculates the greatest common divisor (GCD) of `a` and `b` and returns the result.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The GCD of `a` and `b`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::gcd_lcm::gcd;
/// let result = gcd(48, 18);
/// assert_eq!(result, 6);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calculates the least common multiple (LCM) of `a` and `b` and returns the result.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The LCM of `a` and `b`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::gcd_lcm::lcm;
/// let result = lcm(12, 15);
/// assert_eq!(result, 60);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
