mod coinmarketcap;
mod scheduler;

#[tokio::main]
async fn main() {
    scheduler::Start().await;
}
