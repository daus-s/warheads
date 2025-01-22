use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::{Date};

#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct PlayerBoxScore {
    season_id: u32,
    player_id: u64,
    player_name: String,
    team_id: u64,
    team_abbreviation: String,
    team_name: String,
    game_id: u64,
    game_date: Date,
    matchup: String,
    wl: GameResult,
    min: u32,
    fgm: u32,
    fga: u32,
    fg3m: u32,
    fg3a: u32,
    ftm: u32,
    fta: u32,
    oreb: u32,
    dreb: u32,
    reb: u32,
    ast: u32,
    stl: u32,
    blk: u32,
    tov: u32,
    pf: u32, //personal fouls
    pts: u32,
    plus_minus: i32,
    fantasy_pts: f32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameResult {
    Win,
    Loss,
    Draw
}