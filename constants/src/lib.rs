use dotenv;

pub fn data() -> String {
    dotenv::dotenv().ok();

    dotenv::var("DATA").unwrap()
}

pub fn corrections() -> String {
    dotenv::dotenv().ok();

    dotenv::var("CORRECTIONS").unwrap()
}
