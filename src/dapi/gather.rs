use crate::dapi::rip::fetch_and_process_nba_games;
use crate::format::season::season_fmt;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_stat::NBAStat::{Player, Team};
use crate::stats::player_box_score::PlayerBoxScore;
use crate::stats::season_period::minimum_spanning_era;
use crate::stats::team_box_score::TeamBoxScore;
use crate::types::SeasonId;
use reqwest;
use reqwest::header::*;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

pub fn read_nba_file(file_path: PathBuf) -> String {
    let mut file =
        File::open(&file_path).expect(format!("Failed to open {}", file_path.display()).as_str());

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect(format!("Failed to read {}", file_path.display()).as_str());

    contents
}

pub async fn ask_nba(season: SeasonId, stat_kind: NBAStatKind) -> Result<String, Box<dyn Error>> {
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

pub fn write_games(file_path: PathBuf, raw_json: &str) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        //this creates the directory from the ground up.
        fs::create_dir_all(parent)?;
    }

    fs::write(file_path, raw_json)
}

pub fn player_games(year: i32) -> Vec<PlayerBoxScore> {
    let minimum_spanning_era = minimum_spanning_era(year);

    minimum_spanning_era
        .iter()
        .flat_map(|&season| {
            fetch_and_process_nba_games(season, NBAStatKind::Player)
                .into_iter()
                .filter_map(|stat| match stat {
                    Player(p) => Some(p),
                    _ => None,
                })
        })
        .collect()
}

pub fn team_games(year: i32, roster: Vec<PlayerBoxScore>) -> Vec<TeamBoxScore> {
    let minimum_spanning_era = minimum_spanning_era(year);

    let mut games: Vec<TeamBoxScore> = minimum_spanning_era
        .iter()
        .flat_map(|&season| {
            fetch_and_process_nba_games(season, NBAStatKind::Team)
                .into_iter()
                .filter_map(|stat| match stat {
                    Team(t) => Some(t),
                    _ => None,
                })
        })
        .collect();

    for player in roster {
        for team in &mut games {
            if player.played_in(&team) {
                team.add_player_stats(player.clone());
            }
        }
    }

    games.clone()
}
