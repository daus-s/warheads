use std::{fmt::Display, ops::Mul};

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
            let mut new_row = vec![0.0; rhs.cols];

            for col in 0..rhs.cols {
                new_row[col] += self.data[row][col] * rhs.data[col][row];
            }
            matrix.push(Vector::from(new_row));
        }

        Self {
            data: matrix,
            cols: rhs.cols,
            rows: self.rows,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rows == 0 || self.cols == 0 {
            return write!(f, "[]");
        }

        fn fmt_val(f: f64) -> String {
            if f.abs() >= 1e4 {
                if f.abs() >= 1e10 {
                    format!("{:.3e}", f)
                } else {
                    format!("{:.4e}", f)
                }
            } else if f != 0.0 && f.abs() < 1e-3 {
                if f.abs() < 1e-9 {
                    format!("{:.2e}", f)
                } else {
                    format!("{:.3e}", f)
                }
            } else {
                let f1 = (f * 1_000_000.0).round() / 1_000_000f64;

                let real = format!("{}", f1);
                let decimal = format!("{:.1}", f1);

                if decimal.len() > real.len() {
                    decimal
                } else {
                    real
                }
            }
        }

        let cells: Vec<Vec<String>> = self
            .data
            .iter()
            .map(|row| row.iter().map(|v| fmt_val(v)).collect())
            .collect();

        let col_width = cells.iter().flatten().map(|s| s.len()).max().unwrap_or(1);

        let row_strs: Vec<String> = cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| format!("{:>width$}", cell, width = col_width))
                    .collect::<Vec<_>>()
                    .join("  ")
            })
            .collect();

        if self.rows == 1 {
            return write!(f, "[ {} ]", row_strs[0]);
        }

        let mut out = String::new();
        for (i, row) in row_strs.iter().enumerate() {
            let (left, right) = match i {
                0 => ("┌", "┐"),
                r if r != self.rows - 1 => ("│", "│"),
                _ => ("└", "┘"),
            };
            out.push_str(&format!("{} {} {}\n", left, row, right));
        }

        write!(f, "{}", out.trim_end())
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

    #[test]
    fn test_multiply() {
        let a = Matrix::new(&[&[2.0, -2.0], &[5.0, 3.0]]);
        let b = Matrix::new(&[&[-1.0, 4.0], &[7.0, 6.0]]);
        let c = a * b;
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.index(0, 0), -16.0);
        assert_eq!(c.index(0, 1), 4.0);
        assert_eq!(c.index(1, 0), 16.0);
        assert_eq!(c.index(1, 1), 38.0);
    }

    #[test]
    fn test_display() {
        let matrix = Matrix::new(&[&[0.0001234, 2.3456789], &[3.456789, 4.56789]]);

        pretty_assertions::assert_eq!(
            format!("{}", matrix),
            "┌ 1.234e-4  2.345679 ┐\n\
             └ 3.456789   4.56789 ┘"
        );
    }

    #[test]
    fn test_display_with_height() {
        let matrix = Matrix::new(&[
            &[1.23456789, 2.3456789],
            &[676767.67, 0.0069420],
            &[3.456789, 4.56789],
        ]);

        pretty_assertions::assert_eq!(
            format!("{}", matrix),
            "┌ 1.234568  2.345679 ┐\n\
             │ 6.7677e5  0.006942 │\n\
             └ 3.456789   4.56789 ┘"
        );
    }

    #[test]
    fn test_display_short() {
        let matrix = Matrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);

        pretty_assertions::assert_eq!(
            format!("{}", matrix),
            "┌ 1.0  2.0 ┐\n\
             └ 3.0  4.0 ┘"
        );
    }
}
