use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Record {
    pub wins: u8,
    pub losses: u8,
}

impl Record {
    pub fn new() -> Self {
        Record { wins: 0, losses: 0 }
    }

    pub fn wl(wins: u64, losses: u64) -> Self {
        Record {
            wins: wins as u8,
            losses: losses as u8,
        }
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let wins = parts[0].parse().map_err(|_| ())?;
        let losses = parts[1].parse().map_err(|_| ())?;
        Ok(Record { wins, losses })
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.wins, self.losses)
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}-{}", self.wins, self.losses))
    }
}

impl<'de> Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Record>()
            .map_err(|_| serde::de::Error::custom("❌ failed to parse record from string."))
    }
}
