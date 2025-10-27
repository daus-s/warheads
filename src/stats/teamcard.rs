use std::fmt::Display;

use crate::stats::record::Record;
use crate::types::{TeamAbbreviation, TeamId, TeamName};

#[derive(Debug)]
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

    //todo change this to return what would be needed to find this game (maybe put this on gamcard?)
    fn source_search_query(&self) -> String {
        format!("team_id:{}", self.team_id)
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
