use std::fs;
use crate::format::language::partition;
use crate::format::path_manager::nba_data_path;
use crate::format::season::season_fmt;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_type::SeasonPeriod;

type Domain = (i32, NBAStatKind, SeasonPeriod);

pub fn write_to_data_file(domain: Domain, corrected_data: Vec<String>) -> Result<(), String> {

    let (year,kind,period) = domain;

    let data_path = nba_data_path(year, kind, period);

    let content = fs::read_to_string(&data_path)
        .map_err(|_| format!("failed to read file {:?}", data_path))?;
    
    let new_content = partition(content, corrected_data);

    match fs::write(&data_path, new_content) {
        Ok(_) => {
            println!(
                "successfully applied corrections for {} season the in the file {:?}",
                season_fmt(year),
                data_path
            );
            Ok(())
        }
        Err(e) => Err(format!("failed to write to file. {:?}:\n{}", data_path, e)),
    }
}