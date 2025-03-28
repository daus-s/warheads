use dapi::hunting::save_nba_season;
use stats;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name


    save_nba_season(1959).await;

}