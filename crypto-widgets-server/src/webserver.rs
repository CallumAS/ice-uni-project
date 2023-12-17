use crate::scheduler;
use rocket::fairing::{AdHoc, Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::stream::Event;
use rocket::response::stream::EventStream;
use rocket::tokio::sync::Barrier;
use rocket::tokio::time::{self, Duration};
use rocket::{get, Build, Rocket, State};
use rocket::{routes, Request};
use rocket::{FromForm, Response};
use std::{collections::HashMap, fs::File, sync::Arc};
pub struct CORS;

#[derive(FromForm)]
struct Options<'r> {
    name: Option<&'r str>,
}
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Image Proxy
#[get("/<coin>/<size>")]
fn GetImage(coin: &str, size: u8) -> String {
    format!("👋 Hello, {} year old named {}!", coin, size)
}

#[get("/")]
async fn ListCoins() -> String {
    return "asd".to_string();
}

#[get("/?<opt..>", format = "text/event-stream", rank = 1)]
fn getCoinsSSE(opt: Options<'_>) -> EventStream![] {
    let stream = EventStream! {
        let mut interval = time::interval(Duration::from_secs(1));

        while true {
            interval.tick().await;
            let list = scheduler::COINS.lock().await;
            let data: HashMap<String, scheduler::CoinData> = list.clone();

            yield Event::json(&data).event("message");
        }
    };

    stream.heartbeat(Duration::from_secs(3))
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CORS) //middleware
        .mount("/", routes![ListCoins]) //get all coins or specific
        .mount("/image", routes![GetImage]) //get coin image
        .mount("/sse", routes![getCoinsSSE]) //get SSE event of all coins
}
