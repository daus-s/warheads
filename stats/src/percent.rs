use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Percent (f32); // s.t. f32 >= 0.0 && f32  <= 100.0

impl FromStr for Percent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(f) => if f >= 0. && f <= 100. {
                Ok(Percent(f))
            } else {
                Err("percent is not in the correct bounds [0, 100].".parse().unwrap())
            }
            Err(_) => {
                Err("couldn't parse a percent from string.".parse().unwrap())
            }
        }
    }
}