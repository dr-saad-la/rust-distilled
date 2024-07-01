/// Calculates the standard deviation of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// The standard deviation of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::std::std;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = std(&data);
/// assert_eq!(result, 1.5811388300841898);
/// ```
pub fn std(data: &[f64]) -> f64 {
    let mean = super::mean::mean(data);
    let variance = data
        .iter()
        .map(|value| {
            let diff = mean - *value;
            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64;

    variance.sqrt()
}
