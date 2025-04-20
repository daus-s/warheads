use crate::box_score::BoxScore;
use crate::player_box_score::PlayerBoxScore;
use crate::team_box_score::TeamBoxScore;

pub enum NBAStat {
    Player(PlayerBoxScore),
    Team(TeamBoxScore),
}

impl NBAStat {
    pub fn to_box_score(&self) -> &dyn BoxScore {
        match self {
            NBAStat::Player(p) => p,
            NBAStat::Team(t) => t,
        }
    }
}
