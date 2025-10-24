use reqwest::{Client, Response};
use serde_json::Value;

use crate::{
    constants::header_manager::HEADER_MANAGER,
    format::url_format::UrlFormatter,
    stats::nba_kind::NBAStatKind,
    types::{GameDate, SeasonId},
};

pub async fn query_nba_history(season: SeasonId, stat_kind: NBAStatKind) -> Result<Value, String> {
    // if more url-encoded characters are needed you can use `urlencoding` crate

    let response = make_nba_history_request(season, stat_kind, None, None).await?;

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
pub(crate) async fn make_nba_history_request(
    season: SeasonId,
    kind: NBAStatKind,
    from: Option<GameDate>,
    to: Option<GameDate>,
) -> Result<Response, String> {
    let client = Client::new();

    let url = build_nba_history_url(season, kind, from, to);

    client
        .get(&url)
        .headers(HEADER_MANAGER.history_request_headers())
        .send()
        .await
        .map_err(|e| format!("❌ request failed with error: {}", e))
}

pub async fn make_nba_gamecard_request(date: GameDate) -> Result<Response, String> {
    let client = Client::new();

    let url = build_nba_gamecard_url(date);

    client
        .get(&url)
        .headers(HEADER_MANAGER.gamecard_request_headers())
        .send()
        .await
        .map_err(|e| format!("❌ request failed with error: {}", e))
}

fn build_nba_history_url(
    season: SeasonId,
    kind: NBAStatKind,
    from: Option<GameDate>,
    to: Option<GameDate>,
) -> String {
    format!(
        "\
            https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom={}&DateTo={}&\
            Direction=DESC&ISTRound=&\
            LeagueID=00&\
            PlayerOrTeam={}&\
            Season={}&\
            SeasonType={}&\
            Sorter=DATE",
        from.map(|date| date.to_string()).unwrap_or_default(),
        to.map(|date| date.to_string()).unwrap_or_default(),
        kind.url(),
        season.year().url(),
        season.period().url()
    )
}

fn build_nba_gamecard_url(date: GameDate) -> String {
    format!(
        "https://core-api.nba.com/cp/api/v1.9/feeds/gamecardfeed?gamedate={}&platform=web",
        date
    )
}

#[cfg(test)]
mod test_queries {
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn test_history_query() {
        let response =
            make_nba_history_request(SeasonId::from(22022), NBAStatKind::Player, None, None)
                .await
                .unwrap();

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_gamecard_query() {
        let response = make_nba_gamecard_request(GameDate::from("10/21/2025"))
            .await
            .unwrap();

        assert!(response.status().is_success());
    }
}
