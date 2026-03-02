use crate::constants::header_manager::HEADER_MANAGER;

use crate::format::url_format::UrlFormatter;

use crate::format::parse;

use crate::stats::nba_kind::NBAStatKind;

use crate::types::{GameDate, SeasonId};

use std::fmt::Debug;

use reqwest::{Client, Response};

use serde_json::Value;

use thiserror::Error;

/// HISTORY

pub async fn nba_history_json(
    season: SeasonId,
    stat_kind: NBAStatKind,
) -> Result<Value, NBAQueryError> {
    // if more url-encoded characters are needed you can use `urlencoding` crate

    let response = make_nba_history_request(season, stat_kind, None, None).await?;

    if response.status().is_success() {
        let json = response
            .json()
            .await
            .map_err(|e| NBAQueryError::FormatError(e))?;

        Ok(json)
    } else {
        Err(NBAQueryError::ResponseError {
            status: response.status(),
            url: response.url().to_string(),
        })
    }
}

//return url and headers for nba request given customizable parameters.
// mainly for internal use.
pub(crate) async fn make_nba_history_request(
    season: SeasonId,
    kind: NBAStatKind,
    from: Option<GameDate>,
    to: Option<GameDate>,
) -> Result<Response, NBAQueryError> {
    let client = Client::new();

    let url = build_nba_history_url(season, kind, from, to);

    client
        .get(&url)
        .headers(HEADER_MANAGER.history_request_headers())
        .send()
        .await
        .map_err(|e| NBAQueryError::RequestError(e))
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

/// GAMECARD

pub async fn get_gamecard_json(date: GameDate) -> Result<Value, NBAQueryError> {
    let response = make_nba_gamecard_request(date).await?;

    let json = response
        .json()
        .await
        .map_err(|e| NBAQueryError::FormatError(e))?;

    Ok(json)
}

async fn make_nba_gamecard_request(date: GameDate) -> Result<Response, NBAQueryError> {
    let client = Client::builder()
        .gzip(true)
        .brotli(true)
        .build()
        .map_err(|e| NBAQueryError::ClientError(e))?;

    let url = build_nba_gamecard_url(date);

    client
        .get(&url)
        .headers(HEADER_MANAGER.gamecard_request_headers())
        .send()
        .await
        .map_err(|e| NBAQueryError::RequestError(e))
}

fn build_nba_gamecard_url(date: GameDate) -> String {
    format!(
        "https://core-api.nba.com/cp/api/v1.9/feeds/gamecardfeed?gamedate={}&platform=web",
        date
    )
}

/// EDITS

pub(crate) async fn nba_annotation_file() -> Result<Value, NBAQueryError> {
    let response = make_nba_annotations_request().await?;

    let json = response
        .json()
        .await
        .map_err(|e| NBAQueryError::FormatError(e))?;

    Ok(json)
}

async fn make_nba_annotations_request() -> Result<Response, NBAQueryError> {
    let client = Client::new();

    let url = build_nba_annotations_url();

    client
        .get(&url)
        .send()
        .await
        .map_err(|_| NBAQueryError::DriveResourceError)
}

fn build_nba_annotations_url() -> String {
    //publicly shared, no worries about leaking secrets
    "https://drive.google.com/file/d/1r8XyRZN14Z1Q9_7F6KyHOJapd4PUSzaC".to_owned()
}

#[derive(Error, Debug)]
pub enum NBAQueryError {
    #[error("❌ {0}\n❌ HTTP request failed")]
    RequestError(reqwest::Error),

    #[error("{0}\n❌ failed to construct client")]
    ClientError(reqwest::Error),

    #[error("❌ {}\n❌ request succeed but failed with code {}", status, url)]
    ResponseError {
        status: reqwest::StatusCode,
        url: String,
    },
    #[error("{0}\n❌ failed to parse response as json.")]
    FormatError(reqwest::Error),

    #[error("{0}\n❌ unexpected json object structure.")]
    ObjectStructureError(parse::ParseError),

    #[error("Drive Resource Error")]
    DriveResourceError,
}

#[cfg(test)]
mod test_queries {
    use super::*;

    #[tokio::test]
    async fn test_history_query() {
        let response =
            make_nba_history_request(SeasonId::from(22022), NBAStatKind::Player, None, None)
                .await
                .expect("💀 failed to fetch nba history records. ");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_gamecard_query() {
        let response = make_nba_gamecard_request(GameDate::from("10/21/2025"))
            .await
            .expect("💀 failed to fetch nba timeline records. ");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_annotations_query() {
        let response = make_nba_annotations_request()
            .await
            .expect("💀 failed to fetch nba annotations records. ");

        dbg!(&response);

        assert!(response.status().is_success());
    }
}
