use crate::corrections::correction_builder::CorrectionBuilder;
use crate::dapi::from_value::FromValue;
use crate::format::language::Columnizable;
use crate::format::path_manager::{
    nba_correction_dir, nba_player_correction_file, nba_team_correction_file,
};
use crate::stats::visiting::Visiting;

use serde::{Deserialize, Serialize};

use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::SeasonPeriod;
use crate::stats::stat_column::{player_column_index, team_column_index, StatColumn};

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::stats::box_score::{BoxScore, BoxScoreBuilder};
use crate::stats::nba_kind::NBAStatKind::{LineUp, Player, Team};
use crate::stats::statify::StatPair;
use crate::types::{GameDate, GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Correction {
    pub game_id: GameId,

    pub game_date: GameDate,

    pub season: SeasonId,

    pub player_id: Option<PlayerId>,

    pub team_id: TeamId,

    pub team_abbr: TeamAbbreviation,

    pub kind: NBAStatKind,

    pub period: SeasonPeriod,

    pub delete: bool,

    pub corrections: HashMap<StatColumn, Value>,
}

impl Correction {
    pub fn load(filename: &str) -> Result<Correction, String> {
        let path = Path::new(filename);

        // Read the file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;

        // Deserialize the content into a Correction struct
        let correction: Correction = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize file {}: {}", filename, e))?;

        //todo: assert eq the path info and the file content

        Ok(correction)
    }

    /// Saves the correction to the file:
    /// `corrections/{season_file}/{NBAStatType}/{}.json`
    ///
    /// This is a private function and is called when `fn create ->` is completed.
    ///
    pub fn save(&self) -> io::Result<()> {
        let path = nba_correction_dir(self.season, self.kind);

        fs::create_dir_all(&path)?;

        let json = serde_json::to_string_pretty(self)?;

        let filepath = match self.kind {
            Team => nba_team_correction_file(self.season, self.game_id.clone(), self.team_id),
            Player => nba_player_correction_file(
                self.season,
                self.game_id.clone(),
                self.player_id.unwrap().clone(),
            ),
            LineUp => unimplemented!("lineup stats not yet implemented"),
        };

        // Write the JSON string to the file
        fs::write(filepath, json)?;

        Ok(())
    }

    ///
    ///
    /// process the box score string from file and overwrite with JSON-typed corrected data
    /// consumes the original String and returns a new String
    ///
    ///
    pub fn correct_source_data(&self, game: String) -> String {
        let mut columns = game.columns();

        fn apply_corrections(
            cs: &mut Vec<String>,
            corrections: &HashMap<StatColumn, Value>,
            column_index_fn: fn(&StatColumn) -> Option<usize>,
        ) -> Option<String> {
            for (&col, val) in corrections {
                if let Some(i) = column_index_fn(&col) {
                    cs[i] = format!("{}", StatPair(col, val.clone()));
                }
            }

            // this is formatted like the original nba data.
            // our data will be nice and pretty.

            let corrected_string = format!("[{}]", cs.join(","));

            Some(corrected_string)
        }

        match (columns.as_slice(), self.kind) {
            (
                [_season_id, _player_id, _player_name, _team_id, _team_abbreviation, _team_name, _game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _fantasy_pts, _video_available],
                Player,
            ) => apply_corrections(&mut columns, &self.corrections, player_column_index).unwrap(),

            (
                [_season_id, _team_id, _team_abbreviation, _team_name, _game_id, _game_date, _matchup, _wl, _min, _fgm, _fga, _fg_pct, _fg3m, _fg3a, _fg3_pct, _ftm, _fta, _ft_pct, _oreb, _dreb, _reb, _ast, _stl, _blk, _tov, _pf, _pts, _plus_minus, _video_available],
                Team,
            ) => apply_corrections(&mut columns, &self.corrections, team_column_index).unwrap(),
            (_ls, k) => {
                eprintln!("{k} columns string was not formatted correctly");

                game.to_string()
            }
        }
    }

    pub fn len(&self) -> usize {
        self.corrections.len()
    }

    pub fn domain(&self) -> (SeasonId, NBAStatKind) {
        (self.season, self.kind)
    }

    pub fn set_delete(&mut self, delete: bool) {
        self.delete = delete
    }

    pub fn team_abbr(&self) -> TeamAbbreviation {
        self.team_abbr.clone()
    }

    pub fn correct_box_score(&mut self, game: &mut BoxScore) {
        self.corrections
            .retain(|col, val| match game.try_set_col(col, val) {
                Ok(_) => false, //remove entries that are not successfully applied
                Err(stat_column) => {
                    println!("Error setting column {stat_column}");
                    true
                }
            });
    }

    pub fn correct_box_score_builder(
        &mut self,
        box_score_builder: &mut BoxScoreBuilder,
        correction_builder: &mut CorrectionBuilder,
    ) {
        let keys = self.corrections.keys();

        for column in keys {
            match column {
                StatColumn::WL => {
                    if let Ok(wl) = self.corrections.game_result() {
                        box_score_builder.wl(wl);
                        correction_builder.remove(StatColumn::WL);
                    }
                }
                StatColumn::MIN => {
                    if let Ok(min) = self.corrections.minutes() {
                        box_score_builder.min(min);
                        correction_builder.remove(StatColumn::MIN);
                    }
                }
                StatColumn::FGM => {
                    if let Ok(fgm) = self.corrections.field_goal_makes() {
                        box_score_builder.fgm(fgm);
                        correction_builder.remove(StatColumn::FGM);
                    }
                }
                StatColumn::FGA => {
                    if let Ok(fga) = self.corrections.field_goal_attempts() {
                        box_score_builder.fga(fga);
                        correction_builder.remove(StatColumn::FGA);
                    }
                }
                StatColumn::FG3M => {
                    if let Ok(fg3m) = self.corrections.three_point_makes() {
                        box_score_builder.fg3m(fg3m);
                        correction_builder.remove(StatColumn::FG3M);
                    }
                }
                StatColumn::FG3A => {
                    if let Ok(fg3a) = self.corrections.three_point_attempts() {
                        box_score_builder.fg3a(fg3a);
                        correction_builder.remove(StatColumn::FG3A);
                    }
                }
                StatColumn::FTM => {
                    if let Ok(ftm) = self.corrections.free_throw_makes() {
                        box_score_builder.ftm(ftm);
                        correction_builder.remove(StatColumn::FTM);
                    }
                }
                StatColumn::FTA => {
                    if let Ok(fta) = self.corrections.free_throw_attempts() {
                        box_score_builder.fta(fta);
                        correction_builder.remove(StatColumn::FTA);
                    }
                }
                StatColumn::OREB => {
                    if let Ok(oreb) = self.corrections.offensive_rebounds() {
                        box_score_builder.oreb(oreb);
                        correction_builder.remove(StatColumn::OREB);
                    }
                }
                StatColumn::DREB => {
                    if let Ok(dreb) = self.corrections.defensive_rebounds() {
                        box_score_builder.dreb(dreb);
                        correction_builder.remove(StatColumn::DREB);
                    }
                }
                StatColumn::REB => {
                    if let Ok(reb) = self.corrections.rebounds() {
                        box_score_builder.reb(reb);
                        correction_builder.remove(StatColumn::REB);
                    }
                }
                StatColumn::AST => {
                    if let Ok(ast) = self.corrections.assists() {
                        box_score_builder.ast(ast);
                        correction_builder.remove(StatColumn::AST);
                    }
                }
                StatColumn::STL => {
                    if let Ok(stl) = self.corrections.steals() {
                        box_score_builder.stl(stl);
                        correction_builder.remove(StatColumn::STL);
                    }
                }
                StatColumn::BLK => {
                    if let Ok(blk) = self.corrections.blocks() {
                        box_score_builder.blk(blk);
                        correction_builder.remove(StatColumn::BLK);
                    }
                }
                StatColumn::TOV => {
                    if let Ok(tov) = self.corrections.turnovers() {
                        box_score_builder.tov(tov);
                        correction_builder.remove(StatColumn::TOV);
                    }
                }
                StatColumn::PF => {
                    if let Ok(pf) = self.corrections.personal_fouls() {
                        box_score_builder.pf(pf);
                        correction_builder.remove(StatColumn::PF);
                    }
                }
                StatColumn::PTS => {
                    if let Ok(pts) = self.corrections.points() {
                        box_score_builder.pts(pts);
                        correction_builder.remove(StatColumn::PTS);
                    }
                }
                StatColumn::PLUS_MINUS => {
                    if let Ok(plus_minus) = self.corrections.plus_minus() {
                        box_score_builder.plus_minus(plus_minus);
                        correction_builder.remove(StatColumn::PLUS_MINUS);
                    }
                }

                StatColumn::FT_PCT
                | StatColumn::FG_PCT
                | StatColumn::FG3_PCT
                | StatColumn::FANTASY_PTS
                | StatColumn::VIDEO_AVAILABLE => {
                    panic!("💀 {column} is not a field for the struct BoxScore. ")
                }

                StatColumn::SEASON_ID
                | StatColumn::PLAYER_ID
                | StatColumn::PLAYER_NAME
                | StatColumn::TEAM_ID
                | StatColumn::TEAM_ABBREVIATION
                | StatColumn::TEAM_NAME
                | StatColumn::GAME_ID
                | StatColumn::GAME_DATE
                | StatColumn::MATCHUP => {
                    panic!("💀 {column} is requireed for generating a unique identifier for the game and cannot be corrected in the BoxScore. ")
                }
            }
        }
    }

    //consume the other correction and merge the fields into the self.
    pub fn merge(&mut self, other: Correction) {
        let other_fields = other.corrections;

        for (k, v) in other_fields {
            self.corrections.insert(k, v);
        }
    }

    pub fn correct_matchup(&mut self, visiting: &mut Visiting, team_abbr: &TeamAbbreviation) {
        if let Some(corrected_matchup) = self.corrections.get(&StatColumn::MATCHUP) {
            if let Ok(matchup) = corrected_matchup.matchup() {
                match matchup.home_or_away(team_abbr) {
                    Ok(v) => {
                        *visiting = v;
                        self.corrections.remove(&StatColumn::MATCHUP);
                    }
                    Err(_) => {}
                }
            }
        };
    }

    pub fn file_path(&self) -> PathBuf {
        match self.player_id {
            Some(pid) => nba_player_correction_file(self.season, self.game_id, pid),
            None => nba_team_correction_file(self.season, self.game_id, self.team_id),
        }
    }
}

impl Debug for Correction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "szn: {}:{}\n{}-{} ({})\n{}",
            self.season,
            self.game_id,
            self.team_abbr,
            match self.player_id {
                Some(pid) => pid.to_string(),
                None => self.team_id.to_string(),
            },
            match self.kind {
                Team => "team",
                Player => "player",
                LineUp => "lineup",
            },
            match self.delete {
                true => "del".to_string(),
                false => format!(
                    "corrections: {}",
                    self.corrections
                        .iter()
                        .map(|(col, _val)| format!("{col}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
    }
}
