use std::error::Error;
use serde_json::{from_str, Value};

//rips through the json using the header provided as per NBA apis convention/schema.
//output the file to a (headed) csv to match the pff outputs we will be using.

pub fn nba_json_to_csv(json: &str) -> Result<String, Box< dyn Error>> {
    let v: Value = from_str(json)?;
    let result_sets = v.get("resultSets")
        .and_then(|rs| rs.as_array())
        .ok_or_else(|| "resultSets is not an array or is missing")?;

    let result_set = result_sets.get(0)
        .ok_or_else(|| "resultSets array is empty")?;

    let headers:Vec<&str> = result_set.get("headers")
        .and_then(|h| h.as_array())
        .ok_or_else(|| "Missing or invalid 'headers' field")?.iter().filter_map(|h| h.as_str()).collect();


    let row_set = result_set.get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?;

    let mut output = headers.join(",") + "\n";

    for row in row_set {
        if let Some(row_data) = row.as_array() {
            let row_csv: Vec<String> = row_data.iter()
                .map(|item| match item {
                    Value::String(s) => s.clone(),                // Extract raw string
                    Value::Number(n) => n.to_string(),            // Convert number to string
                    _ => "".to_string(),                          // Handle unexpected types
                })
                .collect();
            output.push_str(&format!("{}\n", row_csv.join(",")));
        }
    }

    Ok(output)
}