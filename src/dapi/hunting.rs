/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/
use crate::stats::team_box_score::TeamBoxScore;
use std::collections::HashMap;

use crate::corrections::correction_loader::load_corrections;
use crate::corrections::corrector::Corrector;
use crate::dapi::gather;
use crate::dapi::gather::{player_games, team_games};
use crate::dapi::parse::{destructure_dt, DT};
use crate::dapi::store::save_nba_season;
use crate::format::path_manager::nba_data_path;
use crate::format::season::season_fmt;
use crate::stats::domain::Domain;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::stats::season_period::minimum_spanning_era;
use crate::types::SeasonId;
use chrono;
use chrono::Local;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, ORIGIN,
    PRAGMA, REFERER, USER_AGENT,
};
use reqwest::Client;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

pub fn load_nba_season_from_file(year: i32) -> Vec<TeamBoxScore> {
    let player_games = player_games(year);

    team_games(year, player_games)
}

/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    let DT { year, month, day } = destructure_dt(Local::now());

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
    let DT {
        year: curr_year,
        month,
        day,
    } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for year in begin..curr_year+seasonal_depression {
        for era in minimum_spanning_era(year) {
            if let Err(msg) = gather::fetch_and_save_nba_stats(&era, Player).await {
                eprintln!("{}", msg);
            }

            if let Err(msg) = gather::fetch_and_save_nba_stats(&era, Team).await {
                eprintln!("{}", msg);
            }
        }
    }
}

pub fn process_elo() {
    todo!("assign elo values to players on a game by game basis")
}

pub async fn query_nba(
    season: &SeasonId,
    stat_kind: NBAStatKind,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_str("*/*").unwrap());
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_str("en-US,en;q=0.9,de;q=0.8'").unwrap(),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache").unwrap());
    headers.insert(CONNECTION, HeaderValue::from_str("keep-alive").unwrap());
    headers.insert(
        ORIGIN,
        HeaderValue::from_str("https://www.nba.com").unwrap(),
    );
    headers.insert(PRAGMA, HeaderValue::from_str("no-cache").unwrap());
    headers.insert(
        REFERER,
        HeaderValue::from_str("https://www.nba.com/").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Sec-Fetch-Dest").unwrap(),
        HeaderValue::from_str("empty").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Sec-Fetch-Mode").unwrap(),
        HeaderValue::from_str("cors").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Sec-Fetch-Site").unwrap(),
        HeaderValue::from_str("same-site").unwrap(),
    );
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap());
    headers.insert(
        HeaderName::from_str("sec-ch-ua").unwrap(),
        HeaderValue::from_str(
            "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        )
        .unwrap(),
    );
    headers.insert(
        HeaderName::from_str("sec-ch-ua-mobile").unwrap(),
        HeaderValue::from_str("?0").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("sec-ch-ua-platform").unwrap(),
        HeaderValue::from_str("macOS").unwrap(),
    );

    let season_param = season_fmt(season.year());

    let period = season.period();

    // if more url-encoded characters are needed you can use `urlencoding` crate
    let url = format!(
        "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={stat_kind}&\
        Season={season_param}&\
        SeasonType={period}&\
        Sorter=DATE"
    );

    let response = client.get(&url).headers(headers).send().await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(format!(
            "❌ request failed with status: {}\nurl: {}",
            response.status(),
            &url
        )
        .into())
    }
}

pub fn revise_nba() {
    let DT { year, month, day } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        let eras = minimum_spanning_era(szn);

        let player_corrections = load_corrections(szn, Player);

        let mut player_archives = eras
            .iter()
            .map(|x| (x, Player))
            .map(|(&s, k)| ((s, k), nba_data_path(&s, k)))
            .collect::<HashMap<Domain, PathBuf>>();

        if let Ok(corrections) = player_corrections {
            if let Err(msg) = corrections.apply(&mut player_archives) {
                println!(
                    "{msg}\n⚠️ failed to overwrite NBA player data for {}",
                    season_fmt(szn)
                );
            }
        } else if let Err(msg) = player_corrections {
            eprintln!(
                "{msg}\n⚠️ failed to load player corrections for the {} season:",
                season_fmt(szn)
            )
        }

        let team_corrections = load_corrections(szn, Team);

        let mut team_archives = eras
            .iter()
            .map(|x| (x, Team))
            .map(|(&s, k)| ((s, k), nba_data_path(&s, k)))
            .collect::<HashMap<Domain, PathBuf>>();

        if let Ok(corrections) = team_corrections {
            if let Err(msg) = corrections.apply(&mut team_archives) {
                println!(
                    "{msg}\n⚠️ failed to overwrite NBA team data for {}",
                    season_fmt(szn)
                );
            }
        } else if let Err(msg) = team_corrections {
            eprintln!(
                "{msg}\n⚠️ failed to load team corrections for the {} season:",
                season_fmt(szn)
            )
        }
    }
}
