/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

use stats::kind::{NBAStatKind, NBAStat};
use stats::player_box_score::PlayerBoxScore;
use stats::team_box_score::TeamBoxScore;
use crate::gather::read_nba;
use crate::rip::process_nba_games;

use chrono;
use chrono::{DateTime, Datelike, Local};
use indicatif::{ProgressBar, ProgressStyle};
use stats::format::season_str;

pub async fn save_nba_season(year: i32) {
    let player_games = match process_nba_games(&read_nba(year, NBAStatKind::Player), NBAStatKind::Player) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                NBAStat::Player(p) => Some(p),
                _ => None,
            })
            .collect::<Vec<PlayerBoxScore>>(),
        Err(e) => unreachable!("{}", e),
    };

    let mut team_games = match process_nba_games(&read_nba(year, NBAStatKind::Team), NBAStatKind::Team) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                NBAStat::Team(t) => Some(t),
                _ => None,
            })
            .collect::<Vec<TeamBoxScore>>(),
        Err(e) => unreachable!("{}", e),
    };

    for player in &player_games {
        for team in &mut team_games {
            if player.played_in(team.clone()) {
                team.add_player_stats(player.clone());
            }
        }
    }


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

    pb.finish_with_message(format!("saved {} season.", season_str(games)));
}

/// you can build around this function but not from it... this is the one function to start the nba into memeroy then iterate over elo.
pub async fn chronicle_nba() {
    let todays_date = chrono::offset::Local::now();
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
        month: dt.month() ,
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