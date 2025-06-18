use crate::stats::season_type::SeasonPeriod;
use crate::stats::season_type::SeasonPeriod::{AllStarGame, PostSeason, PreSeason, RegularSeason, PlayIn, NBACup};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use crate::stats::player_box_score::PlayerBoxScoreBuilder;
use crate::stats::stat_column::StatColumn;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct SeasonId(pub i32);
impl SeasonId {
    pub fn period(&self) -> SeasonPeriod {
        match self.0 {
            x if x > 10000 && x < 19999 => PreSeason,
            x if x > 20000 && x < 29999 => RegularSeason, //also, in season tournament
            x if x > 30000 && x < 39999 => AllStarGame,
            x if x > 40000 && x < 49999 => PostSeason,
            x if x > 50000 && x < 59999 => PlayIn,
            _ => unreachable!("{}", format!("could not match season id {self} to a SeasonPeriod")),
        }
    }

    pub fn destructure(&self) -> (i32, SeasonPeriod) {
        (self.year(), self.period())
    }

    pub fn year(&self) -> i32 {
        self.0 % 10000
    }
}

impl From<i32> for SeasonId {
    fn from(value: i32) -> Self {
        SeasonId(value)
    }
}

impl From<(i32, SeasonPeriod)> for SeasonId {
    fn from(value: (i32, SeasonPeriod)) -> Self {
        let (year, period) = value;

        match period {
            PreSeason => SeasonId(10000 + year),
            RegularSeason => SeasonId(20000 + year),
            PostSeason => SeasonId(40000 + year),
            PlayIn => SeasonId(50000 + year),
            NBACup => SeasonId(20000 + year),
            AllStarGame => SeasonId(30000 + year),
        }
    }
}

impl Display for SeasonId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for SeasonId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let int_string = format!("{}", self);

        serializer.serialize_str(&int_string)
    }
}
