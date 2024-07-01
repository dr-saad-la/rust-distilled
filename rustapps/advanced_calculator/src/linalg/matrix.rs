/// Represents a matrix.
#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    /// Creates a new matrix.
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Matrix { rows, cols, data }
    }

    /// Adds two matrices.
    pub fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a + b)
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    /// Multiplies two matrices.
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut data = vec![0.0; self.rows * other.cols];
        for i in 0..self.rows {
            for j in 0..other.cols {
                data[i * other.cols + j] = (0..self.cols)
                    .map(|k| self.data[i * self.cols + k] * other.data[k * other.cols + j])
                    .sum();
            }
        }
        Matrix::new(self.rows, other.cols, data)
    }
}
