use std::fmt::{Display, Formatter};
use crate::stats::visiting::Visiting;
use crate::stats::visiting::Visiting::{Away, Home};
use crate::types::Matchup;

pub struct FieldStatus {
    matchup: Matchup,
    visiting: Visiting,
}

impl Display for FieldStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.visiting {
            Home => {
                write!(f, "{} @ {}", self.matchup.away, self.matchup.home)
            }
            Away => {
                write!(f, "{} vs. {}", self.matchup.home, self.matchup.away)
            }
        }
    }
}