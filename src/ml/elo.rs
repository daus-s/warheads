use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{GameId, PlayerId};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Elo {
    #[serde(
        serialize_with = "serialize_player_for_csv",
        deserialize_with = "deserialize_player_from_csv"
    )]
    pub player_id: PlayerId,
    pub game_id: GameId,
    #[serde(
        serialize_with = "serialize_rating_for_csv",
        deserialize_with = "deserialize_rating_from_csv"
    )]
    pub rating: i64,
}

pub const INITIAL_RATING: i64 = 3000;

pub const K: i64 = 32;

impl Elo {
    pub fn new(player_id: PlayerId, game_id: GameId, rating: i64) -> Self {
        Elo {
            player_id,
            game_id,
            rating,
        }
    }
}

fn serialize_player_for_csv<S: Serializer>(
    pid: &PlayerId,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let formatted = format!("{:010}", pid.0);

    formatted.serialize(serializer)
}

fn deserialize_player_from_csv<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<PlayerId, D::Error> {
    let i = u64::deserialize(deserializer)?;
    Ok(PlayerId(i))
}

fn serialize_rating_for_csv<S: Serializer>(rating: &i64, serializer: S) -> Result<S::Ok, S::Error> {
    let formatted = format!("{:+06}", rating);

    formatted.serialize(serializer)
}

fn deserialize_rating_from_csv<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<i64, D::Error> {
    let s = String::deserialize(deserializer)?;
    let rating = s
        .parse()
        .map_err(|_| serde::de::Error::custom("invalid rating"))?;

    Ok(rating)
}

#[cfg(test)]
mod test_serde_elo {
    use crate::{
        ml::elo::Elo,
        types::{GameId, PlayerId},
    };

    #[test]
    fn serialize_elo() {
        // 6 digit player_id
        // standard game_id
        // default elo rating
        let elo = Elo {
            player_id: PlayerId(202020),
            game_id: GameId(0025100002),
            rating: 3000,
        };

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&elo).unwrap();

        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

        let expected = "player_id,game_id,rating\n0000202020,0025100002,+03000\n";
        assert_eq!(expected, data);
    }

    #[test]
    fn deserialize_elo() {
        let data = "player_id,game_id,rating\n0000202020,0025100002,+03000\n";
        let mut rdr = csv::Reader::from_reader(data.as_bytes());

        for result in rdr.deserialize() {
            let elo: Elo = result.unwrap();
            assert_eq!(elo.player_id, PlayerId(202020));
            assert_eq!(elo.game_id, GameId(0025100002));
            assert_eq!(elo.rating, 3000);
        }
    }
}
