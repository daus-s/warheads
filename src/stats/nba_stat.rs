use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;

pub enum NBABoxScore {
    Player(PlayerBoxScore),
    Team(TeamBoxScore),
}
