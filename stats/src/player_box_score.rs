use std::fmt::Formatter;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::Date;
use crate::nba::GameResult;
use crate::statify::Statify;
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
    min: Option<u32>,
    fgm: Option<u32>,
    fga: Option<u32>,
    fg3m: Option<u32>,
    fg3a: Option<u32>,
    ftm: Option<u32>,
    fta: Option<u32>,
    oreb: Option<u32>,
    dreb: Option<u32>,
    reb: Option<u32>,
    ast: Option<u32>,
    stl: Option<u32>,
    blk: Option<u32>,
    tov: Option<u32>,
    pf: Option<u32>, //personal fouls
    pts: Option<u32>,
    plus_minus: Option<i32>,
    fantasy_pts: Option<f32>,
    elo: i32, // decisions, decisions
}

impl std::fmt::Display for PlayerBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\npts:{}\tfgs:{}/{}\t3ps:{}/{}\tft: {}/{}\nast:{}\nreb:{} (off {},def {})\nstl: {}\t blk:{}\ttov:{}\nfantasy: {}\n",
               self.player_name.to_ascii_uppercase(), self.pts.unwrap_f("-"), self.fgm.unwrap_f("-"), self.fga.unwrap_f("-"), self.fg3m.unwrap_f("-"), self.fg3a.unwrap_f("-"), self.ftm.unwrap_f("-"), self.fta.unwrap_f("-"), self.ast.unwrap_f("-"), self.reb.unwrap_f("-"), self.oreb.unwrap_f("-"), self.dreb.unwrap_f("-"), self.stl.unwrap_f("-"), self.blk.unwrap_f("-"), self.tov.unwrap_f("-"), self.fantasy_pts.unwrap_f("-"))
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


