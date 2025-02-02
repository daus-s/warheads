use time::Date;
use std::fmt::Formatter;
use derive_builder::Builder;
use crate::format::{format_matchup, opponent};
use serde::{Deserialize, Serialize};
use crate::nba::{GameResult, Visiting};
use crate::nba::Visiting::{Away, Home};
use crate::player_box_score::PlayerBoxScore;

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct TeamBoxScore { season_id: u32,
    roster: Vec<PlayerBoxScore>,
    team_abbreviation: String,
    team_name: String,
    game_date: Date,
    matchup: String,
    wl: GameResult,
    team_id: u64,
    game_id: u64,
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

impl TeamBoxScore {
    pub fn game_id(&self) -> u64 {
        self.game_id
    }

    pub fn add_player_stats(&mut self, value: PlayerBoxScore)  {
        self.roster.push(value);
    }

    pub fn home_or_away(&self) -> Visiting {
        let team = &self.team_abbreviation;

        let matchup = self.matchup.clone();

        match matchup.split(" ").collect::<Vec<&str>>().as_slice() {
            [home, "vs.", away] => if home == team { Home } else if away == team { Away } else { panic!("team is neither of the contestants. ")},
            [away, "@", home] => if home == team { Home } else if away == team { Away } else { panic!("team is neither of the contestants. ")},
            _ => panic!("could not parse game format."),
        }
    }

    pub fn team(&self) -> String {
        self.team_abbreviation.clone()
    }
}

impl std::fmt::Display for TeamBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}\n{} {} against {}.\npts: {}\tfg: {}/{} ({:.1}%)\t3pt: {}/{} ({:.1}%)\tft: {}/{} ({:.1}%)\nreb: {}\toff: {}\tdef: {}\nblocks: {}\t steals: {}\nfouls: {}\t turnovers: {}\n",
               format_matchup(&self.matchup),
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
               self.blk, self.stl,
               self.pf, self.tov
        )
    }
}