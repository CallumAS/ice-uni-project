use std::{fs::File, io::Write};

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.coinmarketcap.com/data-api/v3/cryptocurrency/listing?start=1&limit=99999999&sortBy=rank&sortType=desc&convert=USD,BTC,ETH&cryptoType=all&tagType=all&audited=false&aux=ath,atl,high24h,low24h,num_market_pairs,cmc_rank,date_added,max_supply,circulating_supply,total_supply,volume_7d,volume_30d,self_reported_circulating_supply,self_reported_market_cap";

pub async fn download_image(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // URL of the image to download
    let mut owned_string: String =
        "https://s2.coinmarketcap.com/static/img/coins/128x128/".to_owned();
    let png: &str = ".png";

    owned_string.push_str(id);
    owned_string.push_str(png);
    // Make an HTTP GET request to the URL
    let response = reqwest::get(owned_string).await?;

    // Check if the request was successful
    if response.status() == reqwest::StatusCode::OK {
        // Create a directory (if it doesn't exist) to store the downloaded images
        let directory = "images";
        std::fs::create_dir_all(directory)?;

        // Get the filename from the URL
        let filename = format!("{}/{}.png", directory, id);

        // Create a file to save the image
        let mut file = File::create(&filename)?;

        // Read the response body and save it to the file
        let bytes = response.bytes().await?;
        file.write_all(&bytes)?;

        println!("Image downloaded to: {}", filename);
    } else {
        eprintln!(
            "Failed to download image. Status code: {}",
            response.status()
        );
    }

    Ok(())
}

pub async fn get_request() -> Result<Root, reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                USER_AGENT,
                HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36").unwrap(),
            );
            headers
        })
        .build()
        .expect("Failed to build client");

    let response = client.get(URL).send().await?;
    println!("Status: {}", response.status());

    let json = response.json::<Root>().await?;

    Ok(json)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub crypto_currency_list: Vec<CryptoCurrencyList>,
    pub total_count: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoCurrencyList {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub cmc_rank: i64,
    pub market_pair_count: i64,
    pub circulating_supply: f64,
    pub self_reported_circulating_supply: f64,
    pub total_supply: f64,
    pub max_supply: Option<f64>,
    pub ath: f64,
    pub atl: f64,
    pub high24h: f64,
    pub low24h: f64,
    pub is_active: i64,
    pub last_updated: String,
    pub date_added: String,
    pub quotes: Vec<Quote>,
    pub is_audited: bool,
    #[serde(default)]
    pub audit_info_list: Vec<AuditInfoList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub name: String,
    pub price: f64,
    pub volume24h: f64,
    pub volume7d: f64,
    pub volume30d: f64,
    pub market_cap: f64,
    pub self_reported_market_cap: f64,
    pub percent_change1h: f64,
    pub percent_change24h: f64,
    pub percent_change7d: f64,
    pub last_updated: String,
    pub percent_change30d: f64,
    pub percent_change60d: f64,
    pub percent_change90d: f64,
    pub fully_dillutted_market_cap: f64,
    pub market_cap_by_total_supply: f64,
    pub dominance: f64,
    pub turnover: Option<f64>,
    pub ytd_price_change_percentage: f64,
    pub percent_change1y: f64,
    pub tvl: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditInfoList {
    pub coin_id: String,
    pub auditor: String,
    pub audit_status: i64,
    pub audit_time: Option<String>,
    pub report_url: Option<String>,
    pub score: Option<String>,
    pub contract_address: Option<String>,
    pub contract_platform: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub timestamp: String,
    #[serde(rename = "error_code")]
    pub error_code: String,
    #[serde(rename = "error_message")]
    pub error_message: String,
    pub elapsed: String,
    #[serde(rename = "credit_count")]
    pub credit_count: i64,
}
