use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use format::matchup::display_matchup;
use crate::types::MatchupString;

pub struct GameInfo {
    matchup: MatchupString,
    date: NaiveDate,
    name: Option<String>,
    /// team abbreviation used not for display but for
    tm: String, //todo: could eventually convert both of these to an enum
    /// String value of team
    team: String,
}

impl GameInfo {
    pub fn new(matchup: MatchupString, date: NaiveDate, name: Option<String>, tm: String, team: String) -> Self {
        GameInfo {
            matchup,
            date,
            name,
            tm,
            team,
        }
    }
}

impl Display for GameInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let matchup = display_matchup(self.matchup.str(), self.tm.to_string()).unwrap_or("-".to_string());
        // matching on name's existence is the same as checking Player vs. Team box score
        match self.name {
            Some(_) => write!(f, "{} - {}\n{}\n{}", matchup, self.date, self.team.to_string(), self.name.clone().unwrap().to_string()),
            None => write!(f, "{} - {}\n{}", matchup, self.date, self.team.to_string())
        }
    }
}