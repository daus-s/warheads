//! TODO: Replace with proper environment handling later
use dotenv;

pub fn data() -> String {
    dotenv::dotenv().ok();

    dotenv::var("DATA").unwrap()
}
