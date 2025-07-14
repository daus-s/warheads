use crate::format::language::Columnizable;
use crate::stats::box_score::BoxScore;
use crate::stats::domain::Domain;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::types::{GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};
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
    ///
    /// season_id: SeasonId
    ///
    /// contains the information for both the season period (regular season, pre-season etc.) and
    /// the calendar year of the start of the season
    ///
    /// Ex:
    ///
    /// (1946-47 season -> 1946)
    ///
    /// based on what period of the season the game is we add the season period offset
    ///
    /// *Regular season offset => 20000* for more info on this see the SeasonPeriod module
    ///```
    /// use warheads::stats::season_period::SeasonPeriod::RegularSeason;
    /// use warheads::types::SeasonId;
    ///
    /// let year = 1946;
    ///
    /// let season_id = 1946 + 20000;
    ///
    /// let s_id = SeasonId::from(21946);
    ///
    /// assert_eq!(SeasonId::from((1946, RegularSeason)), s_id)
    ///```
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
            [season_id, player_id, _player_name, team_id, team_abbr, _team_name, game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _fantasy_pts, _video_available] =>
            {
                let szn = season_id.replace('"', "").parse::<i32>().expect("💀 failed to parse season id as an i32 (pre-conversion) while identifying a player game. player");

                let pid = player_id.replace('\"', "").parse::<u64>().expect(
                    "💀 failed to parse player id as an u64 while identifying a player game. ",
                );

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
                }
            }
            [season_id, team_id, team_abbr, _team_name, game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _video_available] =>
            {
                let szn = season_id.replace('\"', "").parse::<i32>().expect("💀 failed to parse season id as an u64 (pre-conversion) while identifying a team game. ");

                let tid = team_id
                    .replace('\"', "")
                    .parse::<u64>()
                    .expect("💀 failed to parse team id as an u64 while identifying a team game. ");

                let gid = game_id.replace('\"', "");

                let tab = team_abbr.replace('\"', "");

                Identity {
                    season_id: SeasonId::from(szn),
                    player_id: None,
                    team_id: TeamId(tid),
                    team_abbr: TeamAbbreviation(tab),
                    game_id: GameId::from(gid),
                }
            }
            _ => panic!(
                "💀 unrecognized schema. could not extract an identity from the box score string. "
            ),
        }
    }
}

impl<T: BoxScore> Identifiable for T {
    fn identity(&self) -> Identity {
        Identity {
            season_id: self.season(),
            player_id: self.player_id(),
            game_id: self.game_id().clone(),
            team_abbr: self.team_abbr().clone(),
            team_id: self.team_id(),
        }
    }
}

impl Identifiable for Value {
    fn identity(&self) -> Identity {
        // eprintln!("raw identity data: {:?}", self);

        match self.as_array().unwrap().as_slice() {
            //player game case
            [Value::String(szn), Number(player_id), _, Number(team_id), Value::String(team_abbr), _, Value::String(game_id), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => {
                Identity {
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
                }
            }
            [szn, Number(team_id), Value::String(team_abbr), _, Value::String(game_id), _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => {
                Identity {
                    season_id: SeasonId::try_from(szn)
                        .expect("💀 couldnt parse season_id from JSON value"),
                    player_id: None,
                    team_id: TeamId(team_id.as_u64().expect("💀 expect team id to be a u64")),
                    team_abbr: TeamAbbreviation(team_abbr.to_string()),
                    game_id: GameId::from(game_id.to_owned()),
                }
            }
            _ => panic!("💀 unrecognized schema. could not match to a player or team stat"),
        }
    }
}
