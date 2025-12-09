use std::ops::{Add, AddAssign, Div, DivAssign, Sub, SubAssign};
use std::sync::{Arc, Mutex};

// ok im gonna have to COOOOK
#[derive(Debug, Clone)]
pub struct Vector(Arc<Mutex<Vec<f64>>>);

impl Vector {
    pub fn origin(dim: usize) -> Self {
        Self(Arc::new(Mutex::new(vec![0.0f64; dim])))
    }
}

impl From<Vec<f64>> for Vector {
    fn from(vec: Vec<f64>) -> Self {
        Vector(Arc::new(Mutex::new(vec)))
    }
}

impl From<Vector> for Arc<Mutex<Vec<f64>>> {
    /// Clones a Vector to a Vec<f64>
    fn from(vector: Vector) -> Self {
        vector.0
    }
}

// TRAITS

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.0.lock().unwrap();
        let other_lock = other.0.lock().unwrap();

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
        let self_lock = self.0.lock().unwrap();
        let rhs_lock = rhs.0.lock().unwrap();

        let mut result = self_lock.clone();
        for (i, &val) in rhs_lock.iter().enumerate() {
            result[i] += val;
        }

        Vector(Arc::new(Mutex::new(result)))
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Self) {
        let mut self_lock = self.0.lock().unwrap();
        let rhs_lock = rhs.0.lock().unwrap();

        for (i, &val) in rhs_lock.iter().enumerate() {
            self_lock[i] += val;
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_lock = self.0.lock().unwrap();
        let rhs_lock = rhs.0.lock().unwrap();

        let mut result = self_lock.clone();
        for (i, &val) in rhs_lock.iter().enumerate() {
            result[i] -= val;
        }

        Vector(Arc::new(Mutex::new(result)))
    }
}

impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, rhs: &Self) {
        let mut self_lock = self.0.lock().unwrap();
        let rhs_lock = rhs.0.lock().unwrap();

        for (i, &val) in rhs_lock.iter().enumerate() {
            self_lock[i] -= val;
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        let self_lock = self.0.lock().unwrap();

        let mut result = self_lock.clone();
        for val in result.iter_mut() {
            *val /= rhs;
        }

        Vector(Arc::new(Mutex::new(result)))
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
}
