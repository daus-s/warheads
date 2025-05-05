use crate::stats::stat_column::StatColumn::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)] // Required for #[derivative(PartialEq)]
use std::cmp::{Ordering, PartialEq};
use std::fmt::{Display, Formatter};

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
    pub fn to_str(&self) -> &'static str {
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
impl Display for StatColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
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

impl PartialOrd<Self> for StatColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StatColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = stat_column_ord(self);
        let b = stat_column_ord(other);

        a.cmp(&b)
    }
}
fn stat_column_ord(col: &StatColumn) -> usize {
    let ord = vec![
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

    match ord.iter().position(|x| x == col) {
        Some(i) => i,
        None => panic!("indexed with non existent stat column"), /* this is a good panic because
                                                                 // if u call a bad index what do
                                                                 // you even mean its not real bro
                                                                 // most goated comment notation
                                                                  */
    }
}
