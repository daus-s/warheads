use std::collections::HashMap;
use std::collections::HashSet;

use crate::stats::game_obj::GameObject;
use crate::types::PlayerId;
use crate::types::TeamId;

pub(crate) struct Appearances {
    appearances: HashMap<TeamId, HashMap<PlayerId, u64>>,
    associations: HashMap<PlayerId, TeamId>,
}

impl Appearances {
    pub fn new() -> Self {
        Self {
            appearances: HashMap::new(),
            associations: HashMap::new(),
        }
    }

    pub fn get_expected_roster(&self, team: TeamId) -> Vec<PlayerId> {
        let roster = self.appearances.get(&team);

        if roster.is_none() {
            return vec![];
        }

        roster
            .unwrap()
            .into_iter()
            .filter(|&(_player, apps)| if *apps > 1u64 { true } else { false })
            .map(|(player, _apps)| *player)
            .collect()
    }

    pub fn record_appearances(&mut self, game: &GameObject) {
        let home = game.home().team_id();
        let away = game.away().team_id();

        let home_roster = game.home().roster();
        let away_roster = game.away().roster();

        let mut home_dnp: HashSet<PlayerId> = self
            .appearances
            .entry(home)
            .or_insert(HashMap::new())
            .iter()
            .map(|(&player, &_apps)| player)
            .collect();
        let mut away_dnp: HashSet<PlayerId> = self
            .appearances
            .entry(away)
            .or_insert(HashMap::new())
            .iter()
            .map(|(&player, &_apps)| player)
            .collect();

        home_roster.iter().for_each(|&player| {
            home_dnp.remove(&player);

            let roster = self
                .appearances
                .entry(home)
                .or_insert(HashMap::<PlayerId, u64>::from([(player, 0)]));

            roster
                .entry(player)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            if let Some(&team) = self.associations.get(&player) {
                if team != home {
                    self.record_transfer(player, home, team);
                }
            } else {
                self.associations.insert(player, home);
            }
        });

        away_roster.iter().for_each(|&player| {
            away_dnp.remove(&player);

            let roster = self
                .appearances
                .entry(away)
                .or_insert(HashMap::<PlayerId, u64>::from([(player, 0)]));

            roster
                .entry(player)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            if let Some(&team) = self.associations.get(&player) {
                if team != away {
                    self.record_transfer(player, away, team);
                }
            } else {
                self.associations.insert(player, away);
            }
        });

        home_dnp.iter().for_each(|&player| {
            self.appearances
                .get_mut(&home)
                .unwrap()
                .entry(player)
                .and_modify(|u| *u /= 2);
        });

        away_dnp.iter().for_each(|&player| {
            self.appearances
                .get_mut(&away)
                .unwrap()
                .entry(player)
                .and_modify(|u| *u /= 2);
        });
    }

    fn record_transfer(&mut self, player: PlayerId, from: TeamId, to: TeamId) {
        let prev_team = self
            .appearances
            .get_mut(&from)
            .expect("tried to record a transfer from an unknown team");

        let apps = prev_team.remove(&player).unwrap_or(1);

        let new_team = self
            .appearances
            .get_mut(&to)
            .expect("tried to transfer a player to an unknown team");

        new_team.insert(player, apps);
    }
}
