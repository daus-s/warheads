use crate::edit::edit::Edit;
use crate::edit::edit_list::EditList;

use crate::format::path_manager;

use crate::stats::nba_kind::NBAStatKind;

use std::fmt::Display;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;

use EditLoadingError::*;

pub fn load_edit_list() -> Result<EditList, EditLoadingError> {
    let filepath = path_manager::nba_edit_file();

    let mut file =
        fs::File::open(&filepath).map_err(|e| EditLoadingError::FileError(e, filepath.clone()))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| EditLoadingError::FileError(e, filepath.clone()))?;

    let edits: EditList = serde_json::from_str(&contents)
        .map_err(|e| EditLoadingError::ParseError(e, filepath.clone()))?;

    Ok(edits)
}

pub fn save_edit_list(edits: &EditList) -> Result<(), ()> {
    let path = path_manager::nba_edit_file();

    let contents = serde_json::to_string(edits.edits()).map_err(|_| ())?;

    fs::write(path, contents).map_err(|_| ())
}

pub fn partition_edit_list(edits: &EditList) -> (Vec<Edit>, Vec<Edit>) {
    edits
        .clone()
        .into_edits()
        .into_iter()
        .partition(|edit| edit.kind() == NBAStatKind::Player)
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

    use crate::{edit::edit_loader::load_edit_list, stats::identity::Identifiable};

    #[test]
    fn bench_edits_v2() {
        let start_new = Instant::now();

        let _new_edits = load_edit_list()
            .expect("failed to load edits from new fs")
            .edits()
            .iter()
            .map(|e| (e.identity(), e.clone()))
            .collect::<HashMap<_, _>>();

        println!("edits loading v2: {:?}", start_new.elapsed());
    }
}
