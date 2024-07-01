/// Calculates the median of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// The median of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::median::median;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = median(&data);
/// assert_eq!(result, 3.0);
/// ```
pub fn median(data: &[f64]) -> f64 {
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted_data.len();
    if len % 2 == 0 {
        (sorted_data[len / 2 - 1] + sorted_data[len / 2]) / 2.0
    } else {
        sorted_data[len / 2]
    }
}
