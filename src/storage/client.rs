use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use dotenv;

async fn create_s3_client(endpoint: &str) -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(endpoint) // Use your local endpoint
        .load()
        .await;

    Client::new(&config)
}

pub async fn create() -> Client {
    dotenv::dotenv().ok();

    let url = dotenv::var("S3_URL").expect("failed to get s3 url from env\n");

    create_s3_client(&url.as_str()).await
}
