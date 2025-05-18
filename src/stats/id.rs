use std::fmt::{Debug, Formatter};
use serde_json::Value;
use serde_json::Value::{Number};
use crate::stats::box_score::BoxScore;
use crate::format::language::{Columnizable};

pub trait Identifiable {
    fn identity(&self) -> Identity;
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Identity {
    ///
    /// year: i32 is the year,
    /// from the original data source we subtract 20000
    ///
    /// Ex:
    ///
    ///     (1946-47 season -> 1946)
    ///
    pub year: i32,

    ///
    /// this field is the player_id of the identifiable object.
    /// in the case of team objects it is `None`. However,
    /// in the case of identifying a player object it
    /// is wrapped in `Some(u64)`
    ///
    pub player_id: Option<u64>,

    pub team_id: u64,

    pub team_abbr: String, // len == 3

    pub game_id: String,
}

impl Debug for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.player_id {
            Some(id) => write!(f, "player_id: {}\nteam: {}\nteam_id: {}\nyear: {}\ngame: {}", id, self.team_abbr, self.team_id, self.year, self.game_id),
            None => write!(f, "team: {}\nteam_id: {}\nyear: {}\ngame: {}", self.team_abbr, self.team_id, self.year, self.game_id)
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
        let mut columns = self.columns();

        match columns.as_slice() {
            [season_id, player_id, _player_name, team_id, team_abbr, _team_name, game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _fantasy_pts, _video_available] =>
            {
                let szn = season_id.replace('"', "").parse::<i32>().expect("failed to parse season id as an i32 (pre-conversion) while identifying a player game. player") - 20000;

                let pid = player_id.replace('\"', "").parse::<u64>().expect("failed to parse player id as an u64 while identifying a player game. ");

                let tid = team_id.replace('\"', "").parse::<u64>().expect("failed to parse team id as an u64 while identifying a player game. ");

                let gid = game_id.replace('\"', "");

                let tab = team_abbr.replace('\"', "");

                Identity {
                    year: szn,
                    player_id: Some(pid),
                    team_id: tid,
                    team_abbr: tab,
                    game_id: gid,
                }
            }
            [season_id, team_id, team_abbr, _team_name, game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _video_available] =>
            {
                let szn = season_id.replace('\"', "").parse::<i32>().expect("failed to parse season id as an u64 (pre-conversion) while identifying a team game. ");

                let tid = team_id.replace('\"', "").parse::<u64>().expect("failed to parse team id as an u64 while identifying a team game. ");

                let gid = game_id.replace('\"', "");

                let tab = team_abbr.replace('\"', "");


                Identity {
                    year: szn,
                    player_id: None,
                    team_id: tid,
                    team_abbr: tab,
                    game_id: gid,
                }
            }
            _ => panic!("unrecognized game schema"),
        }
    }
}

impl<T: BoxScore> Identifiable for T {
    fn identity(&self) -> Identity {
        Identity {
            year: self.season(),
            player_id: self.player_id(),
            game_id: self.game_id().parse().unwrap(),
            team_abbr: self.team_abbr(),
            team_id: self.team_id(),
        }
    }
}


impl Identifiable for Value {
    fn identity(&self) -> Identity {
        // eprintln!("raw identity data: {:?}", self);

        match self.as_array().unwrap().as_slice() {
            //player game case
            [
                Value::String(szn), Number(player_id), _, Number(team_id),
                Value::String(team_abbr), _, Value::String(game_id), _, _, _, _, _,
                _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _
            ] => {
                Identity {
                    year: szn.parse::<i32>().expect("expect szn column to be an i64 (i32)") as i32 - 20000,
                    player_id: Some(player_id.as_u64().expect("expect player id to be a u64")),
                    team_id: team_id.as_u64().expect("expect team id to be a u64"),
                    team_abbr: team_abbr.to_string(),
                    game_id: game_id.to_string(),
                }
            },
            [
                Value::String(szn), Number(team_id), Value::String(team_abbr), _,
                Value::String(game_id), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _,
                _, _, _, _, _, _
            ] => {
                Identity {
                    year: szn.parse::<i32>().expect("expect szn column to be an i64 (i32)") as i32 - 20000,
                    player_id: None,
                    team_id: team_id.as_u64().expect("expect team id to be a u64"),
                    team_abbr: team_abbr.to_string(),
                    game_id: game_id.to_string(),
                }
            },
            _ => panic!("row length is unrecognized. not a player or team stat"),
        }
    }
}