use serde::{Deserialize, Serialize};
use std::io;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct PercentGeneric(f32); // s.t. f32 >= 0.0 && f32  <= 100.0

impl FromStr for PercentGeneric {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(f) => {
                if f >= 0. && f <= 1. {
                    Ok(PercentGeneric(f))
                } else {
                    Err("❌ percent is not in the correct bounds [0, 100]."
                        .parse()
                        .unwrap())
                }
            }
            Err(_) => Err("❌ couldn't parse a percent from string.".parse().unwrap()),
        }
    }
}

pub fn percent_string(num: i32, den: i32) -> String {
    if den == 0 {
        return "-".to_string();
    }

    format!("({:.1}%)", (num as f32 * 100.0) / den as f32)
}

pub struct PercentageFormatter;

impl serde_json::ser::Formatter for PercentageFormatter {
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        write!(writer, "{:.6}", value)
    }
}
