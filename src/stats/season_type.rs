use crate::format::stat_path_formatter::StatPathFormatter;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::types::SeasonId;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum SeasonPeriod {
    PreSeason,
    RegularSeason,
    PostSeason,
    PlayIn, //todo
    NBACup,
    AllStarGame, //ignore
}

impl SeasonPeriod {
    pub fn get_offset(&self) -> i32 {
        match self {
            SeasonPeriod::PreSeason => 10_000,
            SeasonPeriod::RegularSeason => 20_000,
            SeasonPeriod::PostSeason => 40_000,
            SeasonPeriod::PlayIn => 60_000,
            SeasonPeriod::NBACup => 20_000,
            SeasonPeriod::AllStarGame => 30_000,
        }
    }
}

impl Display for SeasonPeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            SeasonPeriod::PreSeason => "Pre%20Season",
            SeasonPeriod::RegularSeason => "Regular%20Season",
            SeasonPeriod::PostSeason => "Playoffs",
            SeasonPeriod::PlayIn => "PlayIn",
            SeasonPeriod::NBACup => "IST", //in season tournament
            SeasonPeriod::AllStarGame => "All%20Star",
        };

        write!(f, "{}", s)
    }
}

impl StatPathFormatter for SeasonPeriod {
    fn path_specifier(&self) -> &'static str {
        match self {
            SeasonPeriod::PreSeason => "preseason",
            SeasonPeriod::RegularSeason => "regularseason",
            SeasonPeriod::PostSeason => "playoffs",
            SeasonPeriod::PlayIn => "playin",
            SeasonPeriod::NBACup => "nbacup",
            SeasonPeriod::AllStarGame => "asg",
        }
    }

    fn ext(&self) -> &'static str {
        //doesn't append anything to the filename, games are the same regardless of season (mostly)
        "" //todo maybe cook here
    }
}

pub fn minimum_spanning_era(year: i32) -> Vec<SeasonId> {
    let mut minimum_spanning_era = if year >= 2003 {
        vec![
            SeasonId::from((year, SeasonPeriod::PreSeason)),
            SeasonId::from((year, SeasonPeriod::RegularSeason)),
        ]
    } else {
        vec![
            SeasonId::from((year, SeasonPeriod::RegularSeason))
        ]
    };

    if year >= 2020 {
        minimum_spanning_era.push(SeasonId::from((year, SeasonPeriod::PlayIn)));
    }

    minimum_spanning_era.push(SeasonId::from((year, SeasonPeriod::PostSeason)));

    minimum_spanning_era
}
