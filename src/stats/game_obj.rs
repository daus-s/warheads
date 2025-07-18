use serde::{Deserialize, Serialize};
use crate::dapi::team_box_score::TeamBoxScore;
use crate::stats::box_score::BoxScore;
use crate::stats::id::{Identifiable, Identity};
use crate::types::{GameDate, GameId, SeasonId};

#[derive(Serialize, Deserialize)]
pub struct GameObject {

    home: TeamBoxScore,
    away: TeamBoxScore,

}



impl GameObject {

    /// `create` consumes two `TeamBoxScore`'s and returns the combination of the two, typically used
    /// for serializing and deserializing from the custom data format specified in **store**.
    ///
    /// ## assertions
    /// - game_id: checks that both game ids are equal
    /// - season_id: checks that both are from the same season
    /// - matchup & abbr: checks that both teams are in the matchup and opposites
    pub fn create(team1: TeamBoxScore, team2: TeamBoxScore) -> Self {
        assert_eq!(team1.game_id(), team2.game_id());

        let x = 1;


        GameObject {
            home: team1,
            away: team2,
        }
    }

    pub fn season(&self) -> SeasonId {
        assert_eq!(self.home.season(), self.away.season());

        self.home.season()
    }
}

impl Identifiable for GameObject {
    fn identity(&self) -> Identity {
        self.home.identity()
    }
}