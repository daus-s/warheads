#[cfg(test)]
mod test_download {

    use std::fs;
    use std::path::PathBuf;

    use once_cell::sync::Lazy;

    use crate::constants::paths::test;

    use crate::dapi::gather;
    use crate::dapi::hunting::make_nba_request;
    use crate::stats::nba_kind::NBAStatKind;
    use crate::stats::season_period::SeasonPeriod;
    use crate::types::SeasonId;

    static TEST: Lazy<String> = Lazy::new(test);

    #[tokio::test]
    async fn test_download() {
        let season = SeasonId::from((2024, SeasonPeriod::PostSeason));
        //request info
        // url: https://www.nba.com/stats/teams/boxscores?Season=2024-25&SeasonType=Playoffs&DateFrom=04%2F26%2F2025&DateTo=04%2F30%2F2025
        let from = String::from("04%2F26%2F");
        let to = String::from("04%2F30%2F");

        let team_response = make_nba_request(
            season,
            NBAStatKind::Team,
            Some(from.clone()),
            Some(to.clone()),
        )
        .await
        .expect("💀 failed to make request to nba.com/stats (Team)");

        let player_response = make_nba_request(
            season,
            NBAStatKind::Player,
            Some(from.clone()),
            Some(to.clone()),
        )
        .await
        .expect("💀 failed to make request to nba.com/stats (Player)");

        let team_path = PathBuf::from(format!("{}/data/tg.json", *TEST));

        let player_path = PathBuf::from(format!("{}/data/pg.json", *TEST));

        assert!(gather::write_games(
            &team_path,
            &(team_response
                .text()
                .await
                .expect("💀 failed to get text from nba team response. check the response data.")),
        )
        .is_ok());

        assert!(gather::write_games(
            &player_path,
            &(player_response.text().await.expect(
                "💀 failed to get text from nba player response. check the response data."
            )),
        )
        .is_ok());

        let expected_team_file =
            fs::read_to_string(PathBuf::from(format!("{}/data/expected_tg.json", *TEST))).expect(
                &format!(
                    "💀 failed to read test team data string: {}/data/expected_tg.json",
                    *TEST,
                ),
            );

        let actual_team_file =
            fs::read_to_string(&team_path).expect("💀 failed to read fetched team directory");

        assert_eq!(expected_team_file, actual_team_file);
    }
}
