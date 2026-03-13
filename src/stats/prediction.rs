use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::format;

use crate::stats::gamecard::GameCard;

use crate::stats::visiting::Visiting;
use crate::types::{GameDate, Matchup};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    #[serde(flatten)]
    card: GameCard,

    /// ### Probability
    /// probability a `f64`, represents the probability that the home team wins as per scoring
    /// home win = 1
    /// away win = 0
    probability: f64,
}

impl Prediction {
    pub fn new(card: &GameCard, probability: f64) -> Self {
        Prediction {
            card: card.clone(),
            probability,
        }
    }

    pub fn from(card: GameCard, probability: f64) -> Self {
        Prediction { card, probability }
    }

    ////////////////////////////////////////////////////////////////////////////////////////
    //// accessors /////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////
    pub fn date(&self) -> GameDate {
        self.card.date()
    }
}

impl Display for Prediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let matchup_string = format!(
            "{} - ({})",
            Matchup::from_matchup(
                self.card.home().team_abbr().clone(),
                self.card.away().team_abbr().clone()
            ),
            self.card.date()
        );

        let away_prob = (1.0 - self.probability) * 100.0;
        let home_prob = self.probability * 100.0;

        let mut away_count = (27f64 * away_prob / 100.0).round() as usize;
        let mut home_count = (27f64 * home_prob / 100.0).round() as usize;

        let mut state = Visiting::Away;

        while away_count + home_count > 27 {
            match state {
                Visiting::Away => {
                    away_count -= 1;
                    state = Visiting::Home;
                }
                Visiting::Home => {
                    home_count -= 1;
                    state = Visiting::Away;
                }
            }
        }

        state = Visiting::Home;
        while away_count + home_count < 27 {
            match state {
                Visiting::Away => {
                    away_count += 1;
                    state = Visiting::Home;
                }
                Visiting::Home => {
                    home_count += 1;
                    state = Visiting::Away;
                }
            }
        }

        let (home_p_bar, away_p_bar) = if self.probability > 0.5 {
            //home team gets favored green
            //
            (
                format!(
                    "{}",
                    "🟩".repeat(if home_count > 5 { home_count - 5 } else { 0 })
                ),
                format!(
                    "{}",
                    "🟥".repeat(if away_count > 5 { away_count - 5 } else { 0 })
                ),
            )
        } else {
            (
                format!(
                    "{}",
                    "🟥".repeat(if home_count > 5 { home_count - 5 } else { 0 })
                ),
                format!(
                    "{}",
                    "🟩".repeat(if away_count > 5 { away_count - 5 } else { 0 })
                ),
            )
        };

        let (home_start_seq, home_end_seq, away_start_seq, away_end_seq) = if self.probability > 0.5
        {
            //home team gets favored green
            //
            (
                format!("\x1b[42m"),
                format!("\x1b[0m"),
                format!("\x1b[41m"),
                format!("\x1b[0m"),
            )
        } else {
            (
                format!("\x1b[41m"),
                format!("\x1b[0m"),
                format!("\x1b[42m"),
                format!("\x1b[0m"),
            )
        };

        writeln!(
            f,
            "░   {}{}░",
            matchup_string,
            format::space(75 - matchup_string.len())
        )?;
        writeln!(f, "░{}░", format::space(78))?;
        writeln!(
            f,
            "░{:^16}|{}{:.1}%{}{}{}{}{:.1}%{}|{:^16}░",
            self.card.away().team_name().0,
            away_start_seq,
            away_prob,
            away_end_seq,
            away_p_bar,
            home_p_bar,
            home_start_seq,
            home_prob,
            home_end_seq,
            self.card.home().team_name().0,
        )?;
        write!(f, "{}", format::bar(80))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    use crate::stats::season_period::SeasonPeriod;
    use crate::stats::{prediction::Prediction, record::Record, teamcard::TeamCard};
    use crate::types::{GameDate, GameId, SeasonId, TeamAbbreviation, TeamId, TeamName};

    #[test]
    fn test_serialize() {
        let home = TeamCard::new(
            TeamId(12345),
            TeamName("Foo Foos".to_owned()),
            TeamAbbreviation::from_str("Foo")
                .expect("failed to create TeamAbbreviation with test string: \"Foo\""),
            Record::new(),
        );

        let away = TeamCard::new(
            TeamId(67890),
            TeamName("Bar Bars".to_owned()),
            TeamAbbreviation::from_str("Bar")
                .expect("failed to create TeamAbbreviation with test string: \"Bar\""),
            Record::new(),
        );

        let card = GameCard::new(
            GameId::from("0000000001"),
            SeasonId::from((2023, SeasonPeriod::RegularSeason)),
            GameDate::from_str("2023-12-01").unwrap(),
            home.clone(),
            away.clone(),
        );

        let prediction = Prediction::new(&card, 0.7);

        let serialized = serde_json::to_string(&prediction).unwrap();
        assert_eq!(
            serialized,
            r#"{"game_id":"0000000001","season_id":"22023","date":"2023-12-01","home":{"team_id":12345,"team_name":"Foo Foos","team_abbr":"Foo","record":"0-0"},"away":{"team_id":67890,"team_name":"Bar Bars","team_abbr":"Bar","record":"0-0"},"probability":0.7}"#
        );
    }
}
