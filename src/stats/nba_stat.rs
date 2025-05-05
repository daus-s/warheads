use crate::stats::box_score::BoxScore;
use crate::stats::player_box_score::PlayerBoxScore;
use crate::stats::team_box_score::TeamBoxScore;

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
