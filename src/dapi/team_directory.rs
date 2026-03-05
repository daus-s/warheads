use crate::types::{TeamAbbreviation, TeamId, TeamName};

type Assoc = (TeamId, TeamAbbreviation, TeamName);

pub struct TeamDirectory {
    teams: Vec<Assoc>,
}

const DIRECTORY: &[Assoc] = &[];

impl TeamDirectory {
    pub fn new() -> Self {
        Self {
            teams: DIRECTORY.to_vec(),
        }
    }
}
