/*
///
/// apply the correction to the vec<player or team box score that will be written.
///
pub fn apply(&self) -> Result<(), String> {
    for game in games.iter_mut() {
        //find the players results
        if self {
            println!("updating game");
            //edit the player vector
            *game = self.correct(game.clone());
            break;
        }
    };
}
*/
use std::fs;
use std::path::Path;
use serde_json::Value;
use format::path_manager::data_path;
use stats::extract::json_to_rows;
use stats::id::Identifiable;
use crate::correction::Correction;

pub trait Corrector {
    fn apply(&self) -> Result<(), String>;
}

impl Corrector for Vec<Correction> {
    fn apply(&self) -> Result<(), String> {
        //assume the first games

        if self.len() == 0 {
            return Ok(())
        }

        let (season, kind) = self[0].domain();

        // a path understandable only in the context of this data schema
        let path = &data_path(season, kind);

        //open file based on season info
        let path_to_file = Path::new(path);

        let content = fs::read_to_string(path_to_file).map_err(|_| format!("failed to read file {}", path))?;

        let parsed: Value = serde_json::from_str(&content)
            .map_err(|_| "failed to parse JSON from file")?;

        let mut games = json_to_rows(parsed).map_err(|e| e)?;

        for game in &mut games {
            for correction in self {
                eprintln!("{:?}", correction);

                //we should only make corrections with "affirmative consent". that is if either option responds with None we cannot compare.
                let (Some(string_id), Some(correction_id)) = (game.identity(), correction.identity()) else {
                    // Skip if either identity is None
                    continue;
                };

                if string_id == correction_id {
                    *game = correction.correct(game.clone());
                }
            }
        }

        Ok(())
    }
}