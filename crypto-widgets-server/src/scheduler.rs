use crate::coinmarketcap::{self, get_request, Root};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, fs::File, sync::Arc, time::Duration};
use tokio::sync::Mutex as TokioMutex;
use tokio::task;
#[derive(Serialize, Deserialize, Clone)]
pub struct CoinData {
    pub id: i64,
    pub cmc: i64,
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub daily_gains: f64,
    pub marketcap: f64,
}

pub async fn start() {
    tokio::spawn(async move {
        loop {
            // adding items to the list
            {
                match get_request().await {
                    Ok(data) => format(&data).await,
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
            dump_coins_as_json("./coins-dump.json").await;

            // wait
            tokio::time::sleep(Duration::from_secs(120)).await;
        }
    });
}

//CURRENT UPTO DATE DATA

lazy_static! {
    pub static ref COINS: Arc<TokioMutex<HashMap<String, CoinData>>> =
        Arc::new(TokioMutex::new(HashMap::new()));
}

pub async fn format(data: &Root) {
    let thread_shared_list = COINS.clone();
    let mut coins = thread_shared_list.lock().await;
    for ele in &data.data.crypto_currency_list {
        for quotes in &ele.quotes {
            if quotes.name == "USD" {
                let id_str = ele.id.to_string();
                if fs::metadata(Path::new("./images/").join(format!("{}.png", id_str))).is_ok()
                    == false
                {
                    task::spawn(async move {
                        coinmarketcap::download_image(&id_str).await;
                    });
                }
                let coin_data = CoinData {
                    id: ele.id,
                    cmc: ele.cmc_rank,
                    name: ele.name.clone(),
                    symbol: ele.symbol.clone(),
                    price: quotes.price,
                    daily_gains: quotes.percent_change24h,
                    marketcap: quotes.market_cap,
                };

                coins.insert(ele.name.clone(), coin_data);
                break;
            }
        }
    }
}

async fn dump_coins_as_json(filename: &str) {
    println!("saving data");

    let thread_shared_list = COINS.clone();
    let coins = thread_shared_list.lock().await;

    // Serialize the coins hashmap to JSON
    let json_data = serde_json::to_string_pretty(&*coins).unwrap();

    // Write JSON data to a file
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");
    println!("saved data");
}
