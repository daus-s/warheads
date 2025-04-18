use std::fs;
use std::path::Path;
use serde_json::Value;
use format::language::partition;
use format::path_manager::data_path;
use format::season::season_fmt;
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

        let (season, kind, period) = self[0].domain();

        // a path understandable only in the context of this data schema
        let path = data_path(season, kind, period);

        //open file based on season info
        let path_to_file = Path::new(&path);

        let content = fs::read_to_string(path_to_file).map_err(|_| format!("failed to read file {:?}", path))?;

        let parsed: Value = serde_json::from_str(&content)
            .map_err(|_| "failed to parse JSON from file")?;

        let mut games = json_to_rows(parsed).map_err(|e| e)?;

        for game in &mut games {
            for correction in self {

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

        let new_content = partition(content, format!("[{}]", games.join(",")));

        match fs::write(path_to_file, new_content) {
            Ok(_) => {
                println!("successfully applied corrections for {} season the in the file {:?}", season_fmt(season) , path);
                Ok(())
            },
            Err(e) => Err(format!("failed to write to file. {:?}:\n{}", path, e ))
        }
    }
}