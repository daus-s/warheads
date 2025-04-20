use crate::box_score::BoxScore;
use format::language::columns;

pub trait Identifiable {
    fn identity(&self) -> Option<Identity>;
}

#[derive(Eq, PartialEq)]
pub struct Identity {
    ///
    /// season: i32 is the year,
    /// from the original data source we subtract 20000
    ///
    /// Ex:
    ///
    ///     (1946-47 season -> 1946)
    ///
    pub szn: i32,

    pub player_id: Option<u64>,

    pub team_id: u64,

    pub team_abbr: String, // len == 3

    pub game_id: String,
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

///
/// this function should only ever accept well formatted strings so it will panic if not passed well.
/// thus it does not return a result but only a boolean
///
/// more importantly this function is asked "is this the game that i correct?"
/// so we answer true or false
///
impl Identifiable for String {
    fn identity(&self) -> Option<Identity> {
        let columns = columns(self.clone());

        match columns.as_slice() {
            [season_id, player_id, _player_name, team_id, team_abbr, _team_name, game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _fantasy_pts, _video_available] =>
            {
                let szn = season_id.replace('"', "").parse::<i32>().ok()? - 20000;

                let pid = player_id.parse::<u64>().ok()?;

                let tid = team_id.replace('\"', "").parse::<u64>().ok()?;

                let gid = game_id.replace('\"', "");

                Some(Identity {
                    szn,
                    player_id: Some(pid),
                    team_id: tid,
                    team_abbr: team_abbr.to_string(),
                    game_id: gid,
                })
            }
            [season_id, team_id, team_abbr, _team_name, game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _video_available] =>
            {
                let szn = season_id.replace('\"', "").parse::<i32>().ok()?;

                let tid = team_id.replace('\"', "").parse::<u64>().ok()?;

                let gid = game_id.replace('\"', "");

                Some(Identity {
                    szn,
                    player_id: None,
                    team_id: tid,
                    team_abbr: team_abbr.to_string(),
                    game_id: gid,
                })
            }
            _ => None,
        }
    }
}

impl<T: BoxScore> Identifiable for T {
    fn identity(&self) -> Option<Identity> {
        Some(Identity {
            szn: self.season(),
            player_id: self.player_id(),
            game_id: self.game_id().parse().unwrap(),
            team_abbr: self.team_abbr(),
            team_id: self.team_id(),
        })
    }
}
