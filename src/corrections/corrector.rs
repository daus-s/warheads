use crate::corrections::correction::Correction;
use crate::corrections::overwrite;
use crate::dapi::archive::Archive;
use crate::dapi::extract::json_to_hashmap;
use crate::stats::domain::Domain;
use crate::stats::id::{Identifiable, Identity};
use std::collections::HashMap;

pub trait Corrector {
    ///applies the corrections from the self and writes it to an Archive object
    fn apply<A: Archive>(&self, archives: &mut HashMap<Domain, A>) -> Result<(), String>;
}

impl Corrector for Vec<Correction> {
    fn apply<A>(&self, archives: &mut HashMap<Domain, A>) -> Result<(), String>
    where
        A: Archive,
    {
        if self.is_empty() {
            return Ok(());
        }

        let mut files: HashMap<Domain, HashMap<Identity, String>> = HashMap::new();

        for (&domain, archive) in archives.into_iter() {
            let value = serde_json::from_str(&archive.contents()).map_err(|e| {
                format!(
                    "❌ failed to parse a JSON object from the archive {}: {e}",
                    archive.path()
                )
            })?;

            let map = json_to_hashmap(&value)
                .map_err(|e| format!("❌ failed to convert JSON object into a hashmap: {e}"))?;

            files.insert(domain, map);
        }

        let mut to_remove = Vec::new();

        // there are always fewer corrections than games so we iterate over the corrections and then
        // search with O(1) lookup in hashmap (hash might be slow for Identity)

        for correction in self {
            let domain = correction.domain();

            let map = files.get_mut(&domain).ok_or_else(|| {
                "❌ correction didnt have a relevant archive to be applied to".to_string()
            })?;

            let id = correction.identity();

            if let Some(game) = map.get_mut(&id) {

                dbg!(correction.delete);

                if correction.delete {
                    to_remove.push(id);
                } else {
                    *game = correction.correct_string(game.to_string());
                }
            }
        }

        dbg!(&to_remove);

        for id in to_remove {
            if let Some(map) = files.get_mut(&id.domain()) {
                map.remove(&id);
            }
        }


        for (domain, games_by_id) in files {
            let mut games_vector = games_by_id.into_values().collect::<Vec<String>>();

            games_vector.sort(); //this is only needed to make the

            overwrite::overwrite(domain, games_vector, archives.get_mut(&domain).unwrap())?;
        }

        Ok(())
    }
}
