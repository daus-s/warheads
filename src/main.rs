use warheads::ml::elo::elo_models::nelder_mead_elo::NelderMeadEloTracker;
use warheads::ml::elo::elo_tracker::EloTracker;
use warheads::ml::model::Model;
use warheads::proc::forecast::forecast_nba;
use warheads::proc::historian::{chronicle_nba, observe_nba, rate_nba};
use warheads::stats::chronology::Chronology;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    //initialize the data
    observe_nba().await;

    chronicle_nba();

    //load history into memory?

    //generate ratings and predictions for model
    // let mut elo = EloTracker::new();

    // rate_nba(&mut elo);

    // forecast_nba(&mut elo).await;
    //
    //
    //
    //todo: implement dispatch for models here
    let chrono = Chronology::new();

    let mut nm_elo = NelderMeadEloTracker::new();

    let training_data = chrono
        .as_training_data()
        .expect("failed to convert chronology object to training data");

    nm_elo.train(&training_data);

    nm_elo
        .serialize()
        .expect("failed to serialize nelder mead elo tracker");
}
