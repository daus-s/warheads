use std::error::Error;
use std::fs;
use reqwest;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use once_cell::sync::Lazy;
use reqwest::Client;
use reqwest::header::*;
use stats::kind::NBAStatKind;
use format::path_manager::data_path;
use constants::data;
use format::season::season_fmt;
use format::stat_path_formatter::StatPathFormatter as SPF;

static DATA: Lazy<String> = Lazy::new(data);
pub fn read_nba(season: i32, stat: impl SPF) -> String {
    let suffix = (season + 1) % 100;
    let filename = format!("{}/nba/{}/{}_{:02}_{}", *DATA, stat.epath(), season, suffix, stat.ext());

    let mut file = File::open(&filename).expect(&stat.dbg_open(
        season
    ));
    let mut data = String::new();

    file.read_to_string(&mut data).expect(&stat.dbg_write(season));

    data
}



pub async fn ask_nba(year: i32, stat: NBAStatKind) -> Result<(), Box<dyn Error>> {

    let client = Client::new();

    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_str("*/*").unwrap());
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str("en-US,en;q=0.9,de;q=0.8'").unwrap());
    headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache").unwrap());
    headers.insert(CONNECTION, HeaderValue::from_str("keep-alive").unwrap());
    headers.insert(ORIGIN, HeaderValue::from_str("https://www.nba.com").unwrap());
    headers.insert(PRAGMA, HeaderValue::from_str("no-cache").unwrap());
    headers.insert(REFERER, HeaderValue::from_str("https://www.nba.com/").unwrap());
    headers.insert(HeaderName::from_str("Sec-Fetch-Dest").unwrap(), HeaderValue::from_str("empty").unwrap());
    headers.insert(HeaderName::from_str("Sec-Fetch-Mode").unwrap(), HeaderValue::from_str("cors").unwrap());
    headers.insert(HeaderName::from_str("Sec-Fetch-Site").unwrap(), HeaderValue::from_str("same-site").unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap());
    headers.insert(HeaderName::from_str("sec-ch-ua").unwrap(), HeaderValue::from_str("\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"").unwrap());
    headers.insert(HeaderName::from_str("sec-ch-ua-mobile").unwrap(), HeaderValue::from_str("?0").unwrap());
    headers.insert(HeaderName::from_str("sec-ch-ua-platform").unwrap(), HeaderValue::from_str("macOS").unwrap());

    let season = season_fmt(year);

    let file_path = data_path(year, stat);

    let url = format!("https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=P&Season={season}&SeasonType=Regular%20Season&Sorter=DATE");

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {

        let raw_json = response.text().await?;

        // Save the raw JSON to a file
        fs::write(file_path, raw_json).map_err(|e| e.into())

    } else {

        Err(format!("Request failed with status: {}", response.status()).into())

    }

}
