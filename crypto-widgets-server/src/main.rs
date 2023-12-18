mod coinmarketcap;
mod scheduler;
mod webserver;

#[tokio::main]
async fn main() {
    coinmarketcap::download_image("20396").await;

    scheduler::start().await;
    webserver::rocket().launch().await;
}
