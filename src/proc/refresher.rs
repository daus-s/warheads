use crate::format::parse::parse_gamecards;

use crate::proc::query::{daily_gamecard_json, NBAQueryError};

use crate::stats::gamecard::GameCard;
use crate::types::GameDate;

// game schedule tracker
pub async fn get_daily_gamecard() -> Result<Vec<GameCard>, NBAQueryError> {
    let response = daily_gamecard_json(GameDate::today()).await?;

    let gamecards =
        parse_gamecards(response).map_err(|e| NBAQueryError::ObjectStructureError(e))?;

    Ok(gamecards)
}

pub fn check_source_data_for_games(gamecards: &[GameCard]) -> bool {
    let mut present = true;

    for gamecard in gamecards {
        present &= gamecard.check_source_data();
    }

    present
}

#[cfg(test)]
mod test_get_daily_gamecard {

    use chrono::NaiveDate;

    use crate::stats::record::Record;
    use crate::stats::teamcard::TeamCard;
    use crate::types::{GameDate, GameId, TeamAbbreviation, TeamId, TeamName};

    use super::*;

    #[tokio::test]
    async fn test_get_daily_gamecard() {
        let response = get_daily_gamecard().await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_setdate_gamecards() {
        let setdate = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap();

        let json_response = daily_gamecard_json(GameDate::from(setdate))
            .await
            .unwrap_or_else(|err| panic!("💀 failed to get daily gamecard: {}", err));

        dbg!(&json_response);

        let mut gamecards = parse_gamecards(json_response)
            .unwrap_or_else(|err| panic!("💀 failed to parse gamecards: {}", err));

        gamecards.sort_by_key(|g| g.game_id());

        pretty_assertions::assert_eq!(gamecards, expected_gamecards());
    }

    fn expected_gamecards() -> Vec<GameCard> {
        let day1 = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap();
        let day2 = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap(); //fuck timezones

        let tc1h = TeamCard::new(
            TeamId(1610612760),
            TeamName("Thunder".to_owned()),
            TeamAbbreviation("OKC".to_owned()),
            Record { wins: 1, losses: 0 },
        );

        let tc1a = TeamCard::new(
            TeamId(1610612745),
            TeamName("Rockets".to_owned()),
            TeamAbbreviation("HOU".to_owned()),
            Record { wins: 1, losses: 0 },
        );

        let g1 = GameCard::new(GameId(22500001), day1.into(), tc1h, tc1a);

        let tc2h = TeamCard::new(
            TeamId(1610612747),
            TeamName("Lakers".to_owned()),
            TeamAbbreviation("LAL".to_owned()),
            Record { wins: 0, losses: 1 },
        );

        let tc2a = TeamCard::new(
            TeamId(1610612744),
            TeamName("Warriors".to_owned()),
            TeamAbbreviation("GSW".to_owned()),
            Record { wins: 1, losses: 0 },
        );

        let g2 = GameCard::new(GameId(22500002), day2.into(), tc2h, tc2a);

        vec![g1, g2]
    }

    #[test]
    fn test_check_source_data() {
        let mut present = true;

        for gamecard in expected_gamecards() {
            present &= gamecard.check_source_data();
        }

        assert!(present);
    }
}
