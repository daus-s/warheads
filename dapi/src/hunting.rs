/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

use stats::kind::NBAStat::*;
use stats::player_box_score::PlayerBoxScore;
use stats::team_box_score::TeamBoxScore;
use crate::rip::process_nba_games;

use chrono;
use chrono::{DateTime, Datelike, Local};
use indicatif::{ProgressBar, ProgressStyle};
use corrections::correction::Correction;
use corrections::correction_builder::CorrectionBuilder;
use stats::kind::NBAStatKind;

use corrections::corrector::Corrector;

fn player_games(year: i32) -> Vec<PlayerBoxScore> {
    match process_nba_games(year, NBAStatKind::Player) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                Player(p) => Some(p),
                _ => None,
            })
            .collect::<Vec<PlayerBoxScore>>(),
        Err(mut meta) => {
            let corrections: Vec<Correction> = meta.into_iter().map(
                |(corr, info)|
                    CorrectionBuilder::new(corr, info).create()
            ).collect();

            corrections.iter().for_each(|c| {
                let _ = c.save(); // we assume t that saving to the file is allowed and disregard
                                  // the result. add more robust error handling here.
            });

            //create will be called from a

            corrections.apply()
                .map(|_| player_games(year))
                .unwrap_or_else(|e| panic!("failed to apply corrections: {}", e))
        }
    }
}


fn team_games(year: i32, roster: Vec<PlayerBoxScore>) -> Vec<TeamBoxScore> {
    let mut games:Vec<TeamBoxScore> = match process_nba_games(year, NBAStatKind::Team) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                Team(t) => Some(t),
                _ => None,
            })
            .collect::<Vec<TeamBoxScore>>(),
        Err(mut meta) => {
            let corrections: Vec<Correction> = meta.into_iter().map(
                |(corr, info)|
                    CorrectionBuilder::new(corr, info).create()
            ).collect();

            corrections.iter().for_each(|c| {
                let _ = c.save(); // we assume t that saving to the file is allowed and disregard
                // the result. add more robust error handling here.
            });

            //create will be called from a

            corrections.apply()
                .map(|_| {
                    println!("successfully applied corrections. ");
                    team_games(year, roster.clone())
                })
                .unwrap_or_else(|e| {
                    panic!("failed to apply corrections: {}", e)
                })
        }
    };

    for player in roster {
        for team in &mut games {
            if player.played_in(&team) {
                team.add_player_stats(player.clone());
            }
        }
    }

    games

}
pub fn load_nba_season(year: i32) -> Vec<TeamBoxScore> {
    let player_games = player_games(year);

    team_games(year, player_games)
}

pub async fn save_nba_season(year: i32) {

    let team_games = load_nba_season(year);

    sub_save(team_games).await;
}

async fn sub_save(games: Vec<TeamBoxScore>) {
    let client = sss::client::create().await;

    let num_games = games.len() as u64;

    let pb = ProgressBar::new(num_games);

    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {bar:40} | {pos}/{len} [{eta}]")
        .unwrap()
        .progress_chars("#>-"));

    for game in &games {
        sss::store_three::save_nba_game(client.clone(), game.clone()).await.unwrap();

        pb.inc(1);
    }

    pb.finish_with_message(format!("saved {} season.", games[0].season_str()));
}

/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    let todays_date = Local::now();

    let D {year, month, day} = destructure_dt(todays_date);

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 { 1 } else { 0 };// august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        save_nba_season(szn).await;
    }
}

fn destructure_dt(dt: DateTime<Local>) -> D {
    D {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
    }
}

struct D {
    year: i32,
    month: u32,
    day: u32,
}
pub fn process_elo() {
    todo!("assign elo values to players on a game by game basis")
}