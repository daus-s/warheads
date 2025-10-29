use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::stats::record::Record;
use crate::types::{TeamAbbreviation, TeamId, TeamName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamCard {
    team_id: TeamId,
    team_name: TeamName,
    team_abbr: TeamAbbreviation,
    record: Record,
}

impl TeamCard {
    pub fn new(
        team_id: TeamId,
        team_name: TeamName,
        team_abbr: TeamAbbreviation,
        record: Record,
    ) -> Self {
        Self {
            team_id,
            team_name,
            team_abbr,
            record,
        }
    }

    pub fn team_abbr(&self) -> &TeamAbbreviation {
        &self.team_abbr
    }

    pub fn team_id(&self) -> TeamId {
        self.team_id
    }

    pub fn add_win(&mut self) {
        self.record.wins += 1;
    }

    pub fn add_loss(&mut self) {
        self.record.losses += 1;
    }
}

impl Display for TeamCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) - {}",
            self.team_name, self.team_abbr, self.record
        )
    }
}

impl PartialEq for TeamCard {
    fn eq(&self, other: &Self) -> bool {
        self.team_id == other.team_id
            && self.team_name == other.team_name
            && self.team_abbr == other.team_abbr

        // am i sure?
        // omit checking the record as the record MAY? update with the season progression
    }
}
