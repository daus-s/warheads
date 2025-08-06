use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::stats::box_score::BoxScore;

pub enum NBAStat {
    Player(PlayerBoxScore),
    Team(TeamBoxScore),
}
