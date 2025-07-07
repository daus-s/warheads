use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub struct Elo(pub i32); // should this be signed or unsigned?

impl Elo {
    /// this function defines the original value of any players ELO for the procession.
    pub fn init() -> Self {
        Elo(3000)
    }
}

impl Display for Elo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
