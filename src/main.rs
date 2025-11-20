use warheads::proc::forecast::forecast_nba;
use warheads::proc::historian::{chronicle_nba, observe_nba, rate_nba};

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    observe_nba().await;

    chronicle_nba();

    let mut elo = match rate_nba().await {
        Ok(tracker) => {
            println!("✅  loaded elo model");
            tracker
        }
        Err(e) => panic!("{e}\n💀 failed to elo model"),
    };

    forecast_nba(&mut elo).await;
}
