use crate::stats::box_score::BoxScore;
use crate::stats::visiting::Visiting;
use crate::types::*;
use crate::{corrections::correction::Correction, dapi::player_box_score::PlayerBoxScore};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamBoxScore {
    // team identification
    pub team_id: TeamId,
    team_abbreviation: TeamAbbreviation,
    team_name: TeamName,

    //home or away
    visiting: Visiting,

    //roster
    roster: Vec<PlayerBoxScore>,

    // classic box score
    box_score: BoxScore,
}

impl TeamBoxScore {
    pub fn add_player_stats(&mut self, value: PlayerBoxScore) {
        self.roster.push(value);
    }

    pub fn team_abbr(&self) -> TeamAbbreviation {
        self.team_abbreviation.clone()
    }

    pub fn team_name(&self) -> TeamName {
        self.team_name.clone()
    }

    pub fn visiting(&self) -> Visiting {
        self.visiting
    }

    pub fn construct(
        team_abbreviation: TeamAbbreviation,
        team_name: TeamName,
        team_id: TeamId,
        visiting: Visiting,
        box_score: BoxScore,
    ) -> Self {
        TeamBoxScore {
            team_abbreviation,
            team_name,
            team_id,
            visiting,
            box_score,
            roster: Vec::new(),
        }
    }

    pub fn roster_mut(&mut self) -> &mut Vec<PlayerBoxScore> {
        &mut self.roster
    }

    pub fn correct_box_score(&mut self, correction: &mut Correction) {
        correction.correct_box_score(&mut self.box_score);
    }
}
