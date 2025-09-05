use crate::types::{GameId, PlayerId};

pub struct Elo {
    pub player_id: PlayerId,
    pub game_id: GameId,
    pub rating: i64,
}

pub const INITIAL_RATING: i64 = 3000;

pub const K: i64 = 32;

impl Elo {
    pub fn new(player_id: PlayerId, game_id: GameId, rating: i64) -> Self {
        Elo {
            player_id,
            game_id,
            rating,
        }
    }
}
