use crate::format::language::Columnizable;
use crate::stats::domain::Domain;
use crate::stats::game_data::GameData;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::types::{GameDate, GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};
use chrono::NaiveDate;
use serde_json::Value;
use serde_json::Value::Number;
use std::fmt::{Debug, Display, Formatter};

/// all identifiable structs can generate a identity struct by calling the identity method on the
/// struct. this implementation is generic while the struct below is a specific to the
pub trait Identifiable {
    fn identity(&self) -> Identity;
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Identity {
    pub season_id: SeasonId,

    ///
    /// this field is the player_id of the identity object.
    /// in the case of team objects it is `None`.
    ///
    /// However, in the case of identifying a player object it
    /// is wrapped in `Some(PlayerId)`
    ///
    pub player_id: Option<PlayerId>,

    pub team_id: TeamId,

    pub team_abbr: TeamAbbreviation,

    pub game_id: GameId,

    pub game_date: GameDate,
}

impl Identity {
    pub fn domain(&self) -> Domain {
        (
            self.season_id,
            match self.player_id {
                None => Team,
                Some(_) => Player,
            },
        )
    }

    pub fn game(&self) -> GameData {
        GameData::new(self.season_id, self.game_id, self.team_id)
    }

    pub fn team_abbr(&self) -> TeamAbbreviation {
        self.team_abbr.to_owned()
    }
}

impl Debug for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.player_id {
            Some(id) => write!(
                f,
                "player_id: {}\nteam: {}\nteam_id: {}\nyear: {}\ngame: {}",
                id, self.team_abbr, self.team_id, self.season_id, self.game_id
            ),
            None => write!(
                f,
                "team: {}\nteam_id: {}\nyear: {}\ngame: {}",
                self.team_abbr, self.team_id, self.season_id, self.game_id
            ),
        }
    }
}

impl Display for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.player_id {
            Some(id) => write!(
                f,
                "player_id: {}\nteam: {}\nyear: {}\ngame: {}",
                id, self.team_abbr, self.season_id, self.game_id
            ),
            None => write!(
                f,
                "team: {}\nyear: {}\ngame: {}",
                self.team_abbr, self.season_id, self.game_id
            ),
        }
    }
}

///
///     player_game schema:
///         ["SEASON_ID","PLAYER_ID","PLAYER_NAME","TEAM_ID","TEAM_ABBREVIATION","TEAM_NAME",
///          "GAME_ID","GAME_DATE","MATCHUP","WL","MIN","FGM","FGA","FG_PCT","FG3M","FG3A",
///          "FG3_PCT","FTM","FTA","FT_PCT","OREB","DREB","REB","AST","STL","BLK","TOV","PF","PTS",
///          "PLUS_MINUS","FANTASY_PTS","VIDEO_AVAILABLE"]
///
///     team_game schema:
///         ["SEASON_ID","TEAM_ID","TEAM_ABBREVIATION","TEAM_NAME","GAME_ID","GAME_DATE","MATCHUP",
///          "WL","MIN","FGM","FGA","FG_PCT","FG3M","FG3A","FG3_PCT","FTM","FTA","FT_PCT","OREB",
///          "DREB","REB","AST","STL","BLK","TOV","PF","PTS","PLUS_MINUS","VIDEO_AVAILABLE"]
///
impl Identifiable for String {
    fn identity(&self) -> Identity {
        let columns = self.columns();

        match columns.as_slice() {
            [
                season_id,
                player_id,
                _player_name,
                team_id,
                team_abbr,
                _team_name,
                game_id,
                game_date,
                _matchup,
                _wl,
                _min,
                _fgm,
                _fga,
                _fg_pct,
                _fg3m,
                _fg3a,
                _fg3_pct,
                _ftm,
                _fta,
                _ft_pct,
                _oreb,
                _dreb,
                _reb,
                _ast,
                _stl,
                _blk,
                _tov,
                _pf,
                _pts,
                _plus_minus,
                _fantasy_pts,
                _video_available,
            ] => {
                let szn = season_id.replace('"', "").parse::<i32>().expect("💀 failed to parse season id as an i32 (pre-conversion) while identifying a player game. player");

                let pid = player_id.replace('\"', "").parse::<u64>().expect(
                    "💀 failed to parse player id as an u64 while identifying a player game. ",
                );

                let date = NaiveDate::parse_from_str(&*game_date, "%Y-%m-%d")
                    .expect("💀 failed to parse game date as an chrono::NaiveDate. ");

                let tid = team_id.replace('\"', "").parse::<u64>().expect(
                    "💀 failed to parse team id as an u64 while identifying a player game. ",
                );

                let gid = game_id.replace('\"', "");

                let tab = team_abbr.replace('\"', "");

                Identity {
                    season_id: SeasonId::from(szn),
                    player_id: Some(PlayerId(pid)),
                    team_id: TeamId(tid),
                    team_abbr: TeamAbbreviation(tab),
                    game_id: GameId::from(gid),
                    game_date: GameDate(date),
                }
            }
            [
                season_id,
                team_id,
                team_abbr,
                _team_name,
                game_id,
                game_date,
                _matchup,
                _wl,
                _min,
                _fgm,
                _fga,
                _fg_pct,
                _fg3m,
                _fg3a,
                _fg3_pct,
                _ftm,
                _fta,
                _ft_pct,
                _oreb,
                _dreb,
                _reb,
                _ast,
                _stl,
                _blk,
                _tov,
                _pf,
                _pts,
                _plus_minus,
                _video_available,
            ] => {
                let szn = season_id.replace('\"', "").parse::<i32>().expect("💀 failed to parse season id as an u64 (pre-conversion) while identifying a team game. ");

                let tid = team_id
                    .replace('\"', "")
                    .parse::<u64>()
                    .expect("💀 failed to parse team id as an u64 while identifying a team game. ");

                let date = NaiveDate::parse_from_str(&*game_date, "%Y-%m-%d")
                    .expect("💀 failed to parse game date as an chrono::NaiveDate. ");

                let gid = game_id.replace('\"', "");

                let tab = team_abbr.replace('\"', "");

                Identity {
                    season_id: SeasonId::from(szn),
                    player_id: None,
                    team_id: TeamId(tid),
                    team_abbr: TeamAbbreviation(tab),
                    game_id: GameId::from(gid),
                    game_date: GameDate(date),
                }
            }
            _ => panic!(
                "💀 unrecognized schema. could not extract an identity from the box score string. "
            ),
        }
    }
}

impl Identifiable for Value {
    fn identity(&self) -> Identity {
        // eprintln!("raw identity data: {:?}", self);

        match self.as_array().unwrap().as_slice() {
            //player game case
            [
                Value::String(szn),
                Number(player_id),
                _,
                Number(team_id),
                Value::String(team_abbr),
                _,
                Value::String(game_id),
                Value::String(game_date),
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ] => Identity {
                season_id: SeasonId::from(
                    szn.parse::<i32>()
                        .expect("💀 expect szn column to be an i64 (i32)"),
                ),
                player_id: Some(PlayerId(
                    player_id.as_u64().expect("💀 expect player id to be a u64"),
                )),
                team_id: TeamId(team_id.as_u64().expect("💀 expect team id to be a u64")),
                team_abbr: TeamAbbreviation(team_abbr.to_string()),
                game_id: GameId::from(game_id.to_owned()),
                game_date: GameDate(
                    NaiveDate::parse_from_str(&*game_date, "%Y-%m-%d")
                        .expect("💀 failed to parse game date as an chrono::NaiveDate. "),
                ),
            },
            [
                szn,
                Number(team_id),
                Value::String(team_abbr),
                _,
                Value::String(game_id),
                Value::String(game_date),
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ] => Identity {
                season_id: SeasonId::try_from(szn)
                    .expect("💀 couldnt parse season_id from JSON value"),
                player_id: None,
                team_id: TeamId(team_id.as_u64().expect("💀 expect team id to be a u64")),
                team_abbr: TeamAbbreviation(team_abbr.to_string()),
                game_date: GameDate(
                    NaiveDate::parse_from_str(&*game_date, "%Y-%m-%d")
                        .expect("💀 failed to parse game date as an chrono::NaiveDate. "),
                ),
                game_id: GameId::from(game_id.to_owned()),
            },
            _ => panic!("💀 unrecognized schema. could not match to a player or team stat"),
        }
    }
}
