use crate::ml::vector::Vector;

pub trait Cost {
    fn cost(&self, x: Vector) -> f64;
}
