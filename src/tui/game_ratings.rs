use std::cmp::max;
use std::collections::HashMap;

use crate::format;

use crate::ml::elo;

use crate::stats::chronology::Chronology;
use crate::stats::gamecard::GameCard;

use crate::tui::tui_display::TuiDisplay;

use crate::types::{GameId, PlayerId};

pub struct GameRatings {
    game_id: GameId,
    home_ratings: HashMap<PlayerId, i64>,
    away_ratings: HashMap<PlayerId, i64>,
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

        GameRatings {
            game_id: gamecard.game_id(),
            home_ratings: home_expected_roster
                .iter()
                .map(|id| (*id, *ratings.get(id).unwrap_or(&elo::INITIAL_RATING)))
                .collect(),
            away_ratings: away_expected_roster
                .iter()
                .map(|id| (*id, *ratings.get(id).unwrap_or(&elo::INITIAL_RATING)))
                .collect(),
        }
    }
}

impl TuiDisplay for GameRatings {
    //performance :thumbsdown: but idc
    fn display(&self) -> String {
        let mut s = String::new();

        let num_rating_rows = max(self.home_ratings.len(), self.away_ratings.len());

        // let rating_width = 50;

        let mut home_ratings_vec = self.home_ratings.iter().collect::<Vec<_>>();
        let mut away_ratings_vec = self.away_ratings.iter().collect::<Vec<_>>();

        home_ratings_vec.sort_by_cached_key(|(_, r)| -1 * *r);
        away_ratings_vec.sort_by_cached_key(|(_, r)| -1 * *r);

        s.push_str(&format!("Home{}Away", format::space(44)));
        s.push('\n');
        s.push_str(&format::underline(52));
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
                        let id = format!("{}", u.0);
                        let rating = format!("{}", u.1);

                        let u0_buffer = format::space(10 - id.len());
                        let u1_buffer = format::space(6 - rating.len());

                        format!("|{}{}|{}{}", u.0, u0_buffer, u1_buffer, rating)
                    })
                    .unwrap_or(format!("|{}|{}", format::space(10), format::space(6))),
            );
            let away_rating_str = format!(
                "{}",
                away_row
                    .map(|u| {
                        let id = format!("{}", u.0);
                        let rating = format!("{}", u.1);

                        let u0_buffer = format::space(10 - id.len());
                        let u1_buffer = format::space(6 - rating.len());

                        format!("{}{}|{}{}|", rating, u1_buffer, u0_buffer, id)
                    })
                    .unwrap_or(format!("{}|{}|", format::space(6), format::space(17))),
            );

            s.push_str(&format!(
                "{}{}{}\n",
                home_rating_str,
                format::space(16),
                away_rating_str
            ));
        }

        s.push_str(&format::underline(52));
        s.push('\n');

        s.push_str(&format::space(11));

        let home_avg = self
            .home_ratings
            .iter()
            .map(|(_, rating)| *rating)
            .sum::<i64>() as f64
            / self.home_ratings.len() as f64;

        let home_average_string = format!("{}", home_avg.round() as i64);

        s.push_str(&format::space(6 - home_average_string.len()));

        s.push_str(&home_average_string);

        let away_avg = self
            .away_ratings
            .iter()
            .map(|(_, rating)| *rating)
            .sum::<i64>() as f64
            / self.away_ratings.len() as f64;

        s.push_str(&format::space(14));

        let away_average_str = format!("{}", away_avg.round() as i64);

        s.push_str(&format::space(6 - away_average_str.len()));

        s.push_str(&away_average_str);

        s.push_str(&format::space(11));

        s.push('\n');

        s
    }
}
