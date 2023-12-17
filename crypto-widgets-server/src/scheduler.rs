use crate::coinmarketcap::{self, Get_request, Root};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
#[derive(Serialize, Deserialize, Clone)]

pub struct CoinData {
    pub symbol: String,
    pub price: f64,
    pub daily_gains: f64,
    pub coin_type: u8,
    pub marketcap: f64,
}

//CURRENT UPTO DATE DATA
lazy_static! {
    static ref COINS: Arc<Mutex<HashMap<String, CoinData>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn Start() {
    // Clone the Arc for the background task

    tokio::spawn(async move {
        loop {
            // Simulate adding items to the list
            {
                match Get_request().await {
                    Ok(data) => Format(&data),
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }

            // Simulate some work
            tokio::time::sleep(Duration::from_secs(120)).await;
        }
    });
    tokio::time::sleep(Duration::from_secs(140)).await;
}

pub fn Format(data: &Root) {
    println!("got data")
}
