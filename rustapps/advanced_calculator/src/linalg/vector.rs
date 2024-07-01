/// Represents a vector.
#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    /// Creates a new vector.
    pub fn new(data: Vec<f64>) -> Self {
        Vector { data }
    }

    /// Adds two vectors.
    pub fn add(&self, other: &Vector) -> Vector {
        assert_eq!(self.data.len(), other.data.len());
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a + b)
            .collect();
        Vector::new(data)
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Vector) -> f64 {
        assert_eq!(self.data.len(), other.data.len());
        self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum()
    }

    /// Computes the cross product of two vectors (only for 3D vectors).
    pub fn cross(&self, other: &Vector) -> Vector {
        assert_eq!(self.data.len(), 3);
        assert_eq!(other.data.len(), 3);
        let data = vec![
            self.data[1] * other.data[2] - self.data[2] * other.data[1],
            self.data[2] * other.data[0] - self.data[0] * other.data[2],
            self.data[0] * other.data[1] - self.data[1] * other.data[0],
        ];
        Vector::new(data)
    }

    /// Computes the magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}
