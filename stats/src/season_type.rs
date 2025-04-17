use std::fmt::{Display, Formatter};

pub enum SeasonType {
    PreSeason,
    RegularSeason,
    PostSeason,
    PlayIn, //todo
    NBACup,
    AllStarGame //ignore
}

impl Display for SeasonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            SeasonType::PreSeason => "Pre%20Season",
            SeasonType::RegularSeason => "Regular%20Season",
            SeasonType::PostSeason => "Playoffs",
            SeasonType::PlayIn => "PlayIn",
            SeasonType::NBACup => "IST", //in season tournament
            SeasonType::AllStarGame => "All%20Star",
        };

        write!(f, "{}", s)
    }
}