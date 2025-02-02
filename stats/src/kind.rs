use crate::player_box_score::PlayerBoxScore;
use crate::team_box_score::TeamBoxScore;

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
    Players(Vec<PlayerBoxScore>),
    Teams(Vec<TeamBoxScore>),
}