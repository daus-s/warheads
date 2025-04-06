use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use stats::kind::NBAStatKind;
use constants::corrections;
use once_cell::sync::Lazy;
use format::path_manager::correction_path;
use crate::correction::Correction;

static CORRECTIONS: Lazy<String> = Lazy::new(corrections);

pub fn load_corrections(szn: i32, kind: NBAStatKind) -> Result<Vec<Correction>, String> {

    let c_path = correction_path(szn, kind);

    let files = fs::read_dir(&c_path)
        .map_err(|e| format!("failed to read directory {}: {}", &c_path, e))?;

    load_corrections_from_file(files)
}

fn load_corrections_from_file(files: impl Iterator<Item = Result<DirEntry, std::io::Error>>) -> Result<Vec<Correction>, String> {
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