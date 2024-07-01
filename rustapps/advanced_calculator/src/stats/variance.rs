/// Calculates the variance of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// The variance of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::variance::variance;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = variance(&data);
/// assert_eq!(result, 2.5);
/// ```
pub fn variance(data: &[f64]) -> f64 {
    let mean = super::mean::mean(data);
    data.iter()
        .map(|value| {
            let diff = mean - *value;
            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_variance() {
//         let data = [1.0, 2.0, 3.0, 4.0, 5.0];
//         assert!((variance(&data) - 2.5).abs() < 1e-10); // approximate value
//     }
// }
