use crate::ml::vector::Vector;

pub(crate) struct EloParams {
    k: i64, //default step size
    f: f64,
    initial_rating: i64,
}
impl EloParams {
    pub(crate) fn scale_factor(&self) -> f64 {
        self.f
    }

    pub(crate) fn initial_rating(&self) -> i64 {
        self.initial_rating
    }

    pub(crate) fn step(&self) -> i64 {
        self.k
    }

    pub(crate) fn new(v: &Vector) -> Self {
        match v.dim() {
            2 => {
                Self {
                    k: v.x() as i64,
                    f: v.y(),
                    initial_rating: 0, //assume 0 init for symmetry and some other lin alg properties
                }
            }
            3 => Self {
                k: v.x() as i64,
                f: v.y(),
                initial_rating: v.z() as i64,
            },
            _ => unimplemented!(""),
        }
    }
}

impl Default for EloParams {
    fn default() -> Self {
        Self {
            k: K,
            f: SCALE_FACTOR,
            initial_rating: INITIAL_RATING,
        }
    }
}

pub const INITIAL_RATING: i64 = 3000;
pub const SCALE_FACTOR: f64 = 400.0;
pub const K: i64 = 32;
