use crate::scheduler;
use rocket::fairing::{AdHoc, Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::response::stream::Event;
use rocket::response::stream::EventStream;
use rocket::serde::json::Json;
use rocket::tokio::sync::Barrier;
use rocket::tokio::time::{self, Duration};
use rocket::{get, Build, Rocket, State};
use rocket::{routes, Request};
use rocket::{FromForm, Response};
use std::sync::Mutex;
use std::vec;
use std::{collections::HashMap, fs::File, sync::Arc};

pub struct CORS;

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
        response.set_header(Header::new("Access-Control-Allow-Credentials", "false"));
    }
}

#[get("/?<opt..>")]
async fn list_coins(opt: Options<'_>) -> Json<HashMap<String, scheduler::CoinData>> {
    let mut new_array_symbols: Vec<String> = Vec::new();

    if let Some(symbols) = opt.symbols {
        new_array_symbols = symbols.split(',').map(String::from).collect();
        println!("Received symbols: {}", symbols);
    } else {
        println!("No symbols parameter provided");
    }

    let list = scheduler::COINS.lock().await;
    let data: HashMap<String, scheduler::CoinData> = list.clone();

    let filtered_list: HashMap<String, scheduler::CoinData> = if new_array_symbols.is_empty() {
        data
    } else {
        data.into_iter()
            .filter(|(key, _)| new_array_symbols.contains(&key))
            .collect()
    };

    Json(filtered_list)
}
#[derive(FromForm)]
struct Options<'r> {
    symbols: Option<&'r str>,
}
#[get("/sse?<opt..>", format = "text/event-stream", rank = 1)]
fn get_coins_sse(opt: Options<'_>) -> EventStream![] {
    let mut new_array_symbols: Vec<String> = Vec::new();

    if let Some(symbols) = opt.symbols {
        new_array_symbols = symbols.split(',').map(String::from).collect();
        println!("Received symbols: {}", symbols);
    } else {
        println!("No symbols parameter provided");
    }

    let stream = EventStream! {
        let mut interval = time::interval(Duration::from_secs(10));

        loop {
            let coin_data = scheduler::COINS.lock().await.clone();

            let filtered_list: HashMap<String, scheduler::CoinData> = if new_array_symbols.is_empty() {
                coin_data
            } else {
                coin_data.into_iter()
                    .filter(|(key, _)| new_array_symbols.contains(&key))
                    .collect()
            };
            yield Event::json(&filtered_list).event("message");

            interval.tick().await;
        }
    };

    stream.heartbeat(Duration::from_secs(3))
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CORS) //middleware
        .mount("/public", FileServer::from("./images"))
        .mount("/", routes![list_coins, get_coins_sse]) //get all coins or specific
}
