use crate::stats::visiting::Visiting;
use crate::stats::visiting::Visiting::{Away, Home};
use crate::types::TeamAbbreviation;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// `MatchupString` is a String wrapper that is
/// enforced by its `fn parse() -> Self`
/// when loaded from String (or str)
#[derive(Clone, Debug)]
pub struct Matchup {
    pub home: TeamAbbreviation,
    pub away: TeamAbbreviation,
}

impl Matchup {
    // todo: implement the functions from player and team box score for matchup string (from s3)

    pub fn from_matchup(home: TeamAbbreviation, away: TeamAbbreviation) -> Self {
        Matchup { home, away }
    }

    pub fn home_or_away(&self, team: &TeamAbbreviation) -> Result<Visiting, String> {
        let home = &self.home;
        let away = &self.away;

        match team == home {
            true => Ok(Home),
            false => match *team == *away {
                true => Ok(Away),
                false => Err(format!("❌ team abbreviation {team} is not one of the two teams in this matchup: {home}, {away}"))
            }
        }
    }

    /// Returns the opposing team's abbreviation unmodified.
    /// It will be a 3 character capitalized string, this rule can
    /// and should be used for validation.
    ///
    /// # Arguments
    ///
    /// * `matchup`:matchup string provided by nba.com api
    /// * `abbr`: team abbreviation (3 characters)
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    /// use warheads::types::{Matchup, TeamAbbreviation};
    ///
    /// let matchup: Matchup = "MEM @ LAL".parse().unwrap();
    ///
    /// let opp = matchup.opponent(&"LAL".parse().unwrap());
    ///
    /// assert_eq!(Ok(TeamAbbreviation("MEM".to_string())), opp);
    /// ```
    pub fn opponent(&self, team: &TeamAbbreviation) -> Result<TeamAbbreviation, String> {
        if *team == self.home {
            Ok(self.away.clone())
        } else if *team == self.away {
            Ok(self.home.clone())
        } else {
            Err(format!(
                "❌ team {team} is neither of the contestants: {self}. "
            ))
        }
    }
}

impl Display for Matchup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.away, self.home)
    }
}

impl Serialize for Matchup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Matchup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let matchup = s.parse().map_err(de::Error::custom)?;

        Ok(matchup)
    }
}

impl FromStr for Matchup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [home, "vs." ,away] => Ok(Matchup {
                home: TeamAbbreviation(home.to_string()),
                away: TeamAbbreviation(away.to_string()),
            }),
            [away, "@" ,home] => Ok(Matchup {
                    home: TeamAbbreviation(home.to_string()),
                    away: TeamAbbreviation(away.to_string()),
                }),
            _ => Err(format!("❌ {} is not a valid matchup string. expected either:\n  • [tm1, @, tm2]\n  • [tm1, vs, tm2]", s )),
        }
    }
}

// this should be called after parsing a matchup
pub fn is_matchup_for_team(matchup_as_string: &str, team_abbreviation: &TeamAbbreviation) -> bool {
    match matchup_as_string.split_whitespace().collect::<Vec<&str>>().as_slice() {
        [home, "vs.", _away] => home == &team_abbreviation.0,
        [away, "@", _home] => away == &team_abbreviation.0,
        _ => panic!("❌ {} is not a valid matchup string. expected either:\n  • [tm1, @, tm2]\n  • [tm1, vs, tm2]", matchup_as_string),
    }
}
