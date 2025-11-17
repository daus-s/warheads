use crate::constants::header_manager::HEADER_MANAGER;

use crate::format::url_format::UrlFormatter;

use crate::format::parse;

use crate::stats::nba_kind::NBAStatKind;

use crate::types::{GameDate, SeasonId};

use std::fmt::{Debug, Display};

use reqwest::{Client, Response};

use serde_json::Value;

use thiserror::Error;

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
        Err(NBAQueryError::ResponseError(response))
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

pub async fn get_gamecard_json(date: GameDate) -> Result<Value, NBAQueryError> {
    let response = make_nba_gamecard_request(date).await?;

    let json = response
        .json()
        .await
        .map_err(|e| NBAQueryError::FormatError(e))?;

    Ok(json)
}

pub(crate) async fn make_nba_gamecard_request(date: GameDate) -> Result<Response, NBAQueryError> {
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

#[derive(Error)]
pub enum NBAQueryError {
    ClientError(reqwest::Error),
    RequestError(reqwest::Error),
    ResponseError(reqwest::Response),
    FormatError(reqwest::Error),
    ObjectStructureError(parse::ParseError),
}

impl Debug for NBAQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NBAQueryError::RequestError(e) => {
                write!(f, "{}\n❌ request failed with error", e)
            }
            NBAQueryError::ResponseError(res) => {
                write!(
                    f,
                    "❌ {}\n❌ request succeed but failed with code {}",
                    res.status(),
                    res.url()
                )
            }
            NBAQueryError::ClientError(error) => {
                write!(f, "{}\n❌ failed to construct client", error)
            }
            NBAQueryError::FormatError(error) => {
                write!(f, "{}\n❌ failed to parse response as json.", error)
            }
            NBAQueryError::ObjectStructureError(error) => {
                write!(f, "{}\n❌ unexpected json object structure.", error)
            }
        }
    }
}

impl Display for NBAQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
}
