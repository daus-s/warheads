use warheads::ml::elo_tracker::EloTracker;
use warheads::proc::forecast::forecast_nba;
use warheads::proc::historian::{chronicle_nba, observe_nba, rate_nba};

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    //initialize the data
    observe_nba().await;

    chronicle_nba();

    //load history into memory?

    //generate ratings and predictions for model
    let mut elo = EloTracker::new();

    rate_nba(&mut elo);

    forecast_nba(&mut elo).await;
}
