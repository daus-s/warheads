use crate::corrections::correction::Correction;
use crate::format::path_manager::nba_correction_dir;
use crate::stats::nba_kind::NBAStatKind;
use crate::types::SeasonId;
use std::fs;
use std::fs::DirEntry;

pub fn load_corrections(
    season: SeasonId,
    kind: NBAStatKind,
) -> Result<Vec<Correction>, String> {
    let c_path = nba_correction_dir(season, kind);

    let files = fs::read_dir(&c_path)
        .map_err(|e| format!("failed to read directory {}: {}", &c_path, e))?;

    load_corrections_from_file(files)
}

fn load_corrections_from_file(
    files: impl Iterator<Item = Result<DirEntry, std::io::Error>>,
) -> Result<Vec<Correction>, String> {
    files
        .map(|file_result| {
            let path = file_result.map_err(|e| format!("File error: {}", e))?;

            let path_buf = path.path();

            let path_str = path_buf.to_str().ok_or("Invalid file path encoding")?;

            read_correction(path_str)
        })
        .collect()
}

fn read_correction(filename: &str) -> Result<Correction, String> {
    fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))
        .and_then(|json| {
            serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse JSON in {}: {}", filename, e))
        })
}
