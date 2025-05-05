use crate::corrections::correction::Correction;
use crate::corrections::overwrite;
use crate::stats::extract::json_to_hashmap;
use crate::stats::id::{Identifiable, Identity};
use std::collections::HashMap;

pub trait Corrector {
    fn apply(&self) -> Result<(), String>;
}

impl Corrector for Vec<Correction> {
    fn apply(&self) -> Result<(), String> {

        if self.is_empty() {
            return Ok(());
        }

        let mut games_by_id = json_to_hashmap(self[0].domain())
                             .map_err(|e| format!("failed to convert json to rows: {}", e))?;

        let corrections_by_id: HashMap<Identity, &Correction> = self
            .iter()
            .map(|correction| {
                (correction.identity(), correction)
            })
            .collect();

        let mut to_remove = Vec::new();

        for (id, game) in &mut games_by_id {
            if let Some(correction) = corrections_by_id.get(id) {
                if correction.delete {
                    to_remove.push(id.clone());
                } else {
                    *game = correction.correct(game);
                }
            }
        }

        for id in to_remove {
            games_by_id.remove(&id.clone());
        }

        let games_vector = games_by_id.into_values().collect::<Vec<String>>();

        overwrite::write_to_data_file(self[0].domain(), games_vector)
    }
}
