use crate::rip::fetch_and_process_nba_games;
use constants::data;
use format::season::season_fmt;
use format::stat_path_formatter::StatPathFormatter as SPF;
use once_cell::sync::Lazy;
use reqwest;
use reqwest::header::*;
use reqwest::Client;
use stats::nba_kind::NBAStatKind;
use stats::nba_stat::NBAStat::{Player, Team};
use stats::player_box_score::PlayerBoxScore;
use stats::season_type::{minimum_spanning_era, SeasonPeriod};
use stats::team_box_score::TeamBoxScore;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

static DATA: Lazy<String> = Lazy::new(data);
pub fn read_nba_file(season: i32, stat: NBAStatKind) -> String {
    let suffix = (season + 1) % 100;
    let filename = format!(
        "{}/nba/{}/{}_{:02}_{}",
        *DATA,
        stat.path_specifier(),
        season,
        suffix,
        stat.ext()
    );

    let mut file = File::open(&filename).expect(&*stat.dbg_open(season));
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect(&*stat.dbg_write(season));

    data
}

pub async fn ask_nba(
    year: i32,
    stat_kind: NBAStatKind,
    period: SeasonPeriod,
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

    let season = season_fmt(year);

    // if more url-encoded characters are needed you can use `urlencoding` crate
    let url = format!(
        "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={stat_kind}&\
        Season={season}&\
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

pub(crate) fn write_games(file_path: PathBuf, raw_json: &str) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        //this creates the directory from the ground up.
        fs::create_dir_all(parent)?;
    }

    fs::write(file_path, raw_json)
}

pub(crate) fn player_games(year: i32) -> Vec<PlayerBoxScore> {
    let minimum_spanning_era = minimum_spanning_era(year);

    minimum_spanning_era
        .iter()
        .flat_map(|period| {
            fetch_and_process_nba_games(year, NBAStatKind::Player, *period)
                .into_iter()
                .filter_map(|stat| match stat {
                    Player(p) => Some(p),
                    _ => None,
                })
        })
        .collect()
}

pub(crate) fn team_games(year: i32, roster: Vec<PlayerBoxScore>) -> Vec<TeamBoxScore> {
    let minimum_spanning_era = minimum_spanning_era(year);

    let mut games: Vec<TeamBoxScore> = minimum_spanning_era
        .iter()
        .flat_map(|period| {
            fetch_and_process_nba_games(year, NBAStatKind::Player, *period)
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
