use std::fmt::Formatter;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::{Date};
use crate::format::{format_matchup, opponent};

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

impl std::fmt::Display for PlayerBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\npts:{}\tfgs:{}/{}\t3ps:{}/{}\tft: {}/{}\nast:{}\nreb:{} (off {},def {})\nstl: {}\t blk:{}\ttov:{}\nfantasy: {}\n",
               self.player_name.to_ascii_uppercase(), self.pts, self.fgm, self.fga, self.fg3m, self.fg3a, self.ftm, self.fta, self.ast, self.reb, self.oreb, self.dreb, self.stl, self.blk, self.tov, self.fantasy_pts)
    }
}

#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct TeamBoxScore {
    season_id: u32,
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
}


impl std::fmt::Display for TeamBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}\n{} {} against {}.\npts: {}\tfg: {}/{} ({:.1}%)\t3pt: {}/{} ({:.1}%)\tft: {}/{} ({:.1}%)\nreb: {}\toff: {}\tdef: {}\nblocks: {}\t steals: {}\np. fouls: {}\t turnovers: {}\n",
               format_matchup(&self.matchup, &self.team_abbreviation),
               self.game_date, self.team_abbreviation,
               match self.wl {
                    GameResult::Win => "win",
                    GameResult::Loss => "loss",
                    GameResult::Draw => panic!("nba games cannot end in a tie")
               },
               opponent(&self.matchup, &self.team_abbreviation),
               self.pts,
               self.fgm, self.fga, (self.fgm as f32 * 100.0) / (self.fga as f32),
               self.fg3m, self.fg3a, (self.fg3m as f32 * 100.0) / (self.fg3a as f32),
               self.ftm, self.fta, (self.ftm as f32 * 100.0) / (self.fta as f32),
               self.reb, self.oreb, self.dreb,
               self.blk, self.stl, self.pf , self.tov
        )
    }
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameResult {
    Win,
    Loss,
    Draw
}