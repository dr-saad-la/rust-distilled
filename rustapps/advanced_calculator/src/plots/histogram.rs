#![allow(unused_imports)]
#![allow(dead_code)]
use crate::stats::mean;
use std::collections::HashMap;

/// Calculates the histogram of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
/// * `bin_count` - The number of bins.
///
/// # Returns
///
/// A vector of (bin, count) pairs representing the histogram.
///
/// # Examples
///
/// ```
/// use advanced_calculator::plots::histogram::histogram;
/// let data = [1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = histogram(&data, 5);
/// assert_eq!(result.len(), 5);
/// ```
pub fn histogram(data: &[f64], bin_count: usize) -> Vec<(f64, usize)> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let bin_width = (max - min) / bin_count as f64;

    let mut bins: HashMap<usize, usize> = HashMap::new();
    for &value in data {
        let bin = ((value - min) / bin_width).floor() as usize;
        *bins.entry(bin).or_insert(0) += 1;
    }

    let mut histogram = Vec::with_capacity(bin_count);
    for bin in 0..bin_count {
        let bin_center = min + bin as f64 * bin_width + bin_width / 2.0;
        let count = *bins.get(&bin).unwrap_or(&0);
        histogram.push((bin_center, count));
    }

    histogram
}
