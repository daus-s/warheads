use format::stat_path_formatter::StatPathFormatter;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SeasonPeriod {
    PreSeason,
    RegularSeason,
    PostSeason,
    PlayIn, //todo
    NBACup,
    AllStarGame, //ignore
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

pub fn minimum_spanning_era(year: i32) -> Vec<SeasonPeriod> {
    let mut minimum_spanning_era = vec![SeasonPeriod::PreSeason, SeasonPeriod::RegularSeason];

    if year >= 2020 {
        minimum_spanning_era.push(SeasonPeriod::PlayIn);
    }

    minimum_spanning_era.push(SeasonPeriod::PostSeason);

    minimum_spanning_era
}
