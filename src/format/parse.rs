use crate::dapi::from_value::FromValue;
use crate::format::extract::{get_result_set, get_rows, headers};
use crate::stats::gamecard::{GameCard, GameCardBuilder};
use crate::stats::record::Record;
use crate::stats::season_period::SeasonPeriod;
use crate::stats::teamcard::TeamCard;
use crate::types::{GameDate, SeasonId};

use chrono::{DateTime, Datelike, Local, NaiveDate};

use serde_json::Value;

use std::fmt::Debug;
use std::fmt::{self, Display};

use thiserror::Error;

pub fn value_to_date(value: &Value) -> Option<NaiveDate> {
    match value {
        Value::String(s) => NaiveDate::parse_from_str(&*s, "%Y-%m-%d").ok(),
        _ => {
            eprintln!("⚠️ JSON Value to parse GameDate from is not a String. ");

            None
        }
    }
}
/// # returns
///
/// - month:
///   -  Returns the month number starting from 1.
///   -  The return value ranges from 1 to 12.
/// ________________________________________________
/// - day:
///   -  Returns the day of month starting from 1.
///   -  The return value ranges from 1 to 31. (The last day of month differs by months.)
/// ________________________________________________
/// - year:
///   -  Returns the year number in the calendar date.
pub fn destructure_dt(dt: DateTime<Local>) -> DestructuredDateTime {
    DestructuredDateTime {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
    }
}

pub struct DestructuredDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub fn parse_season(value: Value) -> Result<(Vec<Value>, Vec<String>), ParseError> {
    let set = get_result_set(&value).map_err(|_| ParseError::ResultSetError)?;

    let headers: Vec<String> = headers(&set).map_err(|_| ParseError::HeaderRowError)?;

    let rows: Vec<Value> = get_rows(&set).map_err(|_| ParseError::BoxScoreRowsError)?;

    Ok((rows, headers))
}

pub fn parse_gamecards(value: Value) -> Result<Vec<GameCard>, ParseError> {
    let modules = value.get("modules").ok_or(ParseError::ModulesError)?;

    let modules_array = modules.as_array().ok_or(ParseError::ModuleListError)?;

    let head = modules_array[0]
        .as_object()
        .ok_or(ParseError::ModuleListError)?;

    let cards = head.get("cards").ok_or(ParseError::CardError)?;

    let cards_array = cards.as_array().ok_or(ParseError::CardListError)?;

    let mut gamecards = Vec::new();

    for card_json in cards_array {
        if let Some(card) = parse_card(card_json) {
            gamecards.push(card);
        } else {
            return Err(ParseError::CardParseError);
        }
    }

    Ok(gamecards)
}

fn parse_card(value: &Value) -> Option<GameCard> {
    let card = value.as_object()?.get("cardData")?;

    // println!("{}", serde_json::to_string_pretty(&card).unwrap());

    let game_id = card.get("gameId")?.game_id().ok()?;

    let mut gamecard_builder = GameCardBuilder::default();

    gamecard_builder.game_id(game_id);

    let game_time = card.get("gameTimeEastern")?.as_str()?;

    let date = chrono::NaiveDateTime::parse_from_str(game_time, "%Y-%m-%dT%H:%M:%SZ")
        .ok()?
        .date();

    gamecard_builder.date(date.into());

    let home_team = card.get("homeTeam")?;
    let away_team = card.get("awayTeam")?;

    let home = parse_team(home_team.as_object()?)?;
    let away = parse_team(away_team.as_object()?)?;

    gamecard_builder.home(home);
    gamecard_builder.away(away);

    let season_year = card.get("seasonYear")?.as_str()?;
    let season_type = card.get("seasonType")?.as_str()?;

    let year = season_year.split('-').next()?.parse::<i32>().ok()?;
    println!("{}", year);
    let season_period = season_type.parse::<SeasonPeriod>().ok()?;
    println!("{}", season_period);

    let season_id = SeasonId::from((year, season_period));

    gamecard_builder.season_id(season_id);

    let gamecard = gamecard_builder.build().ok()?;

    Some(gamecard)
}

fn parse_team(team: &serde_json::Map<String, Value>) -> Option<TeamCard> {
    let team_id = team.get("teamId")?.team_id().ok()?;

    let team_name = team.get("teamName")?.team_name().ok()?;

    let team_abbr = team.get("teamTricode")?.team_abbreviation().ok()?;

    let wins = team.get("wins")?.as_u64()?;
    let losses = team.get("losses")?.as_u64()?;

    let record = Record::wl(wins, losses);

    Some(TeamCard::new(team_id, team_name, team_abbr, record))
}

#[derive(Error)]
pub enum ParseError {
    ResultSetError,
    HeaderRowError,
    BoxScoreRowsError,
    ModulesError,
    ModuleListError,
    CardError,
    CardListError,
    CardParseError,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ResultSetError => {
                write!(f, "❌  missing result set from GameLog request. ")
            }
            ParseError::HeaderRowError => {
                write!(f, "❌  missing headers from GameLog request. ")
            }
            ParseError::BoxScoreRowsError => {
                write!(f, "❌  missing box scores from GameLog request. ")
            }
            ParseError::ModulesError => {
                write!(f, "❌  missing modules field on GameCard request.")
            }
            ParseError::ModuleListError => {
                write!(f, "❌  module field is not an array.")
            }
            ParseError::CardError => {
                write!(f, "❌  missing cards field in module list.")
            }
            ParseError::CardListError => {
                write!(f, "❌  card field is not an array.")
            }
            ParseError::CardParseError => {
                write!(f, "❌  failed to parse game card.")
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
