use crate::stats::visiting::Visiting;
use crate::stats::visiting::Visiting::{Away, Home};
use crate::types::{SeasonId, TeamAbbreviation};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// `MatchupString` is a String wrapper that is
/// enforced by its `fn parse() -> Self`
/// when loaded from String (or str)
#[derive(Clone, Debug)]
pub struct MatchupString {
    home: TeamAbbreviation,
    away: TeamAbbreviation,
    visiting: Visiting,
}

impl MatchupString {
    // todo: implement the functions from player and team box score for matchup string (from s3)

    pub fn from_matchup(
        home: TeamAbbreviation,
        away: TeamAbbreviation,
        visiting: Visiting,
    ) -> Self {
        MatchupString {
            home,
            away,
            visiting,
        }
    }

    pub fn home_or_away(&self) -> Visiting {
        self.visiting
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
    /// use warheads::types::{MatchupString, TeamAbbreviation};
    ///
    /// let matchup: MatchupString = "MEM @ LAL".parse().unwrap();
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

impl Display for MatchupString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.visiting {
            Home => write!(f, "{} vs. {}", self.home, self.away),
            Away => write!(f, "{} @ {}", self.away, self.home),
        }
    }
}

impl Serialize for MatchupString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MatchupString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let matchup = s.parse().map_err(de::Error::custom)?;

        Ok(matchup)
    }
}

impl FromStr for MatchupString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [home, "vs." ,away] => Ok(MatchupString {
                home: TeamAbbreviation(home.to_string()),
                away: TeamAbbreviation(away.to_string()),
                visiting: Home,
            }),
            [away, "@" ,home] => Ok(MatchupString {
                    home: TeamAbbreviation(home.to_string()),
                    away: TeamAbbreviation(away.to_string()),
                    visiting: Away,
                }),
            _ => Err(format!("❌ {} is not a valid matchup string. expected either:\n  • [tm1, @, tm2]\n  • [tm1, vs, tm2]", s )),
        }
    }
}
