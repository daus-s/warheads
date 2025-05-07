#[allow(unused_imports)]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_json::Value::Null;
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;
use std::collections::HashMap;

#[allow(dead_code)] // required for serialize
pub fn serialize<S>(cs: &HashMap<StatColumn, StatValue>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map: HashMap<String, Value> = HashMap::new();
    for (key, value) in cs.clone() {
        map.insert(
            key.column_name().to_string(),
            value.val().unwrap_or_else(|| Null),
        );
    }
    map.serialize(serializer)
}

#[allow(dead_code)] // required for serialize
pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<StatColumn, StatValue>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<StatColumn, StatValue> = HashMap::deserialize(deserializer)?;

    Ok(map)
}
