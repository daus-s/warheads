use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::read_checksum::read_checksum;
use crate::checksum::sign::sign_nba;

use crate::dapi::team_box_score::TeamBoxScore;

use crate::format::parse::{destructure_dt, DT};
use crate::format::path_manager::{nba_checksum_path, nba_data_path, universal_nba_data_path};
use crate::format::url_format::UrlFormatter;

use crate::proc::gather;
use crate::proc::gather::{player_games, team_games};

use crate::stats::id::Identity;
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
use reqwest::{Client, Response};
use serde_json::Value;

use std::str::FromStr;

/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

pub const BEGINNING: i32 = 1946;

pub fn load_nba_season_from_source(year: i32) -> Vec<(Identity, TeamBoxScore)> {
    let mut team_games_vec = Vec::new();

    for period in minimum_spanning_era(year) {
        let player_path = nba_data_path(period, Player);

        let player_games_of_period = player_games(period, &player_path).unwrap_or_else(|e| {
            panic!(
                "{e}\n\
                💀 failed to load and parse player games as JSON.\n\
                run `cargo test checksum::assert_checksums`"
            );
        });

        let team_path = nba_data_path(period, Team);

        let team_games_of_period = team_games(period, &team_path, player_games_of_period)
            .unwrap_or_else(|e| {
                panic!(
                    "{e}\n\
                    💀 failed to load and parse team games as JSON.\n\
                    run `cargo test checksum::assert_checksums`"
                );
            });

        team_games_vec.extend(team_games_of_period);
    }

    team_games_vec
}

pub async fn observe_nba() {
    let checksums = ChecksumMap::load().expect("💀 failed to load checksums");

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

    let begin = BEGINNING; //first year of the nba in record is 1946-1947 szn

    for year in begin..curr_year + seasonal_depression {
        for era in minimum_spanning_era(year) {
            compare_and_fetch(era, Player, &checksums).await;
            compare_and_fetch(era, Team, &checksums).await;
        }
    }

    match sign_nba() {
        Ok(_) => println!(
            "✅ successfully signed nba data with checksums in {}",
            nba_checksum_path().display()
        ),
        Err(_) => eprintln!(
            "❌ failed to sign nba data with checksums in {}",
            nba_checksum_path().display()
        ),
    };
}
/// Compare the checksums of a NBA data source file and if it matches the expected checksum we can bypass refetching from
/// [nba.com/stats](https://www.nba.com/stats). Otherwise we proceed fetching the data and saving the data to our source directory.
async fn compare_and_fetch(season_id: SeasonId, kind: NBAStatKind, checksums: &ChecksumMap) {
    let source_path = nba_data_path(season_id, kind);
    let checksum_path = universal_nba_data_path(season_id, kind);
    if !source_path.exists()
        || read_checksum(&source_path).expect("💀 failed to read file data even though the path exists.")
            != *checksums
                .get(&checksum_path)
                .expect("💀 failed to find checksum for an existent path. all checksums should be initialized")
    //this might fail on new records
    {
        if let Err(msg) = gather::fetch_and_save_nba_stats(season_id, kind).await {
            println!("{}", msg);
        } else {
            println!("✅ successfully wrote {kind} data to file for the {season_id}");
        }
    } else {
        println!("✅ bypassing fetching {kind} data for the {season_id}, checksums match. ");
    }
}

pub async fn query_nba(season: SeasonId, stat_kind: NBAStatKind) -> Result<Value, String> {
    // if more url-encoded characters are needed you can use `urlencoding` crate

    let response = make_nba_request(season, stat_kind, None, None).await?;

    if response.status().is_success() {
        let json_text = response
            .text()
            .await
            .map_err(|e| format!("❌ request failed with error: {}", e))?;

        let json = serde_json::from_str(&json_text)
            .map_err(|e| format!("❌ failed to parse JSON: {}", e))?;

        Ok(json)
    } else {
        Err(format!(
            "❌ request failed with status: {}\nurl: {}",
            response.status(),
            response.url()
        ))
    }
}

//return url and headers for nba request given customizable parameters.
// mainly for internal use.
pub(crate) async fn make_nba_request(
    season: SeasonId,
    kind: NBAStatKind,
    from: Option<String>,
    to: Option<String>,
) -> Result<Response, String> {
    let client = Client::new();

    let url = build_url(
        season,
        kind,
        from.unwrap_or_default(),
        to.unwrap_or_default(),
    );

    let headers = build_headers();

    client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("❌ request failed with error: {}", e))
}

fn build_headers() -> HeaderMap {
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

    headers
}

fn build_url(season: SeasonId, kind: NBAStatKind, from: String, to: String) -> String {
    format!(
        "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom={}&DateTo={}&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
        from,
        to,
        kind.url(),
        season.year().url(),
        season.period().url()
    )
}
