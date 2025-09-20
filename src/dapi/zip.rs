use std::collections::HashMap;
use serde_json::Value;
use crate::stats::stat_column::StatColumn;

/// `headers_and_rows_to_fields` consumes the inputs of `Vec<Value>` and `Vec<String>` and creates a HashMap
/// pair <StatColumn, Value>
///
/// depends on `serde_json` and `warheads::stats::StatColumn`
pub fn headers_and_values_to_fields(columns: &Vec<String>, values: &Vec<Value>) -> HashMap<StatColumn, Value> {
    columns
        .iter()
        .zip(values.iter())
        .map(|(name, value)| (StatColumn::from(name.to_owned()), value.clone()))
        .collect()
}