use crate::corrections::correction::Correction;
use crate::corrections::overwrite;
use crate::format::extract::json_to_hashmap;
use crate::stats::id::{Identifiable, Identity};
use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use crate::format::path_manager::nba_data_path;

pub trait Corrector {
    fn apply(&self) -> Result<(), String>;
}

impl Corrector for Vec<Correction> {
    fn apply(&self) -> Result<(), String> {

        if self.is_empty() {
            return Ok(());
        }

        let (season, kind, period) = self[0].domain();

        let data_path = nba_data_path(season, kind, period);

        let contents = fs::read_to_string(&data_path)
            .unwrap_or_else(|e| format!("failed to read file {:?}: {e}", data_path));

        let mut games_by_id = json_to_hashmap(&Value::from(contents))
            .map_err(|e| format!("failed to convert json to rows: {}", e))?;

        let mut to_remove = Vec::new();

        // there are always fewer corrections than games so we iterate over the corrections and then
        // search with O(1) lookup in hashmap (hash might be slow for Identity)

        for correction in self {

            let id = &correction.identity();

            if let Some(game) = games_by_id.get_mut(id) {
                if correction.delete {
                    to_remove.push(id.clone());
                } else {
                    *game = correction.correct(game.to_string());
                }
            }
        }

        for id in to_remove {
            games_by_id.remove(&id);
        }

        let games_vector = games_by_id.into_values().collect::<Vec<String>>();

        overwrite::write_to_data_file(self[0].domain(), games_vector)
    }
}
