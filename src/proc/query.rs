use std::str::FromStr;

use reqwest::{
    header::{
        HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION,
        ORIGIN, PRAGMA, REFERER, USER_AGENT,
    },
    Client, Response,
};
use serde_json::Value;

use crate::{format::url_format::UrlFormatter, stats::nba_kind::NBAStatKind, types::SeasonId};

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
