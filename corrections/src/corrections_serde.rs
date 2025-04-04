use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use serde_json::Value;
use serde_json::Value::Null;
use stats::stat_column::{StatColumn};
use stats::stat_value::StatValue;

pub fn serialize<S>(corrections: &HashMap<StatColumn, StatValue>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map: HashMap<String, Value> = HashMap::new();
    for (key, value) in corrections.clone() {

        map.insert(key.to_str().to_string(), value.val().unwrap_or_else(|| Null));
    }
    map.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<StatColumn, StatValue>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<StatColumn, StatValue> = HashMap::deserialize(deserializer)?;

    Ok(map)
}