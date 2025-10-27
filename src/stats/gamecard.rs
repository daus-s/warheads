use crate::stats::teamcard::TeamCard;
use crate::types::{GameDate, GameId};

pub struct GameCard {
    game_id: GameId,
    date: Option<GameDate>,
    home: Option<TeamCard>,
    away: Option<TeamCard>,
}

impl GameCard {
    pub fn game_id(game_id: GameId) -> Self {
        GameCard {
            game_id,
            date: None,
            home: None,
            away: None,
        }
    }

    pub fn set_date(&mut self, date: chrono::NaiveDate) {
        self.date = Some(GameDate(date));
    }

    pub fn add_home_team(&mut self, team: TeamCard) {
        self.home = Some(team);
    }

    pub fn add_away_team(&mut self, team: TeamCard) {
        self.away = Some(team);
    }
}
