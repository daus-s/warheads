use crate::checksum::sign::sign_nba;

use crate::dapi::currency::source_data_current;
use crate::dapi::season_manager::get_current_era;

use crate::format::parse::parse_gamecards;

use crate::proc::gather::fetch_and_save_nba_stats;
use crate::proc::query::{daily_gamecard_json, NBAQueryError};

use crate::stats::gamecard::GameCard;
use crate::stats::nba_kind::NBAStatKind;

use crate::types::GameDate;

/// check whether todays gamecards have been completed and if not query and update the source data.
///
/// ### returns
/// `true` if the source data was updated, `false` otherwise.
pub async fn update_source_data() -> bool {
    if !source_data_current().await {
        let current_era = get_current_era();

        match fetch_and_save_nba_stats(current_era, NBAStatKind::Player).await {
            Ok(_) => println!("✅ updated player source data for the {}", current_era),
            Err(e) => println!(
                "{e}\n❌ failed to fetch and update NBA player source data for the {}",
                current_era
            ),
        };

        match fetch_and_save_nba_stats(current_era, NBAStatKind::Team).await {
            Ok(_) => println!("✅ updated team source data for the {}", current_era),
            Err(e) => println!(
                "{e}\n❌ failed to fetch and update NBA team source data for the {}",
                current_era
            ),
        };

        match sign_nba() {
            Ok(_) => println!("✅ updated NBA source data checksums. "),
            Err(_) => println!("❌ failed to update NBA source data checksum"),
        };

        true
    } else {
        false
    }
}

// game schedule tracker
pub async fn get_daily_gamecard() -> Result<Vec<GameCard>, NBAQueryError> {
    let response = daily_gamecard_json(GameDate::today()).await?;

    let gamecards =
        parse_gamecards(response).map_err(|e| NBAQueryError::ObjectStructureError(e))?;

    Ok(gamecards)
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
