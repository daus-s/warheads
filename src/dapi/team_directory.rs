use crate::types::{TeamAbbreviation, TeamId, TeamName};

type Assoc = (TeamId, TeamAbbreviation, TeamName);

pub struct TeamDirectory {
    teams: Vec<Assoc>,
}

impl TeamDirectory {
    pub fn new() -> Self {
        Self { teams: vec![] }
    }

    pub fn insert(&mut self, a: Assoc) {
        match self.teams.binary_search_by_key(&a.0, |a| a.0) {
            Ok(index) => {
                self.teams[index] = a;
            }
            Err(index) => {
                self.teams.insert(index, a);
            }
        }
    }
}

impl Default for TeamDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn gen_dict() {
    let games = super::super::stats::chronology::Chronology::new()
        .as_training_data()
        .expect("💀 failed to generate training data");

    let mut directory = TeamDirectory::new();

    for (card, _) in games {
        let home = card.home();
        let away = card.away();

        let a = (
            home.team_id(),
            home.team_abbr().clone(),
            home.team_name().clone(),
        );
        let b = (
            away.team_id(),
            away.team_abbr().clone(),
            away.team_name().clone(),
        );

        directory.insert(a);
        directory.insert(b);
    }
}
