use crate::stats::game_obj::GameObject;
use aws_sdk_s3 as s3;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

/// saves a game object to the file `history/{gameid}` if home and
/// `history/{gameid}a` if away as json per 0.1.0
/// returns a result measuring the success of the operation
///
/// params:
///     client is the s3 client with which to save it to the database
///     game - should absolutely have its player scores already initialized
///
///
pub async fn save_nba_game(client: Client, game: GameObject) -> Result<(), s3::Error> {
    let game_id = game.game_id();

    let filename = game_id.to_string();

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
