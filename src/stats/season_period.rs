use crate::format::stat_path_formatter::StatPathFormatter;
use crate::format::url_format::UrlFormatter;

use crate::types::SeasonId;

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Hash, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SeasonPeriod {
    PreSeason,
    RegularSeason,
    NBACup, //hopefully ignore
    PlayIn, //todo
    PostSeason,
    AllStarGame, //ignore
}

impl SeasonPeriod {
    pub fn get_offset(&self) -> i32 {
        match self {
            SeasonPeriod::PreSeason => 10_000,
            SeasonPeriod::RegularSeason => 20_000,
            SeasonPeriod::PostSeason => 40_000,
            SeasonPeriod::PlayIn => 50_000,
            SeasonPeriod::NBACup => 20_000,
            SeasonPeriod::AllStarGame => 30_000,
        }
    }
}
impl Display for SeasonPeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            SeasonPeriod::PreSeason => "PreSeason",
            SeasonPeriod::RegularSeason => "RegularSeason",
            SeasonPeriod::PostSeason => "Playoffs",
            SeasonPeriod::PlayIn => "PlayIn",
            SeasonPeriod::NBACup => "NBACup", //in season tournament
            SeasonPeriod::AllStarGame => "AllStarGame",
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
    let mut season = vec![];

    if year >= 2003 {
        season.push(SeasonId::from((year, SeasonPeriod::PreSeason)));
    }

    season.push(SeasonId::from((year, SeasonPeriod::RegularSeason)));

    if year >= 2020 {
        season.push(SeasonId::from((year, SeasonPeriod::PlayIn)));
    }

    season.push(SeasonId::from((year, SeasonPeriod::PostSeason)));

    season
}

impl UrlFormatter for SeasonPeriod {
    fn url(&self) -> String {
        let s: &str = match self {
            SeasonPeriod::PreSeason => "Pre%20Season",
            SeasonPeriod::RegularSeason => "Regular%20Season",
            SeasonPeriod::PostSeason => "Playoffs",
            SeasonPeriod::PlayIn => "PlayIn",
            SeasonPeriod::NBACup => "IST", //in season tournament
            SeasonPeriod::AllStarGame => "All%20Star",
        };

        s.to_string()
    }
}

impl FromStr for SeasonPeriod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PreSeason" => Ok(SeasonPeriod::PreSeason),
            "RegularSeason" | "Regular Season" => Ok(SeasonPeriod::RegularSeason),
            "Playoffs" | "PostSeason" | "Post Season" => Ok(SeasonPeriod::PostSeason),
            "PlayIn" => Ok(SeasonPeriod::PlayIn),
            "IST" | "NBA Cup" => Ok(SeasonPeriod::NBACup),
            "AllStar" | "All Star Game" => Ok(SeasonPeriod::AllStarGame),
            _ => Err(()),
        }
    }
}
