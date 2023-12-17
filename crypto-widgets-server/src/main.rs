mod coinmarketcap;
mod scheduler;
mod webserver;

#[tokio::main]
async fn main() {
    webserver::rocket().launch().await;
    //scheduler::start().await;
}
