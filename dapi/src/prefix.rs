pub fn prefix() -> String {
    dotenv::dotenv().ok();

    dotenv::var("PREFIX").unwrap()
}