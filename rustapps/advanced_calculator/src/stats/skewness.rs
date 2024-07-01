/// Calculates the skewness of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// The skewness of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::skewness::skewness;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = skewness(&data);
/// assert!((result - 0.0).abs() < 1e-10);
/// ```
pub fn skewness(data: &[f64]) -> f64 {
    let mean = super::mean::mean(data);
    let n = data.len() as f64;
    let m3 = data
        .iter()
        .map(|value| {
            let diff = *value - mean;
            diff * diff * diff
        })
        .sum::<f64>()
        / n;
    let m2 = data
        .iter()
        .map(|value| {
            let diff = *value - mean;
            diff * diff
        })
        .sum::<f64>()
        / n;
    m3 / m2.powf(1.5)
}
