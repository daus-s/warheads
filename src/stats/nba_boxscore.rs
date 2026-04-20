use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;

#[derive(Debug, Clone)]
pub enum NBABoxScore {
    Player(PlayerBoxScore),
    Team(TeamBoxScore),
}

impl NBABoxScore {
    pub fn is_player_boxscore(&self) -> bool {
        matches!(self, NBABoxScore::Player(_))
    }

    pub fn is_team_boxscore(&self) -> bool {
        matches!(self, NBABoxScore::Team(_))
    }

    pub fn into_player_boxscore(self) -> Option<PlayerBoxScore> {
        match self {
            NBABoxScore::Player(box_score) => Some(box_score),
            NBABoxScore::Team(_) => None,
        }
    }

    pub fn into_team_boxscore(self) -> Option<TeamBoxScore> {
        match self {
            NBABoxScore::Team(box_score) => Some(box_score),
            NBABoxScore::Player(_) => None,
        }
    }
}
