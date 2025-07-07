//! TODO: Replace with proper environment handling later
use dotenv;

pub fn data() -> String {
    dotenv::dotenv().ok();

    match dotenv::var("DATA") {
        Ok(s) => s,
        Err(_) => panic!("💀 could not read DATA from environment. (.env)"),
    }
}