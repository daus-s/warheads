use warheads::proc::dispatch::Dispatch;

#[tokio::main]
async fn main() {
    match Dispatch::new().dispatch().await {
        Ok(_) => {
            println!("successfully completed command.\ngoodbye!")
        }
        Err(e) => {
            eprintln!("{}\n failed to run command", e);
        }
    }
}
