use std::fmt::Display;

use derive_builder::Builder;

use crate::stats::teamcard::TeamCard;
use crate::types::{GameDate, GameId};

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct GameCard {
    game_id: GameId,
    date: GameDate,
    home: TeamCard,
    away: TeamCard,
}

impl GameCard {
    pub fn new(game_id: GameId, date: GameDate, home: TeamCard, away: TeamCard) -> Self {
        GameCard {
            game_id,
            date,
            home,
            away,
        }
    }
}

impl Display for GameCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game ID: {}, Date: {}\nhome: {}\naway: {}",
            self.game_id, self.date, self.home, self.away
        )
    }
}

impl GameCard {
    pub fn game_id(&self) -> GameId {
        self.game_id
    }
}
