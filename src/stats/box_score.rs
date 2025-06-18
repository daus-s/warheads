use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;
use crate::stats::visiting::Visiting;
use crate::types::{GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};

pub trait BoxScore {
    fn season(&self) -> SeasonId;

    fn game_id(&self) -> &GameId;

    fn player_id(&self) -> Option<PlayerId>;

    fn team_id(&self) -> TeamId;

    fn team_abbr(&self) -> &TeamAbbreviation;

    fn home_or_away(&self) -> Visiting;

    fn set(&mut self, col: &StatColumn, val: &StatValue);
}
