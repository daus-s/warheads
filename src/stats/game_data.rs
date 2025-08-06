use crate::types::{GameId, SeasonId, TeamId};

#[derive(Eq, PartialEq)]
pub struct GameData {
    season_id: SeasonId,
    game_id: GameId,
    team_id: TeamId,
}

impl GameData {
    pub fn new(season_id: SeasonId, game_id: GameId, team_id: TeamId) -> Self {
        GameData {
            season_id,
            game_id,
            team_id,
        }
    }
}
