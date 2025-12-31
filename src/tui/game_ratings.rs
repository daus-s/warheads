use std::cmp::max;
use std::collections::HashMap;

use crate::format;

use crate::ml::cdf;
use crate::stats::chronology::Chronology;
use crate::stats::gamecard::GameCard;

use crate::tui::tui_display::TuiDisplay;

use crate::types::{PlayerId, PlayerName};

#[derive(Debug, Clone, PartialEq)]
pub struct GameRatings {
    game_card: GameCard,
    home_ratings: HashMap<PlayerId, (PlayerName, i64)>,
    away_ratings: HashMap<PlayerId, (PlayerName, i64)>,
}

impl GameRatings {
    pub fn new(
        gamecard: &GameCard,
        chronology: &Chronology,
        ratings: &HashMap<PlayerId, i64>,
    ) -> Self {
        let home_expected_roster =
            chronology.get_expected_roster(gamecard.home().team_id(), gamecard.game_id());
        let away_expected_roster =
            chronology.get_expected_roster(gamecard.away().team_id(), gamecard.game_id());

        let player_directory = chronology.player_directory();

        GameRatings {
            game_card: gamecard.clone(),
            home_ratings: home_expected_roster
                .iter()
                .map(|id| {
                    (
                        *id,
                        (
                            player_directory.get(*id).unwrap().clone(),
                            *ratings.get(id).unwrap(),
                        ),
                    )
                }) //maybe panic on this?
                .collect(),
            away_ratings: away_expected_roster
                .iter()
                .map(|id| {
                    (
                        *id,
                        (
                            player_directory.get(*id).unwrap().clone(),
                            *ratings.get(id).unwrap(),
                        ),
                    )
                })
                .collect(),
        }
    }

    pub fn home_roster(&self) -> Option<Vec<PlayerId>> {
        Some(self.home_ratings.keys().cloned().collect())
    }

    pub fn away_roster(&self) -> Option<Vec<PlayerId>> {
        Some(self.away_ratings.keys().cloned().collect())
    }
}

impl TuiDisplay for GameRatings {
    //performance :thumbsdown: but idc
    fn display(&self) -> String {
        let mut s = String::new();

        let num_rating_rows = max(self.home_ratings.len(), self.away_ratings.len());

        let mut home_ratings_vec = self.home_ratings.iter().collect::<Vec<_>>();
        let mut away_ratings_vec = self.away_ratings.iter().collect::<Vec<_>>();

        home_ratings_vec.sort_by_cached_key(|(_, (_, r))| -1 * *r);
        away_ratings_vec.sort_by_cached_key(|(_, (_, r))| -1 * *r);

        s.push_str(&format::underline(80));
        s.push_str("\n| ");
        s.push_str(&self.game_card.away().team_abbr().emphasize());
        s.push_str(" @ ");
        s.push_str(&self.game_card.home().team_abbr().emphasize());
        s.push_str(&format!(" {:>10}", self.game_card.date()));
        s.push_str(&format::space(56));
        s.push_str(&format!(" |\n| Away{}Home |", format::space(68)));
        s.push('\n');
        s.push_str(&format::underline(80));
        s.push('\n');

        for i in 0..num_rating_rows {
            let home_row = match 2 * home_ratings_vec.len() - num_rating_rows {
                x if i < x => {
                    let home_rating = home_ratings_vec[i];

                    Some(home_rating)
                }
                _ => None,
            };
            let away_row = match 2 * away_ratings_vec.len() - num_rating_rows {
                x if i < x => {
                    let away_rating = away_ratings_vec[i];

                    Some(away_rating)
                }
                _ => None,
            };

            let home_rating_str = format!(
                "{}",
                home_row
                    .map(|u| {
                        let id = format!("{}", u.1 .0);
                        let rating = format!("{}", u.1 .1);

                        let id_buffer = format::space(24 - id.chars().count());
                        let rating_buffer = format::space(6 - rating.len());

                        format!("{}{}|{}{}|", rating, rating_buffer, id_buffer, id)
                    })
                    .unwrap_or(format!("{}|{}|", format::space(6), format::space(24))),
            );
            let away_rating_str = format!(
                "{}",
                away_row
                    .map(|u| {
                        let id = format!("{}", u.1 .0);
                        let rating = format!("{}", u.1 .1);

                        let id_buffer = format::space(24 - id.chars().count());
                        let rating_buffer = format::space(6 - rating.len());

                        format!("|{}{}|{}{}", id, id_buffer, rating_buffer, rating)
                    })
                    .unwrap_or(format!("|{}|{}", format::space(24), format::space(6))),
            );

            s.push_str(&format!(
                "{}{}{}\n",
                away_rating_str,
                format::space(16),
                home_rating_str,
            ));
        }

        s.push_str(&format::underline(80));
        s.push('\n');

        let home_avg = self
            .home_ratings
            .iter()
            .map(|(_, (_, rating))| *rating)
            .sum::<i64>() as f64
            / self.home_ratings.len() as f64;
        let away_avg = self
            .away_ratings
            .iter()
            .map(|(_, (_, rating))| *rating)
            .sum::<i64>() as f64
            / self.away_ratings.len() as f64;

        let p_home = cdf::prob(home_avg - away_avg, 400.0);
        let p_away = 1f64 - p_home;

        let home_prob_string = format!("({:.1}%)", p_home * 100.0);

        let home_rating_string = format!("{}", home_avg.round() as i64,);

        let away_prob_string = format!("({:.1}%)", p_away * 100.0);

        let away_rating_string = format!("{}", away_avg.round() as i64,);

        s.push('|');
        s.push_str(&format::space(25));

        s.push_str(&away_rating_string);
        s.push_str(&format::space(6 - away_rating_string.len()));

        s.push_str(&format::space(16));

        s.push_str(&format::space(6 - home_rating_string.len()));
        s.push_str(&home_rating_string);

        s.push_str(&format::space(25));
        s.push('|');
        s.push('\n');

        s.push('|');
        s.push_str(&format::space(25));

        s.push_str(&away_prob_string);
        s.push_str(&format::space(7 - away_prob_string.len()));

        s.push_str(&format::space(14));

        s.push_str(&format::space(7 - home_prob_string.len()));
        s.push_str(&home_prob_string);

        s.push_str(&format::space(25));
        s.push('|');

        s.push('\n');

        s.push_str(&format::underline(80));

        s
    }
}
