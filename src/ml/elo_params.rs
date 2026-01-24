use thiserror::Error;

use crate::ml::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EloParams {
    k: f64, //default step size
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

    pub(crate) fn step(&self) -> f64 {
        self.k
    }
}
#[derive(Error, Debug)]
pub(crate) enum EloParamError {
    #[error("❌ no parameters exist from vector with dimension {0}.")]
    UnknownDimension(usize),
    #[error("❌ parameter k must be greater than 0. {0} < 0")]
    NegativeK(f64),
    #[error("❌ parameter f must be greater than 0. {0} < 0")]
    NegativeF(f64),
    #[error(
        "❌ parameters k and f must have a ratio greater than 10k:f and less than 20k:f \n❌ (k, f) = ({0}, {1}) => {2}"
    )]
    InvalidRatio(f64, f64, f64),
}

impl TryFrom<&Vector> for EloParams {
    type Error = EloParamError;

    fn try_from(v: &Vector) -> Result<Self, Self::Error> {
        match v.dim() {
            2 => {
                if v.x() < 0. {
                    return Err(EloParamError::NegativeK(v.x()));
                }
                if v.y() < 0. {
                    return Err(EloParamError::NegativeF(v.y()));
                }
                if v.y() / v.x() < 10. || v.y() / v.x() > 20. {
                    return Err(EloParamError::InvalidRatio(v.x(), v.y(), v.y() / v.x()));
                }

                Ok(Self {
                    k: v.x(),
                    f: v.y(),
                    initial_rating: 0, //assume 0 init for symmetry and some other lin alg properties
                })
            }
            3 => Ok(Self {
                k: v.x(),
                f: v.y(),
                initial_rating: v.z() as i64,
            }),
            u => Err(EloParamError::UnknownDimension(u)),
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
pub const SCALE_FACTOR: f64 = 400.;
pub const K: f64 = 32.;
