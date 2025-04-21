use serde_json::Value;

pub fn json_to_rows(json: Value) -> Result<Vec<String>, String> {
    let set = get_result_set(&json).map_err(|s| s.to_string())?;

    let rows = rows(&set).map_err(|s| s.to_string())?;

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
