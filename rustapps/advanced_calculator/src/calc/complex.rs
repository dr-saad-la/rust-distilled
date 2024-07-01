use num_complex::Complex;

/// Adds two complex numbers and returns the result.
///
/// # Arguments
///
/// * `a` - The first complex number.
/// * `b` - The second complex number.
///
/// # Returns
///
/// The sum of `a` and `b`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::calc::complex::add_complex;
/// use num_complex::Complex;
/// let a = Complex::new(1.0, 2.0);
/// let b = Complex::new(3.0, 4.0);
/// let result = add_complex(a, b);
/// assert_eq!(result, Complex::new(4.0, 6.0));
/// ```
pub fn add_complex(a: Complex<f64>, b: Complex<f64>) -> Complex<f64> {
    a + b
}
