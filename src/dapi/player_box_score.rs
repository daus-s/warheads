use crate::corrections::correction::Correction;
use crate::stats::box_score::BoxScore;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerBoxScore {
    player_id: PlayerId,
    player_name: PlayerName,

    box_score: BoxScore,
}

impl std::fmt::Display for PlayerBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\nfantasy: {}",
            self.player_name,
            self.box_score,
            self.box_score.calculate_fantasy()
        )
    }
}

impl PlayerBoxScore {
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn construct(player_id: PlayerId, player_name: PlayerName, box_score: BoxScore) -> Self {
        PlayerBoxScore {
            player_id,
            player_name,
            box_score,
        }
    }

    pub fn correct_box_score(&mut self, correction: &mut Correction) {
        correction.correct_box_score(&mut self.box_score);
    }
}
