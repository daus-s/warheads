use warheads::dapi::hunting::{chronicle_nba, observe_nba};

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    observe_nba().await;

    chronicle_nba().await;
}
