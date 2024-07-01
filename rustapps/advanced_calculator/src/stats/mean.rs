/// Calculates the mean of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// The mean of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::mean::mean;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = mean(&data);
/// assert_eq!(result, 3.0);
/// ```
pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}
