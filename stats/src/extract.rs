use serde_json::Value;
use std::error::Error;

pub fn nba_json_to_str(json: Value) -> Result<Vec<String>, Box<dyn Error>> {
    let set = get_set(&json)?;

    let rows = rows(&set)?;

    Ok((&rows).iter().map(|v| v.to_string()).collect())
}

pub fn get_set(v: &Value) -> Result<Value, &'static str> {
    let result_sets = v
        .get("resultSets")
        .and_then(|rs| rs.as_array())
        .ok_or_else(|| "resultSets is not an array or is missing")?;

    let result_set = result_sets
        .get(0)
        .ok_or_else(|| "resultSets array is empty")?;

    Ok(result_set.clone())
}

pub fn headers(s: &Value) -> Result<Vec<&str>, &'static str> {
    Ok(s.get("headers")
        .and_then(|h| h.as_array())
        .ok_or_else(|| "Missing or invalid 'headers' field")?
        .iter()
        .filter_map(|h| h.as_str())
        .collect())
}

pub fn rows(set: &Value) -> Result<Vec<Value>, &'static str> {
    Ok(set
        .get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?
        .clone())
}
