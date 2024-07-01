/// Represents a polynomial.
#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    /// Creates a new polynomial.
    pub fn new(coefficients: Vec<f64>) -> Self {
        Polynomial { coefficients }
    }

    /// Adds two polynomials.
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result = self.coefficients.clone();
        for (i, &coeff) in other.coefficients.iter().enumerate() {
            if i < result.len() {
                result[i] += coeff;
            } else {
                result.push(coeff);
            }
        }
        Polynomial::new(result)
    }

    /// Evaluates the polynomial at a given value of x.
    pub fn evaluate(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .rev()
            .fold(0.0, |acc, &coeff| acc * x + coeff)
    }
}
