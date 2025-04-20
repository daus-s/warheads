use aws_sdk_s3 as s3;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use stats::team_box_score::TeamBoxScore;
use stats::visiting::Visiting;

/// saves a game object to the file history/{gameid} if home and
/// history/{gameid}a if away as json per 0.1.0
/// returns a result measuring the success of the operation
///
/// params:
///     client is the s3 client with which to save it to the database
///     game - should absolutely have its player scores already initialized
///
///
pub async fn save_nba_game(client: Client, game: TeamBoxScore) -> Result<(), s3::Error> {
    let game_id = game.game_id();

    let away = game.home_or_away();

    let filename = game_id.to_string()
        + match away {
            Visiting::Home => "",
            Visiting::Away => "a",
        };

    let content = match serde_json::to_string(&game) {
        Ok(str) => str.as_bytes().to_vec(),
        Err(e) => panic!("{:#?}", e), //todo: improve this error handling
    };

    let byte_stream = ByteStream::from(content);

    client
        .put_object()
        .bucket("nba")
        .key(format!("history/{}", filename))
        .body(byte_stream)
        .send()
        .await
        .map_err(s3::Error::from)
        .expect(format!("Failed to save game {} at games/{}", game_id, filename).as_str());

    Ok(())
}

async fn list_player_games(pid: u32) -> Vec<u64> {
    todo!("this will return a list of all the games in which a player played")
    /*
        due to the nature of the data this function does not need to be optimized, but it really should be.
        between most games, players are not traded so you can kinda assume that they'll be on the team the next game.
    */
}
