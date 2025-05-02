use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use format::path_manager::data_path;
use crate::id::{Identifiable, Identity};
use crate::nba_kind::NBAStatKind;
use crate::season_type::SeasonPeriod;

type Domain = (i32, NBAStatKind, SeasonPeriod);
pub fn json_to_hashmap(domain: Domain) -> Result<HashMap<Identity, String>, String> {

    let (year, kind, period) = domain;

    let path = data_path(year, kind, period);

    let content = fs::read_to_string(&path)
        .map_err(|_| format!("failed to read file {:?}", path))?;

    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JSON from file: {}", e))?;

    let set = get_result_set(&json)
        .map_err(|e| format!("failed to get result set: {}", e))?;

    let rows = rows(&set)
        .map_err(|e| format!("failed to get rows: {}", e))?;

    Ok((&rows).iter().map(|v| (v.identity(), v.to_string())).collect())

}

pub fn json_to_rows(content: &str) -> Result<Vec<String>, String> {

    let json: Value =
        serde_json::from_str(&content).map_err(|_| "failed to parse JSON from file")?;

    let set = get_result_set(&json).map_err(|e| e.to_string())?;

    let rows = rows(&set).map_err(|e| e.to_string())?;

    Ok((&rows).iter().map(|v| v.to_string()).collect())
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

pub fn rows(set: &Value) -> Result<Vec<Value>, String> {
    Ok(set
        .get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?
        .clone())
}
