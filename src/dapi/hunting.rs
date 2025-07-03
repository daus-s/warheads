/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

use crate::stats::team_box_score::TeamBoxScore;

use crate::dapi::gather::{ask_nba, player_games, team_games, write_games};
use crate::dapi::parse::{destructure_dt, DT};
use crate::dapi::store::save_nba_season;
use crate::format::path_manager::nba_data_path;
use crate::format::season::season_fmt;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::SeasonPeriod;
use crate::types::SeasonId;
use chrono;
use chrono::Local;

pub fn load_nba_season_from_file(year: i32) -> Vec<TeamBoxScore> {
    let player_games = player_games(year);

    team_games(year, player_games)
}

/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    let todays_date = Local::now();

    let DT { year, month, day } = destructure_dt(todays_date);

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        save_nba_season(szn).await;
    }
}

pub async fn observe_nba() {
    let todays_date = Local::now();

    let DT {
        year: curr_year,
        month,
        day,
    } = destructure_dt(todays_date);

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for year in begin..curr_year + seasonal_depression {
        if year >= 2003 {
            if let Err(error_message) = fetch_and_save_nba_stats(
                SeasonId::from((year, SeasonPeriod::PreSeason)),
                NBAStatKind::Player,
            )
            .await
            {
                eprintln!("{}", error_message);
            }

            if let Err(error_message) = fetch_and_save_nba_stats(
                SeasonId::from((year, SeasonPeriod::PreSeason)),
                NBAStatKind::Team,
            )
            .await
            {
                eprintln!("{}", error_message);
            }
        }

        if let Err(error_message) = fetch_and_save_nba_stats(
            SeasonId::from((year, SeasonPeriod::RegularSeason)),
            NBAStatKind::Player,
        )
        .await
        {
            eprintln!("{}", error_message);
        }

        if let Err(error_message) = fetch_and_save_nba_stats(
            SeasonId::from((year, SeasonPeriod::RegularSeason)),
            NBAStatKind::Team,
        )
        .await
        {
            eprintln!("{}", error_message);
        }

        if year >= 2020 {
            if let Err(error_message) = fetch_and_save_nba_stats(
                SeasonId::from((year, SeasonPeriod::PlayIn)),
                NBAStatKind::Player,
            )
            .await
            {
                eprintln!("{}", error_message);
            }

            if let Err(error_message) = fetch_and_save_nba_stats(
                SeasonId::from((year, SeasonPeriod::PlayIn)),
                NBAStatKind::Team,
            )
            .await
            {
                eprintln!("{}", error_message);
            }
        }

        if let Err(error_message) = fetch_and_save_nba_stats(
            SeasonId::from((year, SeasonPeriod::PostSeason)),
            NBAStatKind::Player,
        )
        .await
        {
            eprintln!("{}", error_message);
        }

        if let Err(error_message) = fetch_and_save_nba_stats(
            SeasonId::from((year, SeasonPeriod::PostSeason)),
            NBAStatKind::Team,
        )
        .await
        {
            eprintln!("{}", error_message);
        }
    }
}

async fn fetch_and_save_nba_stats(season: SeasonId, stat: NBAStatKind) -> Result<(), String> {
    let file_path = || nba_data_path(season, stat);

    let (year, _period) = season.destructure();

    match ask_nba(season, stat).await {
        Ok(response_data) => match write_games(file_path(), &response_data) {
            Ok(_) => {
                println!(
                    "✅ successfully saved nba stats for {} season at file: {:?}",
                    season_fmt(season.year()),
                    file_path()
                );
                Ok(())
            }
            Err(e) => Err(format!(
                "❌ error saving nba stats for {} season at file {:?}: {}",
                season_fmt(season.year()),
                file_path(),
                e
            )),
        },
        Err(e) => Err(format!(
            "❌ failed to fetch {} stats for {} season: {:?}",
            year, stat, e
        )),
    }
}

pub fn process_elo() {
    todo!("assign elo values to players on a game by game basis")
}
