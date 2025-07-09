use crate::format::season::season_fmt;
use crate::stats::box_score::BoxScore;
use crate::stats::domain::Domain;
use crate::stats::nba_kind::NBAStatKind::Team;
use crate::stats::percent::percent_string;
use crate::dapi::player_box_score::PlayerBoxScore;
use crate::stats::shooting::{Attempts, Makes};
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;
use crate::stats::visiting::Visiting;
use crate::types::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct TeamBoxScore {
    // team identification
    team_abbreviation: TeamAbbreviation,
    team_name: TeamName,
    team_id: TeamId,

    // game data
    season_id: SeasonId,
    matchup: MatchupString,
    game_date: GameDate,
    game_id: GameId,

    //roster
    roster: Vec<PlayerBoxScore>,

    // classic box score
    wl: GameResult,

    min: Minutes,
    fgm: FieldGoalMakes,
    fga: FieldGoalAttempts,

    fg3m: ThreePointMakes,
    fg3a: ThreePointAttempts,

    ftm: FreeThrowMakes,
    fta: FreeThrowAttempts,

    oreb: OffensiveRebounds,
    dreb: DefensiveRebounds,
    reb: Rebounds,

    ast: Assists,

    stl: Steals,

    blk: Blocks,

    tov: Turnovers,

    pf: PersonalFouls, //personal fouls
    pts: Points,

    //advanced stats
    plus_minus: PlusMinus,
}

impl TeamBoxScore {
    // not sure what type to return
    pub fn game_id(&self) -> GameId {
        self.game_id.clone()
    }

    pub fn add_player_stats(&mut self, value: PlayerBoxScore) {
        self.roster.push(value);
    }

    pub fn team_abbr(&self) -> TeamAbbreviation {
        self.team_abbreviation.clone()
    }

    pub fn season_str(&self) -> String {
        season_fmt(self.season_id.year())
    }

    pub fn elo(&self) -> i32 {
        let mut sum = 0;

        for player in &self.roster {
            sum += player.elo();
        }

        sum
    }


    pub fn result(&self) -> GameResult {
        self.wl
    }
    pub fn domain(&self) -> Domain {
        (self.season_id, Team)
    }
}

impl BoxScore for TeamBoxScore {
    fn season(&self) -> SeasonId {
        self.season_id
    }

    fn game_id(&self) -> &GameId {
        &self.game_id
    }

    fn player_id(&self) -> Option<PlayerId> {
        None
    }

    fn team_id(&self) -> TeamId {
        self.team_id
    }

    fn team_abbr(&self) -> &TeamAbbreviation {
        &self.team_abbreviation
    }

    fn home_or_away(&self) -> Visiting {
        self.matchup.home_or_away()
    }

    fn set(&mut self, col: &StatColumn, val: &StatValue) {
        todo!()
    }
}

impl std::fmt::Display for TeamBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}\n{} {} against {}.\npts: {}\tfg: {}/{} {}\t3pt: {}/{} ({:.1}%)\tft: {}/{} ({:.1}%)\nreb: {}\toff: {}\tdef: {}\nblocks: {}\t steals: {}\nfouls: {}\t turnovers: {}\n",
               self.matchup,
               self.game_date, self.team_abbreviation,
               match self.wl {
                   GameResult::Win => "win",
                   GameResult::Loss => "loss",
                   GameResult::Draw => panic!("nba games cannot end in a tie")
               },
               match self.matchup.opponent(&self.team_abbreviation)
               {
                   Ok(t) => t.to_string(),
                   Err(e) => e,
               },
               self.pts,
               self.fgm, self.fga, percent_string(self.fgm.makes() as i32, (self.fga.attempts().unwrap_or_else(|| 0)) as i32),
               self.fg3m, self.fg3a, percent_string(self.fg3m.makes() as i32, (self.fg3a.attempts().unwrap_or_else(|| 0)) as i32),
               self.ftm, self.fta, percent_string(self.ftm.makes() as i32, (self.fta.attempts().unwrap_or_else(|| 0)) as i32),
               self.reb, self.oreb, self.dreb,
               self.blk, self.stl,
               self.pf, self.tov
        )
    }
}
