use crate::dapi::team_box_score::TeamBoxScore;
use crate::stats::box_score::BoxScore;
use crate::stats::game_metadata::GameDisplay;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::visiting::Visiting;
use crate::types::{GameDate, GameId, SeasonId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameObject {
    season_id: SeasonId,
    game_date: GameDate,
    game_id: GameId,

    home: TeamBoxScore,
    away: TeamBoxScore,
}

impl GameObject {
    /// `create` consumes two `TeamBoxScore`'s and returns the combination of the two, typically
    /// used for serializing and deserializing from the custom data format specified in **store**.
    ///
    /// ## assertions
    /// - `game_id`: checks that both game ids are equal
    /// - `season_id`: checks that both are from the same season
    /// - `matchup` & `abbr`: checks that both teams are in the matchup and opposites
    pub fn create(
        season_id: SeasonId,
        game_date: GameDate,
        game_id: GameId,
        team1: TeamBoxScore,
        team2: TeamBoxScore,
    ) -> Self {
        match (team1.visiting(), team2.visiting()) {
            // team 1 is home, team 2 is away
            (Visiting::Home, Visiting::Away) => {
                GameObject {
                    season_id,
                    game_date,
                    game_id,
                    home: team1,
                    away: team2,
                }
            }
            // team 2 is home, team 1 is away
            (Visiting::Away, Visiting::Home) => {
                GameObject {
                    season_id,
                    game_date,
                    game_id,
                    home: team2,
                    away: team1,
                }
            }
            _ => panic!("💀 if this error is arising check that your input box scores have opposite field states to this function")
        }
    }

    pub fn season(&self) -> SeasonId {
        self.season_id
    }

    pub fn game_id(&self) -> GameId {
        self.game_id
    }

    /// ## Moment
    /// returns the moment (season and game) of the specific game.
    pub fn moment(&self) -> (SeasonId, GameId) {
        (self.season_id, self.game_id)
    }
}
