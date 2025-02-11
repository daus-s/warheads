use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use crate::stat_column::StatColumn::*;

#[derive(Serialize, Deserialize, Derivative, Eq)]
#[derivative(PartialEq, Hash, Clone)]
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

const COLUMNS: [StatColumn; 32] = [
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

pub fn column_index(stat: &StatColumn) -> Option<usize> {
    COLUMNS.iter().position(|x| x == stat)
}

impl StatColumn {
    pub fn column_fmt(&self, s: String) -> String {
        match self {
            SEASON_ID => format!("\"{}\"", s),
            PLAYER_ID => format!("{}", s),
            PLAYER_NAME => format!("\"{}\"", s),
            TEAM_ID => format!("{}", s),
            TEAM_ABBREVIATION => format!("\"{}\"", s),
            TEAM_NAME => format!("\"{}\"", s),
            GAME_ID => format!("\"{}\"", s),
            GAME_DATE => format!("\"{}\"", s),
            MATCHUP => format!("\"{}\"", s),
            WL => format!("\"{}\"", s),
            MIN => format!("{}", s),
            FGM => format!("{}", s),
            FGA => format!("{}", s),
            FG_PCT => format!("{}", s),
            FG3M => format!("{}", s),
            FG3A => format!("{}", s),
            FG3_PCT => format!("{}", s),
            FTM => format!("{}", s),
            FTA => format!("{}", s),
            FT_PCT => format!("{}", s),
            OREB => format!("{}", s),
            DREB => format!("{}", s),
            REB => format!("{}", s),
            AST => format!("{}", s),
            STL => format!("{}", s),
            BLK => format!("{}", s),
            TOV => format!("{}", s),
            PF => format!("{}", s),
            PTS => format!("{}", s),
            PLUS_MINUS => format!("{}", s),
            FANTASY_PTS => format!("{}", s),
            VIDEO_AVAILABLE => format!("{}", s),
        }
    }
}


#[derive(Serialize, Deserialize, Eq, PartialEq, Derivative)]
#[derivative(Hash)]
pub struct StatEntry {
    col: StatColumn,
    val: String,
}

impl StatEntry {
    pub fn new(col: StatColumn, val: String) -> StatEntry {
        StatEntry {
            col,
            val
        }
    }

    pub fn col(&self) -> StatColumn {
        self.col.clone()
    }

    pub fn val(&self) -> String {
        format!("{:#}", self)
    }
}

impl Display for StatEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.col.column_fmt(self.val.clone()))
    }
}