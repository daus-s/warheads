use crate::format::language::Columnizable;
use crate::format::path_manager::{
    nba_correction_dir, nba_player_correction_file, nba_team_correction_file,
};

use serde::{Deserialize, Serialize};

use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::SeasonPeriod;
use crate::stats::stat_column::{player_column_index, team_column_index, StatColumn};

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::stats::box_score::BoxScore;
use crate::stats::nba_kind::NBAStatKind::{LineUp, Player, Team};
use crate::stats::statify::StatPair;
use crate::types::{GameDate, GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};
use serde_json::Value;
use std::path::Path;
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
    pub fn correct_string(&self, game: String) -> String {
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

    pub fn correct_box_score(&self, game: &mut BoxScore) {
        for (col, val) in self.corrections.iter() {
            let _ = game.try_set_col(col, val);
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
