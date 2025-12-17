use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// ok im gonna have to COOOOK
#[derive(Debug, Clone)]
pub struct Vector {
    vec: Vec<f64>,
    dim: usize,
}

impl Vector {
    pub fn origin(dim: usize) -> Self {
        Self {
            vec: vec![0.0f64; dim],
            dim,
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn x(&self) -> f64 {
        assert!(
            self.dim >= 1,
            "💀 vector is {}-dimensional and tried to access 1st dimension.",
            self.dim
        );
        self.vec[0]
    }

    pub fn y(&self) -> f64 {
        assert!(
            self.dim >= 2,
            "💀 vector is {}-dimensional and tried to access 2nd dimension.",
            self.dim
        );
        self.vec[1]
    }
}

pub fn midpoint(v1: &Vector, v2: &Vector) -> Vector {
    assert_eq!(v1.dim(), v2.dim(), "💀 vectors have different dimensions.");
    let mut mid = Vec::with_capacity(v1.dim());
    for (x, y) in v1.vec.iter().zip(v2.vec.iter()) {
        mid.push((x + y) / 2.0);
    }
    Vector::from(mid)
}

impl From<Vec<f64>> for Vector {
    fn from(vec: Vec<f64>) -> Self {
        Self {
            dim: vec.len(),
            vec,
        }
    }
}

impl From<&Vector> for Vector {
    fn from(vector: &Vector) -> Self {
        Self {
            vec: vector.vec.clone(),
            dim: vector.dim,
        }
    }
}

// TRAITS

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl Eq for Vector {}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, &val) in self.vec.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }
        write!(f, ")")
    }
}
// ADD + +=
// behaves like linear combination of vectors:
//                                              ax+by+cz = d
//                                            + ex+fy+gz = h
//                                            ______________
//                                            (a+e)x +
//                                            (b+f)y +
//                                            (c+g)z = d + h

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.dim, rhs.dim,
            "Vectors must have the same dimension. Cannot add a {}-d vector and a {}-d vector",
            rhs.dim, self.dim
        );

        let mut result = self.vec.clone();
        for (i, &val) in rhs.vec.iter().enumerate() {
            result[i] += val;
        }

        Vector {
            vec: result,
            dim: self.dim,
        }
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(
            self.dim, rhs.dim,
            "Vectors must have the same dimension. Cannot add {}-d vector to a {}-d vector",
            rhs.dim, self.dim
        );

        for (i, &val) in rhs.vec.iter().enumerate() {
            self.vec[i] += val;
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.dim, rhs.dim,
            "Vectors must have the same dimension. Cannot subtract a {}-d vector and a {}-d vector",
            rhs.dim, self.dim
        );

        let mut result = self.vec.clone();
        for (i, &val) in rhs.vec.iter().enumerate() {
            result[i] -= val;
        }

        Vector {
            vec: result,
            dim: self.dim,
        }
    }
}

impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, rhs: &Self) {
        assert_eq!(
            self.dim, rhs.dim,
            "Vectors must have the same dimension. Cannot subtract a {}-d vector from a {}-d vector",
            rhs.dim, self.dim
        );

        for (i, &val) in rhs.vec.iter().enumerate() {
            self.vec[i] -= val;
        }
    }
}

// Scalar Multiplication and Division

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Self::Output {
        let mut result = self.vec.clone();
        for val in result.iter_mut() {
            *val /= scalar;
        }

        Vector {
            vec: result,
            dim: self.dim,
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        for val in self.vec.iter_mut() {
            *val /= scalar;
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Self::Output {
        let mut result = self.vec.clone();

        for val in result.iter_mut() {
            *val *= scalar;
        }

        Vector {
            vec: result,
            dim: self.dim,
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.vec.iter_mut() {
            *val *= scalar;
        }
    }
}

//todo: implement vector multiplication: inner and outer product

#[cfg(test)]
mod test_vector {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::from(vec![4.0, 5.0, 6.0]);
        let v3 = &v1 + &v2;

        assert_eq!(v3, Vector::from(vec![5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::from(vec![4.0, 5.0, 6.0]);

        v1 += &v2;

        assert_eq!(v1, Vector::from(vec![5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_sub() {
        let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::from(vec![4.0, 5.0, 6.0]);
        let v3 = &v1 - &v2;

        assert_eq!(v3, Vector::from(vec![-3.0, -3.0, -3.0]));
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::from(vec![4.0, 5.0, 6.0]);

        v1 -= &v2;

        assert_eq!(v1, Vector::from(vec![-3.0, -3.0, -3.0]));
    }

    #[test]
    fn test_scalar_div() {
        let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = &v1 / 2.0;

        assert_eq!(v2, Vector::from(vec![0.5, 1.0, 1.5]));
    }

    #[test]
    fn test_scalar_mul() {
        let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        let v2 = &v1 * 2.0;

        assert_eq!(v2, Vector::from(vec![2.0, 4.0, 6.0]));
    }

    #[test]
    fn test_scalar_mul_assign() {
        let mut v1 = Vector::from(vec![1.0, 2.0, 3.0]);
        v1 *= 2.0;

        assert_eq!(v1, Vector::from(vec![2.0, 4.0, 6.0]));
    }
}
