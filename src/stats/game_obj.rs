use crate::corrections::correction_builder::CorrectionBuilder;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::stats::game_display::GameDisplay;
use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::stat_column::StatColumn::{GAME_DATE, MATCHUP};
use crate::stats::visiting::Visiting;
use crate::types::matchup::home_and_away;
use crate::types::{GameDate, GameId, Matchup, SeasonId};
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameObject {
    season_id: SeasonId,
    game_date: GameDate,
    game_id: GameId,

    home: TeamBoxScore,
    away: TeamBoxScore,
}

impl GameObject {
    pub fn try_create(
        id1: Identity,
        game1: TeamBoxScore,
        id2: Identity,
        game2: TeamBoxScore,
    ) -> Result<GameObject, Vec<CorrectionBuilder>> {
        let mut correction1 = CorrectionBuilder::new(
            id1.game_id,
            id1.season_id,
            None,
            id1.team_id,
            id1.team_abbr(),
            NBAStatKind::Team,
            id1.game_date,
        );
        let mut correction2 = CorrectionBuilder::new(
            id2.game_id,
            id2.season_id,
            None,
            id2.team_id,
            id2.team_abbr(),
            NBAStatKind::Team,
            id2.game_date,
        );

        if id1.game_id != id2.game_id || id1.season_id != id2.season_id {
            panic!("💀 mismatched GameId's or SeasonId's in try_create.")
        }

        if id1.game_date != id2.game_date {
            correction1.add_missing_field(GAME_DATE, Null);
            correction2.add_missing_field(GAME_DATE, Null);
        }

        let mut matchup: Matchup = Default::default();

        // by this point we must know that both box scores are well-made and all other fields match.
        let game = home_and_away(game1.clone(), game2.clone())
            .map(|(home, away)| {
                matchup = Matchup::from_matchup(home.team_abbr(), away.team_abbr());

                (home, away)
            })
            .map_err(|(a, b)| {
                correction1.add_missing_field(MATCHUP, Null);
                correction2.add_missing_field(MATCHUP, Null);

                //despite this not being correct, we have ensured that the matchup need be corrected before using this data
                matchup = Matchup::from_matchup(a.team_abbr(), b.team_abbr());
            });

        correction1.update_display(GameDisplay::new(
            matchup.clone(),
            id1.game_date,
            None,
            game1.team_name(),
        ));
        correction2.update_display(GameDisplay::new(
            matchup.clone(),
            id2.game_date,
            None,
            game2.team_name(),
        ));

        if correction1.has_corrections() || correction2.has_corrections() {
            Err(vec![correction1, correction2])
        } else {
            let (home, away) = game
                .expect("💀 if home_and_away errs it must be covered in the correcting branch.");

            let game = GameObject {
                season_id: id1.season_id,
                game_id: id1.game_id,
                game_date: id1.game_date,
                home,
                away,
            };

            Ok(game)
        }
    }
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
