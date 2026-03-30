use std::ops::Mul;

use super::vector::Vector;

#[derive(Default, Clone, Debug)]
pub struct Matrix {
    data: Vec<Vector>, //each vector is a row
    cols: usize,
    rows: usize,
}

impl Matrix {
    pub fn new(vecs: &[&[f64]]) -> Self {
        if vecs.is_empty() {
            return Self::default();
        }

        let rows = vecs.len();
        let cols = vecs[0].len();

        let data: Vec<Vector> = vecs
            .iter()
            .map(|v| {
                assert!(
                    v.len() == cols,
                    "💀 inconsistent column lengths in matrix construction."
                );
                Vector::from(v.to_vec())
            })
            .collect();

        Self { data, cols, rows }
    }

    pub fn index(&self, row: usize, col: usize) -> f64 {
        assert!(row < self.rows, "💀 row index out of bounds.");
        assert!(col < self.cols, "💀 col index out of bounds.");

        self.data[row][col]
    }

    #[allow(non_snake_case)]
    pub fn T(&self) -> Self {
        let mut transposed = Vec::with_capacity(self.cols);
        for col in 0..self.cols {
            let mut row = Vec::with_capacity(self.rows);
            for row_vec in &self.data {
                row.push(row_vec[col]);
            }
            transposed.push(Vector::from(row));
        }
        Self {
            data: transposed,
            cols: self.data.len(),
            rows: self.cols,
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows, "💀 matrix dimensions do not match.");

        //multiply m x n by n x p => m x p matrix
        let mut matrix = Vec::with_capacity(self.rows);

        for row in 0..self.rows {
            let mut new_row = Vec::with_capacity(rhs.cols);

            for col in 0..rhs.cols {
                new_row[col] = self.data[row];
            }
            matrix.push(Vector::from(new_row));
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let matrix = Matrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.index(0, 0), 1.0);
        assert_eq!(matrix.index(0, 1), 2.0);
        assert_eq!(matrix.index(1, 0), 3.0);
        assert_eq!(matrix.index(1, 1), 4.0);
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::new(&[&[1.0, 2.0]]);
        let transposed = matrix.T();
        assert_eq!(transposed.rows, 2);
        assert_eq!(transposed.cols, 1);
        assert_eq!(transposed.index(0, 0), 1.0);
        assert_eq!(transposed.index(1, 0), 2.0);
    }
}
