use crate::format::path_manager::{nba_checksum_path, nba_data_path};
use crate::stats::domain::Domain;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct ChecksumMap {
    pub map: HashMap<PathBuf, u32>,
}

impl ChecksumMap {
    pub fn load() -> Result<Self, String> {
        let path = nba_checksum_path();

        let file = File::open(&path).map_err(|e| format!("❌ failed to open file: {}", e))?;

        let map = serde_json::from_reader(file)
            .map_err(|e| format!("❌ failed to parse JSON into a HashMap: {}", e))?;

        Ok(ChecksumMap { map })
    }

    pub fn new() -> Self {
        ChecksumMap {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: PathBuf, checksum: u32) {
        self.map.insert(path, checksum);
    }

    pub fn verify_checksums(&self, other: &ChecksumMap) -> Vec<PathBuf> {
        let mut errors = Vec::new();

        for path in self.map.keys() {
            if let Some(self_checksum) = self.map.get(path) {
                if let Some(other_checksum) = other.map.get(path) {
                    if self_checksum != other_checksum {
                        errors.push(path.clone());
                    }
                } else {
                    errors.push(path.clone());
                }
            } else {
                //if its in keys and its not in self then how can it possibly exist ?
                panic!("💀 ChecksumMap::verify_checksums: domain not found in self");
            }
        }

        for path in other.map.keys() {
            if !self.map.contains_key(path) {
                errors.push(path.clone());
            }
        }

        errors
    }

    pub fn save(&self) -> Result<(), String> {
        let path = nba_checksum_path();

        let file = File::create(&path).map_err(|e| format!("❌ failed to create file: {}", e))?;

        let serializable_map = self
            .map
            .iter()
            .map(|(path, checksum)| (path.display().to_string(), *checksum))
            .collect::<HashMap<String, u32>>();

        serde_json::to_writer_pretty(file, &serializable_map)
            .map_err(|e| format!("❌ failed to serialize ChecksumMap into JSON: {}", e))?;

        Ok(())
    }
}
