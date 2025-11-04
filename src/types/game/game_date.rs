use chrono::{Datelike, NaiveDate};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

/// `GameDate`is a `chrono::NaiveDate` wrapper that implements the necessary traits to work
/// interchangeably in the code base.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct GameDate(pub NaiveDate);

impl GameDate {
    pub fn today() -> Self {
        let naive_date = chrono::Utc::now().date_naive();

        GameDate(naive_date)
    }

    /// destructure GameDate object into (y, m, d) tuple. year is an integer,
    /// years before the big JC was born are negative
    ///
    /// returns month and day as unsigned. both are 1-indexed. (month [1..12]) (day [1..31])
    pub fn destructure(&self) -> (i32, u32, u32) {
        let (year, month, day) = (
            self.0.year_ce().1 as i32 * (if self.0.year_ce().0 { 1i32 } else { -1i32 }),
            self.0.month0() + 1,
            self.0.day0() + 1,
        );

        (year, month, day)
    }


    pub fn next(&self) -> Self {
        let naive_date = self.0.succ_opt().unwrap();

        GameDate(naive_date)
    }
}

impl From<NaiveDate> for GameDate {
    fn from(date: NaiveDate) -> Self {
        GameDate(date)
    }
}

impl FromStr for GameDate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())?;

        Ok(GameDate(date))
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// SERIALIZATION & DESERIALIZATION /////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////
impl Serialize for GameDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format!(
            "{}-{:02}-{:02}",
            self.0.year(),
            self.0.month(),
            self.0.day()
        );

        serializer.serialize_str(&format)
    }
}

impl<'de> Deserialize<'de> for GameDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let date = NaiveDate::parse_from_str(&*s, "%Y-%m-%d").map_err(de::Error::custom)?;

        Ok(GameDate(date))
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// PARSING /////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////
impl From<&str> for GameDate {
    fn from(s: &str) -> Self {
        if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            GameDate(date)
        } else if let Ok(date) = NaiveDate::parse_from_str(s, "%m/%d/%Y") {
            GameDate(date)
        } else if let Ok(date) = NaiveDate::parse_from_str(s, "%Y_%m_%d") {
            GameDate(date)
        } else {
            GameDate(NaiveDate::MIN)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// FORMATTERS //////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////
impl Debug for GameDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted_date = self.0.format("%Y-%m-%d").to_string();

        write!(f, "{}", formatted_date)
    }
}

impl Display for GameDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted_date = self.0.format("%m/%d/%Y").to_string();

        write!(f, "{}", formatted_date)
    }
}
