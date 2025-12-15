use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::{Arc, Mutex};

// ok im gonna have to COOOOK
#[derive(Debug, Clone)]
pub struct Vector {
    vec: Arc<Mutex<Vec<f64>>>,
    dim: usize,
}

impl Vector {
    pub fn origin(dim: usize) -> Self {
        Self {
            vec: Arc::new(Mutex::new(vec![0.0f64; dim])),
            dim,
        }
    }
}

impl From<Vec<f64>> for Vector {
    fn from(vec: Vec<f64>) -> Self {
        Self {
            dim: vec.len(),
            vec: Arc::new(Mutex::new(vec)),
        }
    }
}

impl From<Vector> for Arc<Mutex<Vec<f64>>> {
    /// Clones a Vector to a Vec<f64>
    fn from(vector: Vector) -> Self {
        vector.vec
    }
}

// TRAITS

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.vec.lock().unwrap();
        let other_lock = other.vec.lock().unwrap();

        *self_lock == *other_lock
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

        let self_lock = self.vec.lock().unwrap();
        let rhs_lock = rhs.vec.lock().unwrap();

        let mut result = self_lock.clone();
        for (i, &val) in rhs_lock.iter().enumerate() {
            result[i] += val;
        }

        Vector {
            vec: Arc::new(Mutex::new(result)),
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

        let mut self_lock = self.vec.lock().unwrap();
        let rhs_lock = rhs.vec.lock().unwrap();

        for (i, &val) in rhs_lock.iter().enumerate() {
            self_lock[i] += val;
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
        let self_lock = self.vec.lock().unwrap();
        let rhs_lock = rhs.vec.lock().unwrap();

        let mut result = self_lock.clone();
        for (i, &val) in rhs_lock.iter().enumerate() {
            result[i] -= val;
        }

        Vector {
            vec: Arc::new(Mutex::new(result)),
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
        let mut self_lock = self.vec.lock().unwrap();
        let rhs_lock = rhs.vec.lock().unwrap();

        for (i, &val) in rhs_lock.iter().enumerate() {
            self_lock[i] -= val;
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Self::Output {
        let self_lock = self.vec.lock().unwrap();

        let mut result = self_lock.clone();
        for val in result.iter_mut() {
            *val /= scalar;
        }

        Vector {
            vec: Arc::new(Mutex::new(result)),
            dim: self.dim,
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        let mut self_lock = self.vec.lock().unwrap();

        for val in self_lock.iter_mut() {
            *val /= scalar;
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Self::Output {
        let self_lock = self.vec.lock().unwrap();

        let mut result = self_lock.clone();
        for val in result.iter_mut() {
            *val *= scalar;
        }

        Vector {
            vec: Arc::new(Mutex::new(result)),
            dim: self.dim,
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        let mut self_lock = self.vec.lock().unwrap();

        for val in self_lock.iter_mut() {
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
