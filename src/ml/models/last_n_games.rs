use std::collections::HashMap;
use std::fs;

use clap::Arg;

use wincode::{SchemaRead, SchemaWrite};

use crate::format::path_manager::{model_dir, records_path, results_path};

use crate::ml::log_loss::LogLossTracker;
use crate::ml::model::Model;
use crate::ml::models::registration::Registration;
use crate::ml::observation::Observation;

use crate::stats::gamecard::GameCard;

use crate::types::{GameResult, TeamId};

const LAST_N_GAMES: &str = "last-n-games";
const DEFAULT_WINDOW_SIZE: usize = 16;

/// this model uses the last `N` games to predict the outcome of a game via the
/// Bayesian method and probability function `P(A) = P(A) / (P(A) + P(B))` and
/// its complement `P(B) = P(B) / (P(A) + P(B))`.
pub struct LastNGames {
    n: usize,
    /// outcomes is a vector of game id, result pairs, the newest
    ll: LogLossTracker,
    map: HashMap<TeamId, CircularBuffer>,
}

impl LastNGames {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            map: HashMap::new(),
            ll: LogLossTracker::new(),
        }
    }

    pub fn save(&self) {
        fs::create_dir_all(model_dir(self)).ok();

        // save records
        if let Ok(record_contents) = wincode::serialize(&self.map) {
            if let Err(e) = fs::write(records_path(self), &record_contents) {
                println!("❌ {e}\n❌ failed to save records map to file for last_n_games model. you will not be able to load this model from file.")
            };
        } else if let Err(e) = wincode::serialize(&self.map) {
            println!("❌ {e}\n❌️ failed to serialize records map for last_n_games model. you will not be able to load this model from file.");
        }

        //save results
        if let Ok(result_contents) = serde_json::to_string(&self.evaluate()) {
            if let Err(e) = fs::write(results_path(self), &result_contents) {
                println!(
                    "❌ {e}\n❌ failed to save model performance to file for last_n_games model. this model cannot be evaluated from file.",
                )
            };
        } else if let Err(e) = serde_json::to_string(&self.evaluate()) {
            println!("❌ {e}\n❌️ failed to serialize results for last_n_games model. you will not be able to evaluate this model from file.");
        }
    }

    //mutable incase we need to insert a new team entry to map
    fn rolling_avg(&mut self, team_id: TeamId) -> f64 {
        let rolling_count = self
            .map
            .entry(team_id)
            .or_insert_with(|| CircularBuffer::new(self.n))
            .iter()
            .map(|x| if x == &GameResult::Win { 1.0 } else { 0.0 })
            .sum::<f64>();

        if self.map.get(&team_id).expect(&format!(
                    "💀 could not find team record in (team, record) mapping after insertion. team_id: {}",
                    team_id
                ))
                .is_empty() {
            return 0.5;
        } //avoid division by zero

        let games_measured = self.map.get(&team_id).expect(&format!(
                    "💀 could not find team record in (team, record) mapping after insertion. team_id: {}",
                    team_id
                ))
                .len() as f64;

        let rolling_avg = rolling_count / games_measured;

        // println!("{team_id}: {}-{} ({})",
        //     rolling_count,
        //     self.map.get(&team_id).expect(&format!("💀 could not find team record in (team, record) mapping after insertion. team_id: {}", team_id)).len() as u64 - rolling_count as u64,
        //     rolling_avg
        // );

        if rolling_avg != 0. {
            return rolling_avg;
        } else {
            return (rolling_count + 1f64) / (games_measured + 1f64); // if it is 0 optimistically assume that bc
                                                                     // they are in the league they can win the
                                                                     // next game
                                                                     // i.e. the "any given sunday principle"
        }
    }
}

inventory::submit!(Registration {
    model_name: LAST_N_GAMES,
    args_schema: || clap::Command::new("current team performance tracker").arg(
        Arg::new("games")
            .long("games")
            .short('g')
            .value_parser(clap::value_parser!(usize))
            .default_value("16")
    ),
    factory: |args| {
        let window_size = args
            .get_one::<usize>("games")
            .copied()
            .unwrap_or(DEFAULT_WINDOW_SIZE);

        Box::new(LastNGames::new(window_size))
    },
});

fn conditioned_probability(p_a: f64, p_h: f64) -> f64 {
    if p_a + p_h == 0.0 {
        return 0.5;
    } else {
        p_a / (p_a + p_h)
    }
}

impl Model for LastNGames {
    fn model_name(&self) -> String {
        if self.n == DEFAULT_WINDOW_SIZE {
            LAST_N_GAMES.to_owned()
        } else {
            format!("{}(days={})", LAST_N_GAMES, self.n)
        }
    }

    fn initialize(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn train(
        &mut self,
        data: &[(
            crate::stats::gamecard::GameCard,
            crate::stats::game_obj::GameObject,
        )],
    ) {
        for (_card, game) in data {
            let prob_home = self.rolling_avg(game.home_team_id());
            let prob_away = self.rolling_avg(game.away_team_id());

            let home_result = game.home().box_score().wl(); //record relative to home team
            let away_result = game.away().box_score().wl(); //record relative to away team

            let obs = Observation::new(
                if home_result == &GameResult::Win {
                    1
                } else {
                    0
                },
                conditioned_probability(prob_home, prob_away),
            );

            self.ll.add_observation(obs);

            self.map
                .entry(game.home_team_id())
                .or_insert_with(|| CircularBuffer::new(self.n))
                .insert(*home_result);
            self.map
                .entry(game.away_team_id())
                .or_insert_with(|| CircularBuffer::new(self.n))
                .insert(*away_result);
        }

        self.save();
    }

    fn evaluate(&self) -> std::collections::HashMap<String, f64> {
        if self.ll.is_empty() {
            return HashMap::new();
        }

        let map = HashMap::from([
            ("freq".to_string(), self.ll.freq()),
            ("log_loss".to_string(), self.ll.log_loss()),
            ("count".to_string(), self.ll.observations() as f64),
        ]);

        return map;
    }

    fn predict(&mut self, card: &GameCard) -> f64 {
        conditioned_probability(
            self.rolling_avg(card.home().team_id()),
            self.rolling_avg(card.away().team_id()),
        )
    }
}

#[derive(Debug, Clone, SchemaRead, SchemaWrite)]
struct CircularBuffer {
    buffer: Vec<GameResult>,
    size: usize,
}

impl CircularBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(size),
            size,
        }
    }

    pub fn insert(&mut self, result: GameResult) {
        if self.buffer.len() == self.size {
            self.buffer.remove(0);
        }
        self.buffer.push(result);
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &GameResult> {
        self.buffer.iter()
    }
}

#[cfg(test)]
mod test_last_n_games_model {
    use once_cell::sync::Lazy;

    use crate::constants::paths::data;

    use super::*;

    #[test]
    fn test_circular_buffer() {
        let mut buffer = CircularBuffer::new(3);
        buffer.insert(GameResult::Win);
        buffer.insert(GameResult::Loss);
        buffer.insert(GameResult::Draw);
        assert_eq!(buffer.len(), 3);
        assert_eq!(
            buffer.iter().copied().collect::<Vec<_>>(),
            vec![GameResult::Win, GameResult::Loss, GameResult::Draw]
        );
        buffer.insert(GameResult::Win);
        assert_eq!(buffer.len(), 3);
        assert_eq!(
            buffer.iter().copied().collect::<Vec<_>>(),
            vec![GameResult::Loss, GameResult::Draw, GameResult::Win]
        );
    }

    #[test]
    fn dont_count_empty() {
        let mut buffer = CircularBuffer::new(3);
        buffer.insert(GameResult::Win);
        buffer.insert(GameResult::Win);
        assert_eq!(buffer.len(), 2);
        assert_eq!(
            buffer.iter().copied().collect::<Vec<_>>(),
            vec![GameResult::Win, GameResult::Win]
        );
    }

    #[test]
    fn test_dir_name() {
        static DATA: Lazy<String> = Lazy::new(data);

        let model = LastNGames::new(DEFAULT_WINDOW_SIZE);
        assert_eq!(
            model_dir(&model).display().to_string(),
            format!("{}/nba/last-n-games", *DATA)
        );
        let model = LastNGames::new(10);
        assert_eq!(
            model_dir(&model).display().to_string(),
            format!("{}/nba/last-n-games(days=10)", *DATA)
        );
    }
}
