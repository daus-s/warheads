use crate::format::box_score_formatter::format_player_box_score;
use crate::{dapi::from_value::FromValue, edit::edit::Edit};

use crate::stats::box_score::BoxScore;
use crate::stats::stat_column::StatColumn::*;

use crate::types::*;

use serde::{Deserialize, Serialize};

use wincode::{SchemaRead, SchemaWrite};

use std::fmt::Formatter;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, SchemaWrite, SchemaRead)]
pub struct PlayerBoxScore {
    player_id: PlayerId,
    player_name: PlayerName,

    box_score: BoxScore,
}

impl std::fmt::Display for PlayerBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_player_box_score(f, self)
    }
}

impl PlayerBoxScore {
    pub fn construct(player_id: PlayerId, player_name: PlayerName, box_score: BoxScore) -> Self {
        PlayerBoxScore {
            player_id,
            player_name,
            box_score,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn player_name(&self) -> &PlayerName {
        &self.player_name
    }

    pub fn box_score(&self) -> &BoxScore {
        &self.box_score
    }

    pub fn correct_box_score(&mut self, edit: &mut Edit) {
        edit.edit_box_score(&mut self.box_score);
    }

    pub(crate) fn correct_identifiers(&mut self, correction: &mut Edit) {
        correction.corrections.retain(|key, value| match key {
            PLAYER_ID => {
                if let Ok(player_id) = value.player_id() {
                    self.player_id = player_id;
                    false
                } else {
                    true
                }
            }
            PLAYER_NAME => {
                if let Ok(player_name) = value.player_name() {
                    self.player_name = player_name;
                    false
                } else {
                    true
                }
            }
            _ => true,
        });
    }
}
