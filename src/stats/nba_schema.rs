/// # Schema
/// this enum defines the type of data provided as different eras of nba have yielded different recorded
/// schemas. (e.g. early NBA stats only included points, pfs and makes but not attempts.)
pub(super) enum NBASchema {
    EarlyNBA(),      //46-?
    Pre3PtEra(),     //79-
    PlayByPlayEra(), //96-present
}

impl From<i64> for NBASchema {
    fn from(year: i64) -> Self {
        assert!(
            1946 < year && year < 2046,
            "💀 cannot assign stat schema to years not included in NBA history got: {year}"
        );

        match year {
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        dapi::season_manager::{nba_lifespan, nba_lifespan_period},
        stats::{chronology::Chronology, season_period::minimum_spanning_era},
    };

    #[test]
    fn calculate_schema_from_year() {
        let mut chrono = Chronology::new();
        for era in nba_lifespan_period() {
            chrono.load_era(era).expect("failed to load chronology");
            let games = chrono.games().as_ref().unwrap();
            let mut schema_map = HashMap::<u32, u32>::new();
            for game in games {
                let boxscore = game.home().box_score();
                let k: u32 = 0
                        | (1 << 0)  // min (always present)
                        | (1 << 1)  // fgm (always present)
                        | (if boxscore.fga().0.is_some() { 1 } else { 0 } << 2)
                        | (if boxscore.fg3m().0.is_some() { 1 } else { 0 } << 3)
                        | (if boxscore.fg3a().0.is_some() { 1 } else { 0 } << 4)
                        | (1 << 5)  // ftm (always present)
                        | (if boxscore.fta().0.is_some() { 1 } else { 0 } << 6)
                        | (if boxscore.oreb().0.is_some() { 1 } else { 0 } << 7)
                        | (if boxscore.dreb().0.is_some() { 1 } else { 0 } << 8)
                        | (if boxscore.reb().0.is_some() { 1 } else { 0 } << 9)
                        | (if boxscore.ast().0.is_some() { 1 } else { 0 } << 10)
                        | (if boxscore.stl().0.is_some() { 1 } else { 0 } << 11)
                        | (if boxscore.blk().0.is_some() { 1 } else { 0 } << 12)
                        | (if boxscore.tov().0.is_some() { 1 } else { 0 } << 13)
                        | (1 << 14)  // pf (always present)
                        | (1 << 15)  // pts (always present)
                        | (if boxscore.plus_minus().0.is_some() { 1 } else { 0 } << 16);
                schema_map.entry(k).and_modify(|v| *v += 1).or_insert(1);
            }

            let columns = vec![
                "min", "fgm", "fga", "fg3m", "fg3a", "ftm", "fta", "oreb", "dreb", "reb", "ast",
                "stl", "blk", "tov", "pf", "pts", "+-",
            ];

            let col_width = 5;
            println!(
                "{}",
                columns
                    .iter()
                    .map(|col| format!("{:^width$}", col, width = col_width))
                    .collect::<Vec<_>>()
                    .join("|")
            );

            println!(
                "{}",
                (0..columns.len())
                    .map(|_| "-".repeat(col_width))
                    .collect::<Vec<_>>()
                    .join("+")
            );

            for (schema, count) in schema_map {
                println!(
                    "{}",
                    columns
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let marker = if (schema >> i) & 1 == 1 { "✓" } else { "✗" };
                            format!("{:^width$}", marker, width = col_width)
                        })
                        .collect::<Vec<_>>()
                        .join("|")
                );
            }
        }
    }
}
