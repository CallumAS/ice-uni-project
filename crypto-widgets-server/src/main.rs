mod coinmarketcap;
mod scheduler;
mod webserver;

#[tokio::main]
async fn main() {
    scheduler::start().await;
    webserver::rocket().launch().await;
}
