use crate::{stats::gamecard::GameCard, types::GameDate};

use serde::{Deserialize, Serialize};

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
