use crate::types::PlayerId;
use crate::types::PlayerName;

use std::collections::HashMap;

pub struct PlayerDirectory {
    map: HashMap<PlayerId, PlayerName>,
}

impl PlayerDirectory {
    pub fn load() -> Self {
        todo!();
    }

    pub fn get(&self, pid: PlayerId) -> Option<&PlayerName> {
        self.map.get(&pid)
    }

    pub fn insert(&mut self, pid: PlayerId, name: PlayerName) {
        let _ = self.map.insert(pid, name);
    }
}

impl Default for PlayerDirectory {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}
