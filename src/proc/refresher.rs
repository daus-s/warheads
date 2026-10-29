use crate::checksum::sign::sign_nba;

use crate::dapi::season_manager::get_current_era;

use crate::proc::hunting::fetch_and_save_nba_stats;

use crate::stats::nba_kind::NBAStatKind;

/// check whether todays gamecards have been completed and if not query and update the source data.
///
/// ### returns
/// `true` if the source data was updated, `false` otherwise.
pub(crate) async fn update_source_data() -> Result<(), ()> {
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

    Ok(())
}

#[cfg(test)]
mod test_get_daily_gamecard {

    use chrono::NaiveDate;

    use crate::format::parse::parse_gamecards;
    use crate::proc::query::get_gamecard_json;

    use crate::stats::gamecard::GameCard;
    use crate::stats::record::Record;
    use crate::stats::teamcard::TeamCard;

    use crate::types::{GameId, SeasonId, TeamAbbreviation, TeamId, TeamName};

    #[tokio::test]
    async fn test_setdate_gamecards() {
        let setdate = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap();

        let json_response = get_gamecard_json(setdate)
            .await
            .unwrap_or_else(|err| panic!("💀 failed to get daily gamecard: {}", err));

        let mut gamecards = parse_gamecards(json_response)
            .unwrap_or_else(|err| panic!("💀 failed to parse gamecards: {}", err));

        gamecards.sort_by_key(|g| g.date());

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

        let g1 = GameCard::new(
            GameId(22500001),
            SeasonId::from(22025),
            day1.into(),
            tc1h,
            tc1a,
        );

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

        let g2 = GameCard::new(
            GameId(22500002),
            SeasonId::from(22025),
            day2.into(),
            tc2h,
            tc2a,
        );

        vec![g1, g2]
    }
}
