use std::collections::HashMap;

/// Converts an `f64` to `i64` by scaling. This approach is simple but may lose precision for very large or small values.
fn float_to_int_key(value: f64) -> i64 {
    (value * 1_000_000.0) as i64
}

/// Calculates the mode of a slice of numbers.
///
/// # Arguments
///
/// * `data` - A slice of f64 numbers.
///
/// # Returns
///
/// A vector containing the mode(s) of the numbers.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::mode::mode;
/// let data = [1.0, 2.0, 2.0, 3.0];
/// let result = mode(&data);
/// assert_eq!(result, vec![2.0]);
/// ```
pub fn mode(data: &[f64]) -> Vec<f64> {
    let mut occurrences: HashMap<i64, usize> = HashMap::new();
    for &value in data {
        let key = float_to_int_key(value);
        *occurrences.entry(key).or_insert(0) += 1;
    }

    let max_occurrence = occurrences.values().cloned().max().unwrap_or(0);
    let modes: Vec<f64> = occurrences
        .into_iter()
        .filter(|&(_, count)| count == max_occurrence)
        .map(|(key, _)| key as f64 / 1_000_000.0)
        .collect();

    modes
}
