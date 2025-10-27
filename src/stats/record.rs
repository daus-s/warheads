use std::{fmt::Display, str::FromStr};

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
