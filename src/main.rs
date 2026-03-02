use warheads::proc::dispatch::Dispatch;

#[tokio::main]
async fn main() {
    Dispatch::new().dispatch().await.expect("dispatch failed");
}
