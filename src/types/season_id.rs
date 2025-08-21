use crate::format::season::season_fmt;
use crate::stats::season_period::SeasonPeriod;
use crate::stats::season_period::SeasonPeriod::{
    AllStarGame, NBACup, PlayIn, PostSeason, PreSeason, RegularSeason,
};
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::{Display, Formatter};

///
/// season_id: SeasonId
///
/// contains the information for both the season period (regular season, pre-season etc.) and
/// the calendar year of the start of the season
///
/// Ex:
///
/// (1946-47 season -> 1946)
///
/// based on what period of the season the game is we add the season period offset
///
/// *Regular season offset => 20000* for more info on this see the SeasonPeriod module
///```
/// use warheads::stats::season_period::SeasonPeriod::RegularSeason;
/// use warheads::types::SeasonId;
///
/// let year = 1946;
///
/// let season_id = 1946 + 20000;
///
/// let s_id = SeasonId::from(21946);
///
/// assert_eq!(SeasonId::from((1946, RegularSeason)), s_id)
///```
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct SeasonId {
    year: i32,
    period: SeasonPeriod,
}

impl SeasonId {
    pub fn period(&self) -> SeasonPeriod {
        self.period
    }

    pub fn destructure(&self) -> (i32, SeasonPeriod) {
        (self.year(), self.period())
    }

    pub fn year(&self) -> i32 {
        self.year
    }
}

/*
===================================================================================================
 From functions.

  from:
    • i32 -> SeasonId
    • (i32, SeasonPeriod) -> SeasonId
    • Value (serde_json_ -> Result<SeasonId, Infallible>
*/
impl From<i32> for SeasonId {
    fn from(id: i32) -> Self {
        let year = id % 10000;

        let per_int = id - year;

        let period = match per_int {
            10_000 => PreSeason,
            20_000 => RegularSeason, //although some games are NBA Cup games, this determination is not made by the SeasonId.
            30_000 => AllStarGame,
            40_000 => PostSeason,
            50_000 => PlayIn,
            _ => panic!("💀 no other season period offsets exist:  {per_int}"),
        };

        SeasonId { year, period }
    }
}

impl TryFrom<&Value> for SeasonId {
    type Error = ();

    fn try_from(value: &Value) -> Result<SeasonId, Self::Error> {
        let s = match value.as_str() {
            Some(s) => s,
            None => {
                eprintln!("⚠️ SeasonId is not a JSON String.");

                return Err(());
            }
        };

        match s.parse::<i32>() {
            Ok(x) => Ok(SeasonId::from(x)),
            Err(e) => {
                eprintln!("⚠️ failed to parse an integer from the SeasonId field: {e}");

                Err(())
            }
        }
    }
}

impl From<(i32, SeasonPeriod)> for SeasonId {
    fn from(value: (i32, SeasonPeriod)) -> Self {
        let (year, period) = value;

        SeasonId { year, period }
    }
}

/*
====================================================================================================
Display functions
*/

impl Display for SeasonId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let year_str = season_fmt(self.year());

        let period_str = serde_json::to_string(&self.period()).unwrap();

        write!(f, "{} {}", year_str, period_str)
    }
}

/*
====================================================================================================
serde_json functions

    Serialize
*/
impl Serialize for SeasonId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let sum = match self.period {
            PreSeason => 10_000,
            RegularSeason => 20_000,
            NBACup => 20_000,
            AllStarGame => 30_000,
            PostSeason => 40_000,
            PlayIn => 60_000,
        } + self.year;

        serializer.serialize_str(&format!("{}", sum))
    }
}

impl<'de> Deserialize<'de> for SeasonId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let i = s.parse::<i32>().map_err(de::Error::custom)?;

        Ok(SeasonId::from(i))
    }
}
