use thiserror::Error;

use crate::edit::edit::Edit;
use crate::edit::edit_list::EditList;
use crate::format::path_manager::{
    correction_path_from_identity, nba_correction_dir, nba_edit_file,
};
use crate::format::season::season_fmt;
use crate::stats::identity::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;
use crate::types::SeasonId;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{DirEntry, ReadDir};
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

use EditLoadingError::*;

pub fn load_edit_list() -> Result<EditList, EditLoadingError> {
    let filepath = nba_edit_file();

    let mut file =
        fs::File::open(&filepath).map_err(|e| EditLoadingError::FileError(e, filepath.clone()))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| EditLoadingError::FileError(e, filepath.clone()))?;

    let edits: EditList = serde_json::from_str(&contents)
        .map_err(|e| EditLoadingError::ParseError(e, filepath.clone()))?;

    Ok(edits)
}

pub fn split_edit_list(edits: EditList) -> (Vec<Edit>, Vec<Edit>) {
    edits
        .into_edits()
        .into_iter()
        .partition(|edit| edit.kind == NBAStatKind::Player)
}

fn load_season_corrections(year: i32, kind: NBAStatKind) -> Result<Vec<Edit>, String> {
    let season = minimum_spanning_era(year);

    let mut edits = Vec::new();

    for period in season {
        let files = correction_files(period, kind)?;

        let dir = load_corrections_from_file(files);

        if let Ok(mut loaded_corrections) = dir {
            edits.append(&mut loaded_corrections);
        } else if let Err(e) = dir {
            return Err(format!(
                "⚠️  failed to load corrections for the {} {}\n{e}",
                season_fmt(year),
                period.period()
            ));
        }
    }

    Ok(edits)
}

pub fn load_season_correction_maps(
    year: i32,
) -> Result<(HashMap<Identity, Edit>, HashMap<Identity, Edit>), String> {
    let player_corrections = load_season_corrections(year, NBAStatKind::Player)?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Edit>>();

    let team_corrections = load_season_corrections(year, NBAStatKind::Team)?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Edit>>();

    Ok((player_corrections, team_corrections))
}

fn correction_files(season_id: SeasonId, kind: NBAStatKind) -> Result<ReadDir, String> {
    let c_path = nba_correction_dir(season_id, kind);

    fs::create_dir_all(&c_path).map_err(|e| format!("⚠️  failed to create directory path:{e}"))?;

    fs::read_dir(&c_path).map_err(|e| format!("⚠️  failed to read directory {}: {}", &c_path, e))
}

fn load_corrections_from_file(
    files: impl Iterator<Item = Result<DirEntry, std::io::Error>>,
) -> Result<Vec<Edit>, EditLoadingError> {
    files
        .map(|file_result| {
            let path = file_result.map_err(|e| DirectoryError(e))?;

            let path_buf = path.path();

            read_correction(&path_buf)
        })
        .collect()
}

pub fn load_single_correction(identity: &Identity) -> Result<Edit, EditLoadingError> {
    let correction_path = correction_path_from_identity(identity);

    read_correction(&correction_path)
}

fn read_correction(filename: &PathBuf) -> Result<Edit, EditLoadingError> {
    fs::read_to_string(filename)
        .map_err(|e| FileError(e, filename.clone()))
        .and_then(|json| serde_json::from_str(&json).map_err(|e| ParseError(e, filename.clone())))
}

#[derive(Error, Debug)]
pub enum EditLoadingError {
    DirectoryError(io::Error),
    FileError(io::Error, PathBuf),
    ParseError(serde_json::Error, PathBuf),
}

impl Display for EditLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError(error, filename) => write!(
                f,
                "⚠️  failed to read file {}: {}",
                filename.display(),
                error
            ),
            ParseError(error, filename) => write!(
                f,
                "⚠️  failed to parse JSON in {}: {}",
                filename.display(),
                error
            ),
            DirectoryError(error) => write!(
                f,
                "⚠️  failed to access directory, (DirEntry's failed): {}",
                error
            ),
        }
    }
}

#[cfg(test)]
mod load_edits {
    use std::{collections::HashMap, time::Instant};

    use crate::{
        dapi::season_manager::nba_lifespan,
        edit::edit_loader::{load_edit_list, load_season_correction_maps},
        stats::identity::Identifiable,
    };

    #[test]
    fn test_load_edits() {
        let start_old = Instant::now();
        let mut old_edits = HashMap::new();

        for year in nba_lifespan() {
            let (player_edits, team_edits) =
                load_season_correction_maps(year).expect("failed to get edits in test workflow");

            old_edits.extend(player_edits);
            old_edits.extend(team_edits);
        }
        println!("edits loading v1: {:?}", start_old.elapsed());

        let start_new = Instant::now();
        let new_edits = load_edit_list()
            .expect("failed to load edits from new fs")
            .edits()
            .iter()
            .map(|e| (e.identity(), e.clone()))
            .collect::<HashMap<_, _>>();
        println!("edits loading v2: {:?}", start_new.elapsed());

        assert_eq!(
            old_edits,
            new_edits,
            "old len: {}\nnew len: {}",
            old_edits.len(),
            new_edits.len()
        );
    }
}
