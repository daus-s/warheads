/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

use stats::team_box_score::TeamBoxScore;

use chrono;
use chrono::Local;
use format::path_manager::data_path;
use format::season::season_fmt;
use stats::nba_kind::NBAStatKind;
use stats::season_type::SeasonPeriod;
use crate::gather::{ask_nba, player_games, team_games, write_games};
use crate::parse::{destructure_dt, DT};
use crate::store::save_nba_season;


pub fn load_nba_season_from_file(year: i32) -> Vec<TeamBoxScore> {
    let player_games = player_games(year);

    team_games(year, player_games)
}



/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    let todays_date = Local::now();

    let DT {year, month, day} = destructure_dt(todays_date);

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 { 1 } else { 0 };// august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        save_nba_season(szn).await;
    }
}

pub async fn observe_nba() {
    let todays_date = Local::now();

    let DT {year: curr_year, month, day} = destructure_dt(todays_date);

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 { 1 } else { 0 };// august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for year in begin..curr_year + seasonal_depression {
        if year >= 2003 {

            if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Player, SeasonPeriod::PreSeason).await {
                eprintln!("{}", error_message);
            }

            if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Team, SeasonPeriod::PreSeason).await {
                eprintln!("{}", error_message);
            }

        }

        if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Player, SeasonPeriod::RegularSeason).await {
            eprintln!("{}", error_message);
        }

        if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Team, SeasonPeriod::RegularSeason).await {
            eprintln!("{}", error_message);
        }

        if year >= 2020 {

            if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Player, SeasonPeriod::PlayIn).await {
                eprintln!("{}", error_message);
            }

            if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Team, SeasonPeriod::PlayIn).await {
                eprintln!("{}", error_message);
            }

        }

        if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Player, SeasonPeriod::PostSeason).await {
            eprintln!("{}", error_message);
        }

        if let Err(error_message) = fetch_and_save_nba_stats(year, NBAStatKind::Team, SeasonPeriod::PostSeason).await {
            eprintln!("{}", error_message);
        }


    }
}

async fn fetch_and_save_nba_stats(year: i32, stat: NBAStatKind, period: SeasonPeriod) -> Result<(), String> {

    let file_path = || data_path(year, stat, period);

     match ask_nba(year, stat, period).await {
         Ok(response_data) => match write_games(file_path(),  &response_data) {
             Ok(_) => {
                 println!("✅ successfully saved nba stats for {} season at file: {:?}",
                          season_fmt(year),
                          file_path());
                 Ok(())
             }
             Err(e) => {
                 Err(format!("❌ error saving nba stats for {} season at file {:?}: {}",
                           season_fmt(year),
                           file_path(),
                           e
                 ))
             }
         }
         Err(e) => {
             Err(format!("❌ failed to fetch {} stats for {} season: {:?}", year, stat, e))
         }
     }
}

pub fn process_elo() {
    todo!("assign elo values to players on a game by game basis")
}