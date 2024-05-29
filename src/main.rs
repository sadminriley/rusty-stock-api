extern crate reqwest;
extern crate tokio;
extern crate dotenv;

use reqwest::header::{HeaderMap, UPGRADE_INSECURE_REQUESTS};
use dotenv::dotenv;
use std::env;
use tokio::main;

async fn get_stock_quote(symbol: &str) -> Result<String, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let url = format!(
        "https://financialmodelingprep.com/api/v3/quote-short/{}?apikey={}",
        symbol, api_key
    );

    let res = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

#[main]
async fn main() -> Result<(), reqwest::Error> {
    let symbol: &str = "TSLA"; // Pass any stock symbol
    match get_stock_quote(symbol).await {
        Ok(quote) => println!("Stock quote for {}: {}", symbol, quote),
        Err(e) => eprintln!("Error fetching stock quote: {}", e),
    }

    Ok(())
}
