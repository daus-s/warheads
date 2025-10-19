use crate::format::path_manager::nba_source_path;
use crate::stats::domain::Domain;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::stats::season_period::minimum_spanning_era;
use crate::types::SeasonId;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub trait Archive {
    fn write(&mut self, new_content: String) -> Result<(), ()>;

    fn contents(&self) -> String;

    fn path(&self) -> String;
}

impl Archive for String {
    fn write(&mut self, new_content: String) -> Result<(), ()> {
        *self = new_content;

        Ok(())
    }

    fn contents(&self) -> String {
        self.to_string()
    }

    fn path(&self) -> String {
        String::from("String")
    }
}

impl Archive for PathBuf {
    fn write(&mut self, new_content: String) -> Result<(), ()> {
        let _parseable: Value = serde_json::from_str(&new_content).map_err(|e| {
            eprintln!("❌ failed to convert new data to JSON. writing to JSON file requires being parseable: {}", e);

            ()
        })?;

        match fs::write(&self, new_content) {
            Ok(_) => Ok(()),
            Err(_e) => Err(()),
        }
    }

    fn contents(&self) -> String {
        fs::read_to_string(&self).unwrap_or_default()
    }

    fn path(&self) -> String {
        format!("{}", &self.display())
    }
}

pub fn domain_archive_pairs(year: i32) -> HashMap<Domain, PathBuf> {
    let eras = minimum_spanning_era(year);

    let mut dap: HashMap<Domain, PathBuf> = HashMap::new();

    for season_id in eras {
        dap.insert((season_id, Player), nba_source_path(season_id, Player));
        dap.insert((season_id, Team), nba_source_path(season_id, Team));
    }

    dap
}

pub fn typed_domain_archive_pairs(year: i32, kind: NBAStatKind) -> HashMap<Domain, PathBuf> {
    let eras = minimum_spanning_era(year);

    let mut dap: HashMap<Domain, PathBuf> = HashMap::new();

    for season_id in eras {
        dap.insert((season_id, kind), nba_source_path(season_id, kind));
    }

    dap
}

// return a single domain archive pair as a HashMap. this is used for by dapi,
// when sourcing and writing data, as it is broken into a single season period.
pub fn domain_archive_pair(season_id: SeasonId, kind: NBAStatKind) -> HashMap<Domain, PathBuf> {
    let mut dap = HashMap::new();

    let domain = (season_id, kind);

    dap.insert(domain, nba_source_path(season_id, kind));

    dap
}
