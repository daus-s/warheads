use format::language::columns;
use format::path_manager::{correction_file, correction_path};
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use serde_with::chrono;
use stats::kind::NBAStatKind;
use stats::stat_column::{column_index, StatColumn};
use stats::stat_value::StatValue;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::path::Path;
use std::{fs, io};

#[derive(Serialize, Deserialize, Clone)]
pub struct Correction {

    pub gameid: String,

    pub season: i32,

    pub player_id: Option<u64>,

    pub team_id: u64,

    pub team_abbr: String,

    pub kind: NBAStatKind,

    pub corrections: HashMap<StatColumn, StatValue>,
}

impl Correction {
    pub fn new(gameid: String, season: i32, player_id: Option<u64>, team_id: u64, team_abbr: String, kind: NBAStatKind) -> Correction {
        Correction {
            gameid,
            season,
            player_id,
            team_id,
            team_abbr,
            kind,
            corrections: HashMap::new(),
        }
    }

    pub(crate) fn load(filename: &str) -> Result<Correction, String> {
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
    pub(crate) fn correct(&self, game: String) -> String {
        let columns = columns(game.clone());

        match columns.as_slice() {
            [season_id, player_id, player_name, team_id, team_abbreviation, team_name, game_id, game_date, matchup, wl, min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast, stl, blk, tov, pf, pts, plus_minus, fantasy_pts, video_available] =>
            {
                let mut cs: Vec<String> = vec![
                    season_id.to_string(),
                    player_id.to_string(),
                    player_name.to_string(),
                    team_id.to_string(),
                    team_abbreviation.to_string(),
                    team_name.to_string(),
                    game_id.to_string(),
                    game_date.to_string(),
                    matchup.to_string(),
                    wl.to_string(),
                    min.to_string(),
                    fgm.to_string(),
                    fga.to_string(),
                    fg_pct.to_string(),
                    fg3m.to_string(),
                    fg3a.to_string(),
                    fg3_pct.to_string(),
                    ftm.to_string(),
                    fta.to_string(),
                    ft_pct.to_string(),
                    oreb.to_string(),
                    dreb.to_string(),
                    reb.to_string(),
                    ast.to_string(),
                    stl.to_string(),
                    blk.to_string(),
                    tov.to_string(),
                    pf.to_string(),
                    pts.to_string(),
                    plus_minus.to_string(),
                    fantasy_pts.to_string(),
                    video_available.to_string(),
                ];

                for (&col, val) in self.corrections.iter() {
                    if let Some(i) = column_index(&col) {

                        let f_str = val.val().unwrap_or_else(|| Null).to_string();

                        cs[i] = f_str;
                    }
                }

                format!("[{:#}]", cs.join(","))
            }
            _ => {
                eprintln!("columns string was not formatted correctly");
                "".to_string()
            }
        }
    }

    /// Saves the correction to the file:
    /// `corrections/{season_file}/{NBAStatType}/{}.json`
    ///
    /// This is a private function and is called when `fn create ->` is completed.
    ///
    pub fn save(&self) -> io::Result<()> {
        let path = correction_path(self.season - 20000, self.kind);

        let file = correction_file(self.gameid.as_str(), self.player_id.unwrap_or(self.team_id));

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

    pub fn domain(&self) -> (i32, NBAStatKind) {
        (self.season - 20000, self.kind)
    }
}

impl Debug for Correction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "szn: {}:{}\n{}\nid: {} ({})\n[.{}.]", self.season, self.gameid, self.team_abbr ,self.player_id.unwrap_or(self.team_id), match self.kind {
            NBAStatKind::Team => "team",
            NBAStatKind::Player => "player",
            NBAStatKind::LineUp => "lineup",
        }, self.corrections.len() )
    }
}