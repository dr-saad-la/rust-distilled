#![allow(unused_imports)]
#![allow(dead_code)]
use crate::stats::mean;
use std::collections::HashMap;
/// Calculates the kurtosis of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// The kurtosis of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::kurtosis::kurtosis;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = kurtosis(&data);
/// assert!((result - 1.7).abs() < 0.1); // approximate value
/// ```
pub fn kurtosis(data: &[f64]) -> f64 {
    let mean = mean::mean(data);
    let n = data.len() as f64;
    let m4 = data
        .iter()
        .map(|value| {
            let diff = *value - mean;
            diff * diff * diff * diff
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
    m4 / m2.powf(2.0) - 3.0
}
