use crate::stats::stat_column::StatColumn::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // Required for #[derivative(PartialEq)]
use std::cmp::{Ordering, PartialEq};
use std::fmt::{Debug, Display, Formatter};

#[derive(Serialize, Deserialize, Derivative, Eq)]
#[derivative(PartialEq, Hash, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum StatColumn {
    SEASON_ID,
    PLAYER_ID,
    PLAYER_NAME,
    TEAM_ID,
    TEAM_ABBREVIATION,
    TEAM_NAME,
    GAME_ID,
    GAME_DATE,
    MATCHUP,
    WL,
    MIN,
    FGM,
    FGA,
    FG_PCT,
    FG3M,
    FG3A,
    FG3_PCT,
    FTM,
    FTA,
    FT_PCT,
    OREB,
    DREB,
    REB,
    AST,
    STL,
    BLK,
    TOV,
    PF,
    PTS,
    PLUS_MINUS,
    FANTASY_PTS,
    VIDEO_AVAILABLE,
}

impl StatColumn {
    pub fn column_name(&self) -> &'static str {
        match self {
            SEASON_ID => "season_id",
            PLAYER_ID => "player_id",
            PLAYER_NAME => "player_name",
            TEAM_ID => "team_id",
            TEAM_ABBREVIATION => "team_abbreviation",
            TEAM_NAME => "team_name",
            GAME_ID => "game_id",
            GAME_DATE => "game_date",
            MATCHUP => "matchup",
            WL => "wl",
            MIN => "min",
            FGM => "fgm",
            FGA => "fga",
            FG_PCT => "fg_pct",
            FG3M => "fg3m",
            FG3A => "fg3a",
            FG3_PCT => "fg3_pct",
            FTM => "ftm",
            FTA => "fta",
            FT_PCT => "ft_pct",
            OREB => "oreb",
            DREB => "dreb",
            REB => "reb",
            AST => "ast",
            STL => "stl",
            BLK => "blk",
            TOV => "tov",
            PF => "pf",
            PTS => "pts",
            PLUS_MINUS => "plus_minus",
            FANTASY_PTS => "fantasy_pts",
            VIDEO_AVAILABLE => "video_available",
        }
    }
}

impl From<String> for StatColumn {
    fn from(value: String) -> Self {
        let s = value.replace("\"", "");

        match s.as_str() {
            "SEASON_ID" => SEASON_ID,
            "PLAYER_ID" => PLAYER_ID,
            "PLAYER_NAME" => PLAYER_NAME,
            "TEAM_ID" => TEAM_ID,
            "TEAM_ABBREVIATION" => TEAM_ABBREVIATION,
            "TEAM_NAME" => TEAM_NAME,
            "GAME_ID" => GAME_ID,
            "GAME_DATE" => GAME_DATE,
            "MATCHUP" => MATCHUP,
            "WL" => WL,
            "MIN" => MIN,
            "FGM" => FGM,
            "FGA" => FGA,
            "FG_PCT" => FG_PCT,
            "FG3M" => FG3M,
            "FG3A" => FG3A,
            "FG3_PCT" => FG3_PCT,
            "FTM" => FTM,
            "FTA" => FTA,
            "FT_PCT" => FT_PCT,
            "OREB" => OREB,
            "DREB" => DREB,
            "REB" => REB,
            "AST" => AST,
            "STL" => STL,
            "BLK" => BLK,
            "TOV" => TOV,
            "PF" => PF,
            "PTS" => PTS,
            "PLUS_MINUS" => PLUS_MINUS,
            "FANTASY_PTS" => FANTASY_PTS,
            "VIDEO_AVAILABLE" => VIDEO_AVAILABLE,
            s => panic!("💀 unrecognized stat column name: {s}"),
        }
    }
}

impl Display for StatColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.column_name())
    }
}

//excluded stat columns: ELO, not in the original data set
const PLAYER_COLUMNS: [StatColumn; 32] = [
    SEASON_ID,
    PLAYER_ID,
    PLAYER_NAME,
    TEAM_ID,
    TEAM_ABBREVIATION,
    TEAM_NAME,
    GAME_ID,
    GAME_DATE,
    MATCHUP,
    WL,
    MIN,
    FGM,
    FGA,
    FG_PCT,
    FG3M,
    FG3A,
    FG3_PCT,
    FTM,
    FTA,
    FT_PCT,
    OREB,
    DREB,
    REB,
    AST,
    STL,
    BLK,
    TOV,
    PF,
    PTS,
    PLUS_MINUS,
    FANTASY_PTS,
    VIDEO_AVAILABLE,
];

pub fn player_column_index(stat: &StatColumn) -> Option<usize> {
    PLAYER_COLUMNS.iter().position(|x| x == stat)
}

const TEAM_COLUMNS: [StatColumn; 29] = [
    SEASON_ID,
    TEAM_ID,
    TEAM_ABBREVIATION,
    TEAM_NAME,
    GAME_ID,
    GAME_DATE,
    MATCHUP,
    WL,
    MIN,
    FGM,
    FGA,
    FG_PCT,
    FG3M,
    FG3A,
    FG3_PCT,
    FTM,
    FTA,
    FT_PCT,
    OREB,
    DREB,
    REB,
    AST,
    STL,
    BLK,
    TOV,
    PF,
    PTS,
    PLUS_MINUS,
    VIDEO_AVAILABLE,
];

pub fn team_column_index(stat: &StatColumn) -> Option<usize> {
    TEAM_COLUMNS.iter().position(|x| x == stat)
}

impl PartialOrd<Self> for StatColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StatColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = player_column_index(self).unwrap();
        /* this is a good panic because if u call a bad index
          what do you even mean its not real bro
        */
        let b = player_column_index(other).unwrap();

        a.cmp(&b)
    }
}

impl Debug for StatColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.column_name())
    }
}
