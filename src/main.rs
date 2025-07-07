use warheads::dapi::hunting::{chronicle_nba, observe_nba, revise_nba};

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    // observe_nba().await;

    revise_nba();

    chronicle_nba().await;
}
