use crate::stats::visiting::Visiting;
use crate::types::{GameId, PlayerId};

type Something = (GameId, PlayerId, Visiting);

pub struct EloTracker {
    game_player_ratings: Vec<(Something, i32)>,
}

impl EloTracker {
    pub fn process_elo(&self) {
        todo!("assign elo values to players on a game by game basis")
    }
}