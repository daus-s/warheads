use time::Date;
use std::fmt::Formatter;
use derive_builder::Builder;
use crate::format::{format_matchup, opponent, percent, season_fmt};
use serde::{Deserialize, Serialize};
use crate::nba::{GameResult, Visiting};
use crate::nba::Visiting::{Away, Home};
use crate::player_box_score::PlayerBoxScore;
use crate::statify::Statify;

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct TeamBoxScore {
    roster: Vec<PlayerBoxScore>,
    team_abbreviation: String,
    team_name: String,
    game_date: Date,
    matchup: String,
    wl: GameResult,
    team_id: u64,
    game_id: u64,
    season_id: u32,
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
}

impl TeamBoxScore {
    pub fn game_id(&self) -> u64 {
        self.game_id
    }

    pub fn add_player_stats(&mut self, value: PlayerBoxScore)  {
        self.roster.push(value);
    }

    pub fn home_or_away(&self) -> Visiting {
        let team = &self.team_abbreviation.trim();

        let matchup = self.matchup.clone();

        match matchup.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [home, "vs.", away] => if home == team { Home } else if away == team { Away } else { panic!("team {} is neither of the contestants {} @ {}. ", self.team_abbreviation, away, home)},
            [away, "@", home] => if home == team { Home } else if away == team { Away } else { panic!("team {} is neither of the contestants {} @ {}. ", self.team_abbreviation, away, home)},
            e => panic!("could not parse game format. {:#?}", e),
        }
    }

    pub fn team(&self) -> String {

        self.team_abbreviation.clone()

    }

    pub fn season_str(&self) -> String {

        season_fmt((self.season_id - 20000) as i32)

    }
}

impl std::fmt::Display for TeamBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}\n{} {} against {}.\npts: {}\tfg: {}/{} {}\t3pt: {}/{} ({:.1}%)\tft: {}/{} ({:.1}%)\nreb: {}\toff: {}\tdef: {}\nblocks: {}\t steals: {}\nfouls: {}\t turnovers: {}\n",
               format_matchup(&self.matchup),
               self.game_date, self.team_abbreviation,
               match self.wl {
                   GameResult::Win => "win",
                   GameResult::Loss => "loss",
                   GameResult::Draw => panic!("nba games cannot end in a tie")
               },
               opponent(&self.matchup, &self.team_abbreviation),
               self.pts.unwrap_f("-"),
               self.fgm.unwrap_f("-"), self.fga.unwrap_f("-"), percent(self.fgm, self.fga),
               self.fg3m.unwrap_f("-"), self.fg3a.unwrap_f("-"), percent(self.fg3m, self.fg3a),
               self.ftm.unwrap_f("-"), self.fta.unwrap_f("-"), percent(self.ftm, self.fta),
               self.reb.unwrap_f("-"), self.oreb.unwrap_f("-"), self.dreb.unwrap_f("-"),
               self.blk.unwrap_f("-"), self.stl.unwrap_f("-"),
               self.pf.unwrap_f("-"), self.tov.unwrap_f("-")
        )
    }
}
