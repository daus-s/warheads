use crate::nba::{TeamBoxScore, PlayerBoxScore};

#[derive(Copy, Clone)]
pub enum NBAStatKind {
    Team,
    Player,
    LineUp //todo: develop this later
    // this is not a priority yet.
}
pub enum NBAStatType {
    Player(PlayerBoxScore),
    Team(TeamBoxScore),
}