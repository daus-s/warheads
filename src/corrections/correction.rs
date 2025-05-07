use crate::format::language::columns;
use crate::format::path_manager::{nba_correction_dir, nba_correction_file};

use serde::{Deserialize, Serialize};
use serde_json::Value::Null;

use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_type::SeasonPeriod;
use crate::stats::stat_column::{player_column_index, StatColumn};
use crate::stats::stat_value::StatValue;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use std::path::Path;
use std::{fs, io};
use crate::stats::nba_kind::NBAStatKind::{Player, Team};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Correction {
    pub game_id: String,

    pub season: i32,

    pub player_id: Option<u64>,

    pub team_id: u64,

    pub team_abbr: String,

    pub kind: NBAStatKind,

    pub period: SeasonPeriod,

    pub delete: bool,

    pub corrections: HashMap<StatColumn, StatValue>,
}

impl Correction {
    pub fn new(
        game_id: String,
        season: i32,
        player_id: Option<u64>,
        team_id: u64,
        team_abbr: String,
        kind: NBAStatKind,
        period: SeasonPeriod,
    ) -> Correction {
        Correction {
            game_id,
            season,
            player_id,
            team_id,
            team_abbr,
            period,
            kind,
            delete: false,
            corrections: HashMap::new(),
        }
    }

    pub fn load(filename: &str) -> Result<Correction, String> {
        let path = Path::new(filename);

        // Read the file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;

        // Deserialize the content into a Correction struct
        let correction: Correction = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize file {}: {}", filename, e))?;

        Ok(correction)
    }

    ///
    ///
    /// process the box score string from file and overwrite with JSON-typed corrected data
    /// consumes the original String and returns a new String
    ///
    ///
    pub fn correct(&self, game: &str) -> String {
        let mut columns = columns(game);

        fn apply_corrections(
            cs: &mut Vec<String>,
            corrections: &HashMap<StatColumn, StatValue>,
            column_index_fn: fn(&StatColumn) -> Option<usize>,
        ) -> Option<String> {

            for (&col, val) in corrections {
                if let Some(i) = column_index_fn(&col) {
                    cs[i] = val.val().unwrap_or_else(|| Null).to_string();
                }
            }

            Some(format!("[{}]", cs.join(",")))
        }

        match (columns.as_slice(), self.kind) {
            (
                [season_id, player_id, player_name, team_id,
                 team_abbreviation, team_name, game_id, game_date,
                 matchup, wl, min, fgm, fga, fg_pct,
                 fg3m, fg3a, fg3_pct, ftm, fta, ft_pct,
                 oreb, dreb, reb, ast, stl, blk,
                 tov, pf, pts, plus_minus, fantasy_pts,
                 video_available],
                Player
            ) => apply_corrections(&mut columns, &self.corrections, player_column_index).unwrap(),

            (
                [season_id, team_id, team_abbreviation, team_name, game_id, game_date, matchup, wl,
                 min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast,
                 stl, blk, tov, pf, pts, plus_minus, video_available],
                Team
            ) => apply_corrections(&mut columns, &self.corrections, player_column_index).unwrap(),
            _ => {
                eprintln!("columns string was not formatted correctly");
                String::new()
            }
        }
    }

    /// Saves the correction to the file:
    /// `corrections/{season_file}/{NBAStatType}/{}.json`
    ///
    /// This is a private function and is called when `fn create ->` is completed.
    ///
    pub fn save(&self) -> io::Result<()> {
        let path = nba_correction_dir(self.season - 20000, self.kind, self.period);

        let file = nba_correction_file(
            self.season - 20000,
            self.kind,
            self.period,
            self.game_id.as_str(),
            self.player_id.unwrap_or(self.team_id),
        );

        fs::create_dir_all(&path)?;

        let json = serde_json::to_string_pretty(self)?;

        // Write the JSON string to the file
        fs::write(format!("{}{}", path, file), json)?;

        Ok(())
    }

    pub fn add_missing_field(&mut self, col: StatColumn, val: StatValue) {
        self.corrections.insert(col, val);
    }

    pub fn len(&self) -> usize {
        self.corrections.len()
    }

    pub fn domain(&self) -> (i32, NBAStatKind, SeasonPeriod) {
        (self.season - 20000, self.kind, self.period)
    }

    pub fn set_delete(&mut self, delete: bool) {
        self.delete = delete
    }
}

impl Debug for Correction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "szn: {}:{}\n{}\nid: {} ({})\n[.{}.]",
            self.season,
            self.game_id,
            self.team_abbr,
            self.player_id.unwrap_or(self.team_id),
            match self.kind {
                NBAStatKind::Team => "team",
                NBAStatKind::Player => "player",
                NBAStatKind::LineUp => "lineup",
            },
            self.corrections.len()
        )
    }
}


