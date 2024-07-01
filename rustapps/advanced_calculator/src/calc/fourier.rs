/// Computes the Discrete Fourier Transform (DFT) of a vector of complex numbers.
pub fn dft(input: &[num_complex::Complex<f64>]) -> Vec<num_complex::Complex<f64>> {
    let n = input.len();
    (0..n)
        .map(|k| {
            input
                .iter()
                .enumerate()
                .map(|(n, x)| {
                    let theta = -2.0 * std::f64::consts::PI * k as f64 * n as f64 / n as f64;
                    x * num_complex::Complex::new(theta.cos(), theta.sin())
                })
                .sum()
        })
        .collect()
}
