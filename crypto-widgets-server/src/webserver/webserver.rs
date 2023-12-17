use rocket::fairing::AdHoc;
use rocket::tokio::sync::Barrier;
use rocket::{Build, Rocket, State};

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
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Image Proxy
#[get("/<coin>/<size>")]
fn GetImage(coin: &str, size: u8) -> String {
    format!("👋 Hello, {} year old named {}!", coin, size)
}

#[get("/")]
async fn ListCoins() -> Option<RawText<File>> {}

#[get("/?<opt..>", format = "text/event-stream", rank = 1)]
fn getCoinsSSE(opt: Options<'_>) -> EventStream![] {
    let stream = EventStream! {
        let mut interval = time::interval(Duration::from_secs(1));

        while true {
            interval.tick().await;
            let list = COINS.lock().await;
            let data: HashMap<String, CoinData> = list.clone();

            let coins = AllCoins{
                coins: data,
            };
            yield Event::json(&coins).event("message");
        }
    };

    stream.heartbeat(Duration::from_secs(3))
}

pub fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, upload, delete, retrieve])
}
