use crate::corrections::correction_builder::CorrectionBuilder;
use crate::dapi::box_score_builder::BoxScoreBuilder;
use crate::format::language::box_score_value_to_string;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_column::StatColumn::*;
use crate::stats::stat_value::StatValue;
use crate::types::SeasonId;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Domain = (SeasonId, NBAStatKind);
pub fn json_to_hashmap(value: &Value) -> Result<HashMap<Identity, String>, String> {
    let result_set = get_result_set(&value)?;

    let rows = get_rows(&result_set)?;

    Ok((&rows)
        .iter()
        .map(|v| (v.identity(), box_score_value_to_string(v)))
        .collect())
}

pub fn get_result_set(v: &Value) -> Result<Value, String> {
    let result_sets = v
        .get("resultSets")
        .and_then(|rs| rs.as_array())
        .ok_or_else(|| "resultSets is not an array or is missing")?;

    let result_set = result_sets
        .get(0)
        .ok_or_else(|| "resultSets array is empty")?;

    Ok(result_set.clone())
}

pub fn headers(s: &Value) -> Result<Vec<String>, String> {
    Ok(s.get("headers")
        .and_then(|h| h.as_array())
        .ok_or_else(|| "Missing or invalid 'headers' field".to_string())?
        .iter()
        .filter_map(|h| Option::from(h.to_string()))
        .collect())
}

pub fn get_rows(set: &Value) -> Result<Vec<Value>, String> {
    Ok(set
        .get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?
        .clone())
}

fn get_rows_from_file(filepath: PathBuf) -> Result<Vec<Value>, String> {
    let content =
        fs::read_to_string(&filepath).map_err(|_| format!("failed to read file {:?}", filepath))?;

    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JSON from file: {}", e))?;

    let set = get_result_set(&json).map_err(|e| format!("failed to get result set: {}", e))?;

    let rows = get_rows(&set).map_err(|e| format!("failed to get rows: {}", e))?;

    Ok(rows)
}


pub fn record_stat(entry: Result<Stat, StatColumn>, box_score: &mut impl BoxScoreBuilder, correction: &mut CorrectionBuilder) {
    match entry {
        Ok(t) => {
            box_score.add_stat(t);
        },
        Err(col) => {
            correction.add_missing_field(col, StatValue::new());
        },
    };
}

pub fn record_usable_stat(entry: Result<Stat, StatColumn>, box_score: &mut impl BoxScoreBuilder, correction: &mut CorrectionBuilder) -> Result<Stat, StatColumn>
{
    record_stat(entry.clone(), box_score, correction);

    entry
}