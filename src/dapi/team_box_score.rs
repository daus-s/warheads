use crate::stats::box_score::BoxScore;
use crate::stats::visiting::Visiting;
use crate::types::*;
use crate::{dapi::player_box_score::PlayerBoxScore, ml::elo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    pub fn box_score(&self) -> &BoxScore {
        &self.box_score
    }

    pub fn roster(&self) -> &Vec<PlayerBoxScore> {
        &self.roster
    }

    fn get_team_rating(&self, ratings: &mut HashMap<PlayerId, i64>) -> i64 {
        let mut rating = 0;
        for player in self.roster() {
            if let Some(i) = ratings.get(&player.player_id()) {
                rating += *i;
            } else {
                ratings.insert(player.player_id(), elo::INITIAL_RATING);
                rating += elo::INITIAL_RATING;
            }
        }
        rating
    }

    pub fn get_normalized_team_rating(&self, ratings: &mut HashMap<PlayerId, i64>) -> f64 {
        let rating = self.get_team_rating(ratings);
        rating as f64 / self.roster().len() as f64
    }
}
