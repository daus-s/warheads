use std::fs;

use serde::Deserialize;

use crate::{
    edit::edit::Edit,
    format::path_manager,
    stats::identity::{Identifiable, Identity},
    types::{GameId, TeamAbbreviation},
};

#[derive(Debug, Clone)]
pub struct EditList {
    edits: Vec<Edit>,
}

impl EditList {
    pub fn new(edits: Vec<Edit>) -> Self {
        Self { edits }
    }

    pub fn list(&self) -> &Vec<Edit> {
        &self.edits
    }

    pub fn into_edits(self) -> Vec<Edit> {
        self.edits
    }

    pub fn insert(&mut self, edit: Edit) {
        match self.edits.binary_search(&edit) {
            Ok(i) => {
                self.edits[i].merge(edit);
            }
            Err(i) => {
                self.edits.insert(i, edit);
            }
        }
    }

    pub fn write_to_file(&self) -> Result<(), ()> {
        let file_path = path_manager::nba_edit_file();

        let json = serde_json::to_string(self.list()).map_err(|_| ())?;

        fs::write(file_path, json).map_err(|_| ())
    }

    pub fn get(&self, identity: &Identity) -> Option<Edit> {
        self.edits
            .iter()
            .find(|edit| edit.identity() == *identity)
            .cloned()
    }

    pub fn sort(&mut self) {
        self.edits.sort();
    }

    pub fn find_sibling(&self, game_id: GameId, team_abbr: &TeamAbbreviation) -> Option<Edit> {
        for edit in &self.edits {
            if edit.game_id == game_id && edit.team_abbr() != *team_abbr {
                return Some(edit.clone());
            }
        }
        None
    }

    pub(crate) fn merge(&mut self, new: EditList) {
        for edit in new.into_edits() {
            self.insert(edit);
        }
    }
}

impl<'de> Deserialize<'de> for EditList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let edits = Vec::<Edit>::deserialize(deserializer)?;
        Ok(Self { edits })
    }
}

impl Default for EditList {
    fn default() -> Self {
        Self { edits: Vec::new() }
    }
}

#[cfg(test)]
mod test_edit_list {
    use std::{collections::HashMap, str::FromStr};

    use serde_json::Value;

    use crate::edit::edit_loader::{load_edit_list, save_edit_list};
    use crate::stats::stat_column::StatColumn;

    use crate::types::{GameDate, GameId, SeasonId, TeamAbbreviation, TeamId};

    use super::*;

    #[test]
    fn test_insert() {
        let edit_v = vec![
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(1),
                team_abbr: TeamAbbreviation::from_str("LAL")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(110)),
                    (StatColumn::REB, Value::from(45)),
                ]),
            },
            Edit {
                game_id: GameId(3),
                game_date: GameDate::ymd(2022, 3, 17).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(3),
                team_abbr: TeamAbbreviation::from_str("BOS")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(120)),
                    (StatColumn::REB, Value::from(48)),
                ]),
            },
        ];

        let mut edits = EditList::new(edit_v);

        let x = Edit {
            game_id: GameId(2),
            game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
            season: SeasonId::from(22021),
            player_id: None,
            team_id: TeamId(3),
            team_abbr: TeamAbbreviation::from_str("BOS")
                .expect("Failed to create TeamAbbreviation"),
            delete: false,
            corrections: HashMap::from([
                (StatColumn::PTS, Value::from(115)),
                (StatColumn::REB, Value::from(52)),
            ]),
        };

        edits.insert(x);

        assert!(edits.edits.is_sorted())
    }

    #[test]
    fn test_edit_list_lifecycle() {
        let result = load_edit_list();

        assert!(result.is_ok());

        let edits = result.unwrap();

        assert!(edits.list().is_sorted());

        let result = save_edit_list(&edits);

        assert!(result.is_ok());
    }
}
