use crate::corrections::correction::Correction;
use crate::format::path_manager::nba_correction_dir;
use crate::format::season::season_fmt;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;
use crate::types::SeasonId;
use std::fs;
use std::fs::{DirEntry, ReadDir};

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

pub fn load_corrections(season_id: SeasonId, kind: NBAStatKind) -> Result<Vec<Correction>, String> {
    eprintln!("loading corrections for {}", season_id);

    let (year, period) = season_id.destructure();

    let mut corrections = Vec::new();

    let files = correction_files(season_id, kind)?;

    let dir = load_corrections_from_file(files);

    if let Ok(mut loaded_corrections) = dir {
        corrections.append(&mut loaded_corrections);
    } else if let Err(e) = dir {
        return Err(format!(
            "⚠️  failed to load corrections for the {} {}\n{e}",
            season_fmt(year),
            period
        ));
    }

    dbg!(&corrections);

    Ok(corrections)
}

fn correction_files(season_id: SeasonId, kind: NBAStatKind) -> Result<ReadDir, String> {
    let c_path = nba_correction_dir(season_id, kind);

    fs::create_dir_all(&c_path).map_err(|e| format!("⚠️  failed to create directory path:{e}"))?;

    fs::read_dir(&c_path).map_err(|e| format!("⚠️  failed to read directory {}: {}", &c_path, e))
}

fn load_corrections_from_file(
    files: impl Iterator<Item = Result<DirEntry, std::io::Error>>,
) -> Result<Vec<Correction>, String> {
    files
        .map(|file_result| {
            let path = file_result.map_err(|e| format!("⚠️  file error: {}", e))?;

            let path_buf = path.path();

            let path_str = path_buf.to_str().ok_or("⚠️  invalid file path encoding")?;

            read_correction(path_str)
        })
        .collect()
}

fn read_correction(filename: &str) -> Result<Correction, String> {
    fs::read_to_string(filename)
        .map_err(|e| format!("⚠️  failed to read file {}: {}", filename, e))
        .and_then(|json| {
            serde_json::from_str(&json)
                .map_err(|e| format!("⚠️  failed to parse JSON in {}: {}", filename, e))
        })
}
