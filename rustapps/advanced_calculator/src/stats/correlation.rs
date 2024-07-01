/// Calculates the Pearson correlation coefficient between two slices of numbers.
///
/// # Arguments
///
/// * `x` - The first slice of f64 numbers.
/// * `y` - The second slice of f64 numbers.
///
/// # Returns
///
/// The Pearson correlation coefficient between `x` and `y`.
///
/// # Examples
///
/// ```
/// use advanced_calculator::stats::correlation::corr;
/// let x = [1.0, 2.0, 3.0];
/// let y = [1.0, 2.0, 3.0];
/// let result = corr(&x, &y);
/// assert_eq!(result, 1.0);
/// ```
pub fn corr(x: &[f64], y: &[f64]) -> f64 {
    assert_eq!(x.len(), y.len());

    let mean_x = super::mean::mean(x);
    let mean_y = super::mean::mean(y);

    let numerator: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
        .sum();

    let denominator_x: f64 = x.iter().map(|&x| (x - mean_x).powi(2)).sum();
    let denominator_y: f64 = y.iter().map(|&y| (y - mean_y).powi(2)).sum();

    numerator / (denominator_x.sqrt() * denominator_y.sqrt())
}
