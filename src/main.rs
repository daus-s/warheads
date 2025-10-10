use warheads::dapi::historian::chronicle_nba;
use warheads::dapi::hunting::observe_nba;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    observe_nba().await;

    chronicle_nba().await;
}
