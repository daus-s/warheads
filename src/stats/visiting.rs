use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Visiting {
    Home,
    Away,
}

impl Display for Visiting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visiting::Home => write!(f, "Home"),
            Visiting::Away => write!(f, "Away"),
        }
    }
}
