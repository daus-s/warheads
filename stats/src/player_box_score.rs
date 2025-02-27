use std::fmt::Formatter;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::Date;
use crate::nba::GameResult;
use crate::team_box_score::TeamBoxScore;

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
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
    elo: i32, // decisions, decisions
}

impl std::fmt::Display for PlayerBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\npts:{}\tfgs:{}/{}\t3ps:{}/{}\tft: {}/{}\nast:{}\nreb:{} (off {},def {})\nstl: {}\t blk:{}\ttov:{}\nfantasy: {}\n",
               self.player_name.to_ascii_uppercase(), self.pts, self.fgm, self.fga, self.fg3m, self.fg3a, self.ftm, self.fta, self.ast, self.reb, self.oreb, self.dreb, self.stl, self.blk, self.tov, self.fantasy_pts)
    }
}

impl PlayerBoxScore {
    pub fn game_id(&self) -> u64 {
        self.game_id
    }

    pub fn team(&self) -> String {
        self.team_abbreviation.clone()
    }

    pub fn played_in(&self, game: TeamBoxScore) -> bool {
        self.game_id == game.game_id() && self.team_abbreviation == game.team()
    }
}


