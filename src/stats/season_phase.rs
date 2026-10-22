use crate::format::stat_path_formatter::StatPathFormatter;
use crate::format::url_format::UrlFormatter;

use crate::types::SeasonId;

use serde::{Deserialize, Serialize};

use wincode::{SchemaRead, SchemaWrite};

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Eq,
    Hash,
    Debug,
    SchemaRead,
    SchemaWrite,
)]
#[serde(rename_all = "PascalCase")]
pub enum Phase {
    PreSeason,
    RegularSeason,
    NBACup, //hopefully ignore
    PlayIn, //todo
    PostSeason,
    AllStarGame, //ignore
}

impl Phase {
    pub fn get_offset(&self) -> i32 {
        match self {
            Phase::PreSeason => 10_000,
            Phase::RegularSeason => 20_000,
            Phase::PostSeason => 40_000,
            Phase::PlayIn => 50_000,
            Phase::NBACup => 20_000,
            Phase::AllStarGame => 30_000,
        }
    }

    pub fn from_offset(offset: i32) -> Phase {
        match offset {
            10_000 => Phase::PreSeason,
            20_000 => Phase::RegularSeason,
            40_000 => Phase::PostSeason,
            50_000 => Phase::PlayIn,
            30_000 => Phase::AllStarGame,
            _ => panic!("💀 invalid season period offset: {}", offset),
        }
    }
}
impl Display for Phase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Phase::PreSeason => "PreSeason",
            Phase::RegularSeason => "RegularSeason",
            Phase::PostSeason => "Playoffs",
            Phase::PlayIn => "PlayIn",
            Phase::NBACup => "NBACup", //in season tournament
            Phase::AllStarGame => "AllStarGame",
        };

        write!(f, "{}", s)
    }
}

impl StatPathFormatter for Phase {
    fn path_specifier(&self) -> &'static str {
        match self {
            Phase::PreSeason => "preseason",
            Phase::RegularSeason => "regularseason",
            Phase::PostSeason => "playoffs",
            Phase::PlayIn => "playin",
            Phase::NBACup => "nbacup",
            Phase::AllStarGame => "asg",
        }
    }

    fn ext(&self) -> &'static str {
        //doesn't append anything to the filename, games are the same regardless of season (mostly)
        "" //todo maybe cook here
    }
}

pub fn season_phases(year: i32) -> Vec<SeasonId> {
    let mut season = vec![];

    if year >= 2003 {
        season.push(SeasonId::from((year, Phase::PreSeason)));
    }

    season.push(SeasonId::from((year, Phase::RegularSeason)));

    if year >= 2019 {
        season.push(SeasonId::from((year, Phase::PlayIn)));
    }

    season.push(SeasonId::from((year, Phase::PostSeason)));

    season
}

impl UrlFormatter for Phase {
    fn url(&self) -> String {
        let s: &str = match self {
            Phase::PreSeason => "Pre%20Season",
            Phase::RegularSeason => "Regular%20Season",
            Phase::PostSeason => "Playoffs",
            Phase::PlayIn => "PlayIn",
            Phase::NBACup => "IST", //in season tournament
            Phase::AllStarGame => "All%20Star",
        };

        s.to_string()
    }
}

impl FromStr for Phase {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PreSeason" => Ok(Phase::PreSeason),
            "RegularSeason" | "Regular Season" => Ok(Phase::RegularSeason),
            "Playoffs" | "PostSeason" | "Post Season" => Ok(Phase::PostSeason),
            "PlayIn" => Ok(Phase::PlayIn),
            "IST" | "NBA Cup" => Ok(Phase::NBACup),
            "AllStar" | "All Star Game" => Ok(Phase::AllStarGame),
            _ => Err(()),
        }
    }
}
