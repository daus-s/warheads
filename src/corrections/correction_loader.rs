use thiserror::Error;

use crate::corrections::correction::Correction;
use crate::format::path_manager::{correction_path_from_identity, nba_correction_dir};
use crate::format::season::season_fmt;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;
use crate::types::SeasonId;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;
use std::{fs, io};

use CorrectionLoadingError::*;

pub fn load_season_corrections(year: i32, kind: NBAStatKind) -> Result<Vec<Correction>, String> {
    let season = minimum_spanning_era(year);

    let mut corrections = Vec::new();

    for period in season {
        let files = correction_files(period, kind)?;

        let dir = load_corrections_from_file(files);

        if let Ok(mut loaded_corrections) = dir {
            corrections.append(&mut loaded_corrections);
        } else if let Err(e) = dir {
            return Err(format!(
                "⚠️  failed to load corrections for the {} {}\n{e}",
                season_fmt(year),
                period.period()
            ));
        }
    }

    Ok(corrections)
}

pub fn load_season_correction_maps(
    year: i32,
) -> Result<(HashMap<Identity, Correction>, HashMap<Identity, Correction>), String> {
    let player_corrections = load_season_corrections(year, NBAStatKind::Player)?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Correction>>();

    let team_corrections = load_season_corrections(year, NBAStatKind::Team)?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Correction>>();

    Ok((player_corrections, team_corrections))
}

fn correction_files(season_id: SeasonId, kind: NBAStatKind) -> Result<ReadDir, String> {
    let c_path = nba_correction_dir(season_id, kind);

    fs::create_dir_all(&c_path).map_err(|e| format!("⚠️  failed to create directory path:{e}"))?;

    fs::read_dir(&c_path).map_err(|e| format!("⚠️  failed to read directory {}: {}", &c_path, e))
}

fn load_corrections_from_file(
    files: impl Iterator<Item = Result<DirEntry, std::io::Error>>,
) -> Result<Vec<Correction>, CorrectionLoadingError> {
    files
        .map(|file_result| {
            let path = file_result.map_err(|e| DirectoryError(e))?;

            let path_buf = path.path();

            read_correction(&path_buf)
        })
        .collect()
}

pub fn load_single_correction(identity: &Identity) -> Result<Correction, CorrectionLoadingError> {
    let correction_path = correction_path_from_identity(identity);

    read_correction(&correction_path)
}

fn read_correction(filename: &PathBuf) -> Result<Correction, CorrectionLoadingError> {
    fs::read_to_string(filename)
        .map_err(|e| FileError(e, filename.clone()))
        .and_then(|json| serde_json::from_str(&json).map_err(|e| ParseError(e, filename.clone())))
}

#[derive(Error, Debug)]
pub enum CorrectionLoadingError {
    DirectoryError(io::Error),
    FileError(io::Error, PathBuf),
    ParseError(serde_json::Error, PathBuf),
}

impl Display for CorrectionLoadingError {
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
