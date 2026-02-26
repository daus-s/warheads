use crate::dapi::from_value::FromValue;

use crate::edit::edit_builder::EditBuilder;
use crate::edit::edit_loader::load_edit_list;

use crate::format::language::Columnizable;

use crate::stats::visiting::Visiting;

use crate::stats::box_score::{BoxScore, BoxScoreBuilder};
use crate::stats::identity::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{LineUp, Player, Team};
use crate::stats::stat_column::{player_column_index, team_column_index, StatColumn};
use crate::stats::statify::StatPair;

use crate::types::{GameDate, GameId, Matchup, PlayerId, SeasonId, TeamAbbreviation, TeamId};

use serde_json::Value;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Eq, Debug)]
pub struct Edit {
    pub game_id: GameId,

    pub game_date: GameDate,

    pub season: SeasonId,

    pub team_id: TeamId,

    pub player_id: Option<PlayerId>,

    pub team_abbr: TeamAbbreviation,

    pub delete: bool,

    pub corrections: HashMap<StatColumn, Value>,
}

impl Edit {
    pub fn load(filename: &str) -> Result<Edit, String> {
        let path = Path::new(filename);

        // Read the file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;

        // Deserialize the content into a Correction struct
        let correction: Edit = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize file {}: {}", filename, e))?;

        //todo: assert eq the path info and the file content

        Ok(correction)
    }

    /// insert an edit record directly to the json file. this is not as performant and requires reading, parsing
    /// and a O(logn) insert in the edit list loaded from file. if you are creating many edits, use a reference
    /// to an edit list and keep in memory.
    pub fn insert_to_file(&self, edit: Edit) -> Result<(), ()> {
        let mut edit_list = load_edit_list().map_err(|_| ())?;

        edit_list.insert(edit);

        Ok(())
    }

    pub fn kind(&self) -> NBAStatKind {
        match self.player_id {
            Some(_) => NBAStatKind::Player,
            None => NBAStatKind::Team,
        }
    }

    pub fn len(&self) -> usize {
        self.corrections.len()
    }

    pub fn domain(&self) -> (SeasonId, NBAStatKind) {
        (self.season, self.kind())
    }

    pub fn set_delete(&mut self, delete: bool) {
        self.delete = delete
    }

    pub fn team_abbr(&self) -> TeamAbbreviation {
        self.team_abbr.clone()
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

        match (columns.as_slice(), self.kind()) {
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

    pub fn edit_box_score(&mut self, game: &mut BoxScore) {
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
        correction_builder: &mut EditBuilder,
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
                    panic!("💀 {column} is required for generating a unique identifier for the game and cannot be corrected in the BoxScore. ")
                }
            }
        }
    }

    //consume the other correction and merge the fields into the self.
    pub fn merge(&mut self, other: Edit) {
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

    pub fn corrects(&self, matchup: &StatColumn) -> bool {
        self.corrections.contains_key(matchup)
    }

    pub fn matchup_as_value(&self) -> Option<Value> {
        let matchup = self
            .corrections
            .get(&StatColumn::MATCHUP)?
            .as_str()?
            .parse::<Matchup>()
            .ok()?;

        let s = if matchup.home == self.team_abbr() {
            format!("{} vs. {}", self.team_abbr(), matchup.away)
        } else {
            format!("{} @ {}", self.team_abbr(), matchup.home)
        };

        Some(Value::String(s))
    }

    pub fn inverse_matchup_as_value(&self) -> Option<Value> {
        let matchup = self
            .corrections
            .get(&StatColumn::MATCHUP)?
            .as_str()?
            .parse::<Matchup>()
            .ok()?;

        let s = if matchup.home == self.team_abbr() {
            format!("{} @ {}", matchup.away, self.team_abbr())
        } else {
            format!("{} vs. {}", matchup.home, self.team_abbr())
        };

        Some(Value::String(s))
    }
}

impl Display for Edit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "game_id: {} {}\t{}-{} ({})\t{}",
            self.game_id,
            self.season,
            self.team_abbr,
            match self.player_id {
                Some(pid) => pid.to_string(),
                None => self.team_id.to_string(),
            },
            match self.kind() {
                Team => "team",
                Player => "player",
                LineUp => "lineup",
            },
            match self.delete {
                true => "to be deleted".to_string(),
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

impl Identifiable for Edit {
    fn identity(&self) -> Identity {
        // eprintln!("parsing gameid as u64: {}", match self.gameid.replace("\"", "").parse::<u64>() {
        //     Ok(_) => "success",
        //     Err(_) => "failure"
        // });

        match self.kind() {
            NBAStatKind::Team => Identity {
                season_id: self.season,
                game_id: self.game_id.clone(),
                player_id: None,
                team_id: self.team_id,
                game_date: self.game_date,
                team_abbr: self.team_abbr.clone(),
            },
            NBAStatKind::Player => Identity {
                season_id: self.season,
                game_id: self.game_id.clone(),
                player_id: Some(
                    self.player_id
                        .expect("💀 no player id for a player correction object. "),
                ),
                team_id: self.team_id,
                game_date: self.game_date,
                team_abbr: self.team_abbr.clone(),
            },
            NBAStatKind::LineUp => todo!("lineup stats not yet implemented"),
        }
    }
}

impl PartialEq for Edit {
    fn eq(&self, other: &Self) -> bool {
        self.game_id == other.game_id
            && self.season == other.season
            && self.team_id == other.team_id
            && self.player_id == other.player_id
    }
}
