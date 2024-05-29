
extern crate dotenv;
extern crate reqwest;
extern crate tokio;

use reqwest::header::{HeaderMap, UPGRADE_INSECURE_REQUESTS};
use tokio::main;
use dotenv::dotenv;
use std::env;


#[main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let url = format!("https://financialmodelingprep.com/api/v3/quote-short/AAPL?apikey={}", api_key);

    let res = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}
