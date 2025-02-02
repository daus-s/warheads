
use serde::{Deserialize, Serialize};

pub enum Visiting {
    Home,
    Away
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameResult {
    Win,
    Loss,
    Draw
}