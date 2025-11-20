use crate::corrections::correction::Correction;

use crate::dapi::from_value::FromValue;
use crate::dapi::player_box_score::PlayerBoxScore;

use crate::format::box_score_formatter::format_team_box_score;

use crate::stats::box_score::BoxScore;
use crate::stats::record::Record;
use crate::stats::stat_column::StatColumn::*;
use crate::stats::teamcard::TeamCard;
use crate::stats::visiting::Visiting;

use crate::types::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn card(&self) -> TeamCard {
        TeamCard::new(
            self.team_id,
            self.team_name.clone(),
            self.team_abbreviation.clone(),
            Record { wins: 0, losses: 0 },
        )
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

    pub fn box_score(&self) -> &BoxScore {
        &self.box_score
    }

    pub fn roster_box_scores(&self) -> &Vec<PlayerBoxScore> {
        &self.roster
    }

    pub fn roster(&self) -> Vec<PlayerId> {
        let mut player_ids = Vec::new();

        for player in &self.roster {
            player_ids.push(player.player_id());
        }

        player_ids
    }

    pub fn correct_box_score(&mut self, correction: &mut Correction) {
        correction.correct_box_score(&mut self.box_score);
    }

    pub(crate) fn reorient(&mut self, correction: &mut Correction) {
        /*
         *
         * pub team_abbreviation: TeamAbbreviation,
         * pub team_name: TeamName,
         * home or away
         * pub visiting: Visiting,
         */
        let correction_file = correction.file_path();

        correction.corrections.retain(|key, value| match key {
            TEAM_ID => {
                if let Ok(team_id) = value.team_id() {
                    self.team_id = team_id;
                    false
                } else {
                    true
                }
            }
            TEAM_ABBREVIATION => {
                if let Ok(team_abbr) = value.team_abbreviation() {
                    self.team_abbreviation = team_abbr;
                    false
                } else {
                    true
                }
            }
            TEAM_NAME => {
                if let Ok(team_name) = value.team_name() {
                    self.team_name = team_name;
                    false
                } else {
                    true
                }
            }
            MATCHUP => {
                if let Ok(matchup) = value.matchup() {
                    //with new matchup, calculate visiting
                    if let Ok(visiting) = matchup.home_or_away(&self.team_abbreviation) {
                        self.visiting = visiting;
                        false
                    } else {
                        panic!("💀 matchup string provided by correction doesn't match team identity.\nfile:{}", correction_file.display())
                    }
                } else {
                    true
                }
            }
            _ => true,
        });
    }
}

impl std::fmt::Display for TeamBoxScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_team_box_score(f, self)
    }
}
