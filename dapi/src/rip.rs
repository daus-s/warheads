//rips through the json using the header provided as per NBA apis convention/schema.
//output the file to a (headed) csv to match the pff outputs we will be using.
use serde_json::{from_str};

pub fn nba_json_to_csv(json: &str) -> String {
    let v = from_str(json);



    "".parse().unwrap()
}