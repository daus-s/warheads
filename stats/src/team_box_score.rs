use crate::nba::Visiting::{Away, Home};
use crate::nba::{Visiting};
use crate::player_box_score::PlayerBoxScore;
use crate::statify::Statify;
use derive_builder::Builder;
use format::season::season_fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use chrono::NaiveDate;
use format::matchup::{format_matchup, opponent};
use crate::box_score::BoxScore;
use crate::percent::percent;
use crate::types::{GameResult, MatchupString};

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct TeamBoxScore {

    // team identification

    team_abbreviation: String,
    team_name: String,
    team_id: u64,

    // game data

    season_id: i32,
    matchup: MatchupString,
    game_date: NaiveDate,
    game_id: String,

    //roster

    roster: Vec<PlayerBoxScore>,

    // classic box score

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


    //advanced stats

    plus_minus: Option<i32>,
}

impl TeamBoxScore {

    // not sure what type to return
    pub fn game_id(&self) -> String {
        self.game_id.clone()
    }

    pub fn add_player_stats(&mut self, value: PlayerBoxScore) {
        self.roster.push(value);
    }

    pub fn home_or_away(&self) -> Visiting {
        let team = &self.team_abbreviation.trim();

        let MatchupString(matchup) = self.matchup.clone();

        match matchup.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [home, "vs.", away] => {
                if home == team {
                    Home
                } else if away == team {
                    Away
                } else {
                    panic!(
                        "team {} is neither of the contestants {} @ {}. ",
                        self.team_abbreviation, away, home
                    )
                }
            }
            [away, "@", home] => {
                if home == team {
                    Home
                } else if away == team {
                    Away
                } else {
                    panic!(
                        "team {} is neither of the contestants {} @ {}. ",
                        self.team_abbreviation, away, home
                    )
                }
            }
            e => panic!("could not parse game format. {:#?}", e),
        }
    }

    pub fn team(&self) -> String {
        self.team_abbreviation.clone()
    }

    pub fn season_str(&self) -> String {
        season_fmt(self.season_id - 20000)
    }

    pub fn elo(&self) -> i32 {
        todo!()
    }


}

impl BoxScore for TeamBoxScore {
    fn season(&self) -> i32 {
        self.season_id
    }

    fn game_id(&self) -> String {
        self.game_id.clone()
    }

    fn player_id(&self) -> Option<u64> {
        None
    }

    fn team_id(&self) -> u64 {
        self.team_id
    }

    fn team_abbr(&self) -> String {
        self.team_abbreviation.clone()
    }
}

impl std::fmt::Display for TeamBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}\n{} {} against {}.\npts: {}\tfg: {}/{} {}\t3pt: {}/{} ({:.1}%)\tft: {}/{} ({:.1}%)\nreb: {}\toff: {}\tdef: {}\nblocks: {}\t steals: {}\nfouls: {}\t turnovers: {}\n",
               format_matchup(self.matchup.str()),
               self.game_date, self.team_abbreviation,
               match self.wl {
                   GameResult::Win => "win",
                   GameResult::Loss => "loss",
                   GameResult::Draw => panic!("nba games cannot end in a tie")
               },
               opponent(self.matchup.str(), &self.team_abbreviation),
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



